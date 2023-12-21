use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;

pub struct Task<'a>(Box<dyn FnOnce() + Send + 'a>);

impl<'a> Task<'a> {
    fn call(self) {
        self.0()
    }
}

pub struct ThreadPool<'a> {
    task_queue: Mutex<VecDeque<Task<'a>>>,
}

impl<'a> ThreadPool<'a> {
    pub fn new() -> Self {
        Self {
            task_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn add_task<T>(&mut self, task: T)
    where T: FnOnce() + Send + 'a,
    {
        self.task_queue
            .get_mut()
            .unwrap()
            .push_back(Task(Box::new(task)));
    }

    /// Ejecuta las tareas de la cola en paralelo,
    /// la función f es para hacer algo en el hilo principal, por ejemplo
    /// mostrar la barra de progreso.
    pub fn run<F>(
        &mut self,
        f: F)
    where F: FnOnce()
    {
        let threads_count = num_cpus::get();
        println!("Using {} threads.", threads_count);

        thread::scope(|s| {
            let _handlers: Vec<_> = (0..threads_count).map(|_| {
                s.spawn(|| loop {
                    let task = {
                        self.task_queue.lock().unwrap().pop_front()
                    };

                    match task {
                        Some(task) => task.call(),
                        None => break,
                    }
                })
            }).collect();

            f();
        });
    }
}
