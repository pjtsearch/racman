use indicatif::ProgressBar;
use std::sync::Mutex;
use std::collections::HashMap;
use alpm::Progress;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROGRESS_BARS:Mutex<HashMap<Progress,ProgressBar>> = Mutex::new(HashMap::new());
}

pub fn progresscb(progress: Progress, _pkgname: &str, percent: i32, _howmany: usize, _current: usize) {
    if PROGRESS_BARS.lock().unwrap().get(&progress).is_none(){
        if percent < 100{
            let bar = ProgressBar::new(100);
            PROGRESS_BARS.lock().unwrap().insert(progress,bar);
        }
    }

    let mut progress_bars = PROGRESS_BARS.lock().unwrap();
    if let Some(progress_bar) = progress_bars.get_mut(&progress){
        progress_bar.set_position(percent as u64);
        if percent == 100 {
            progress_bar.finish();
            progress_bars.remove(&progress);
        }
    }
}