use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress(bar: u64) -> ProgressBar {
    let bar = ProgressBar::new(bar);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .expect("ProgressBar Error")
            .tick_chars(format!("{}", "⣾⣽⣻⢿⡿⣟⣯⣷").as_str())
            .progress_chars("#>-"),
    );
    bar
}
