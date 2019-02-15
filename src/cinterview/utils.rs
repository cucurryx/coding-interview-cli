use indicatif::{ProgressBar, ProgressStyle};

/// Return a pre-configured progress bar
pub fn get_progress_bar(size: u64, msg: &str) -> ProgressBar {
    let style = ProgressStyle::default_bar().progress_chars("#>-").template(
        "{spinner:.green} {msg} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ",
    );
    let bar = ProgressBar::new(size);
    bar.set_style(style);
    bar.set_message(msg);
    bar
}
