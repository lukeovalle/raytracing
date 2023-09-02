use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub fn parallel_for<F, T>(threads: usize, iter: Vec<T>, f: F)
    where F: Fn(T) + Clone + Send + Sync + 'static,
          T: Copy + Send + Sync + 'static
{
    // usar un solo hilo si son pocas iteraciones o hay un solo hilo configurado
    if threads == 1 || iter.len() <= 30 { // Definir en una macro o algo ese mínimo
        for i in iter {
            f(i);
        }
        return;
    }

    // crear un threadpol
    let threadpool = ThreadPool::new(threads);

//    let f = MyFn::new(f);

    for i in iter {
        let f = f.clone();
        threadpool.execute(move || {
            f(i);
        });
    }
}

type Job = Box<dyn Fn() + Send + Sync + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct MyThread {
    thread: Option<thread::JoinHandle<()>>,
}

impl MyThread {
    fn new(consumer: Arc<Mutex<mpsc::Receiver<Message>>>) -> MyThread {
        let thread = thread::spawn(move || {
            loop {
                let msg = consumer.lock().unwrap().recv().unwrap();

                match msg {
                    Message::NewJob(job) => {
                        job();
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });

        MyThread {
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<MyThread>,
    producer: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let size = if size < 1 { 1 } else { size };

        let (producer, consumer) = mpsc::channel();

        let consumer = Arc::new(Mutex::new(consumer));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(MyThread::new(Arc::clone(&consumer)));
        }

        ThreadPool {
            workers,
            producer,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: Fn() + Send + Sync + 'static
    {
        let job = Box::new(f);

        self.producer.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.producer.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

