use alpm::LogLevel;

pub fn logcb(level: LogLevel, msg: &str) {
    if level == LogLevel::ERROR {
        print!("log {}", msg);
    }
}