use image::{ImageBuffer, Pixel, Rgb};
use indicatif::{ProgressBar, ProgressStyle};

pub type Image = ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>;

pub fn initialize_progress_bar(size: u64) -> Result<ProgressBar,
    anyhow::Error> {
    let barrita = ProgressBar::new(size);

    barrita.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise} ({duration} estimado)] \
            {msg} [{wide_bar:.cyan/blue}] \
            [{human_pos}/{human_len} tiles] {percent}%",
            )?
            .progress_chars("#>-"),
    );

    Ok(barrita)
}
