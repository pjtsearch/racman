use alpm::Progress;

pub fn progresscb(progress: Progress, pkgname: &str, percent: i32, howmany: usize, current: usize) {
    println!(
        "{:?} progress on {}: {}% [{}/{}]",
        progress, pkgname, percent, current, howmany
    );
}