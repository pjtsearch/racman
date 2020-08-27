use indicatif::{ProgressBar,ProgressStyle};
use std::sync::Mutex;
use std::collections::HashMap;
use alpm::Progress;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROGRESS_BARS:Mutex<HashMap<Progress,ProgressBar>> = Mutex::new(HashMap::new());
}

pub fn progresscb(progress: Progress, pkgname: &str, percent: i32, _howmany: usize, _current: usize) {
    let message = match progress {
        Progress::AddStart=>"Adding",
        Progress::UpgradeStart=>"Upgrading",
        Progress::DowngradeStart=>"Downgrading",
        Progress::ReinstallStart=>"Reinstalling",
        Progress::RemoveStart=>"Removing",
        Progress::ConflictsStart=>"Checking conflicts",
        Progress::DiskspaceStart=>"Checking disk space",
        Progress::IntegrityStart=>"Checking integrity",
        Progress::LoadStart=>"Loading",
        Progress::KeyringStart=>"Getting keyring"
    };
    if let Ok(mut progress_bars) = PROGRESS_BARS.lock() {
        if progress_bars.get(&progress).is_none() && percent < 100 {
            let bar = ProgressBar::new(100);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{prefix} [{wide_bar}] {pos}/{len} ")
                    .progress_chars("=> "),
            );
            bar.set_prefix(&format!("{} {}",message, pkgname));
            progress_bars.insert(progress,bar);
        }

        if let Some(progress_bar) = progress_bars.get_mut(&progress) {
            progress_bar.set_position(percent as u64);
            if percent == 100 {
                progress_bar.finish();
                progress_bars.remove(&progress);
            }
        }
    }
}