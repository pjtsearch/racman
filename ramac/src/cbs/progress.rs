use std::sync::Mutex;
use std::collections::HashMap;
use alpm::Progress;
use lazy_static::lazy_static;
use progressing::ClampingBar;
use progressing::Bar;

lazy_static! {
    static ref PROGRESS_BARS:Mutex<HashMap<Progress,ClampingBar>> = Mutex::new(HashMap::new());
}

pub fn progresscb(progress: Progress, _pkgname: &str, percent: i32, _howmany: usize, _current: usize) {
    if PROGRESS_BARS.lock().unwrap().get(&progress).is_none(){
        let mut bar = ClampingBar::new();
        bar.set_bar_len(100);
        PROGRESS_BARS.lock().unwrap().insert(progress,bar);
    }

    let mut progress_bars = PROGRESS_BARS.lock().unwrap();
    if let Some(progress_bar) = progress_bars.get_mut(&progress){
        progress_bar.set(percent as f32/100 as f32).reprintln().expect("could not show progress bar");
        if percent == 100 {
            progress_bars.remove(&progress);
        }
    }
}