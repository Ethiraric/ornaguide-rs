use indicatif::{ProgressBar, ProgressStyle};

pub fn bar(len: u64) -> ProgressBar {
    let bar = ProgressBar::new(len);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg:15!} {eta:>3} [{wide_bar}] {pos:>4}/{len:4}")
            .progress_chars("=> "),
    );
    bar
}

pub fn sanitize_guide_name(name: &str) -> &str {
    if let Some(pos) = name.find('[') {
        name.split_at(pos - 1).0
    } else {
        name
    }
}
