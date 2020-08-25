use alpm::Progress;
use alpm::Question;
use alpm::LogLevel;
use alpm::Event;
use crate::Racman;

pub trait CBs {
    fn set_eventcb(&self,cb:fn(event: &Event)->());
    fn set_logcb(&self,cb:fn(level: LogLevel, msg: &str)->());
    fn set_questioncb(&self,cb:fn(question: &Question)->());
    fn set_progresscb(&self,cb:fn(progress: Progress, pkgname: &str, percent: i32, howmany: usize, current: usize)->());
}
impl CBs for Racman {
    fn set_eventcb(&self,cb:fn(event: &Event)->()){
        use std::ptr;
        use alpm::alpm_sys::*;

        static mut C_ALPM_HANDLE: *mut alpm_handle_t = ptr::null_mut();
        unsafe {
            C_ALPM_HANDLE = self.alpm.as_alpm_handle_t();
        }
        fn blank(_event: &Event){}
        static mut CB: fn(event: &Event) = blank;
        unsafe {
            CB = cb;
        }

        unsafe extern "C" fn c_eventcb(event: *mut alpm_event_t) {
            let event = Event::new(C_ALPM_HANDLE, event);
            CB(&event);
        }

        unsafe { alpm_option_set_eventcb(self.alpm.as_alpm_handle_t(), Some(c_eventcb)) };
    }
    fn set_logcb(&self,cb:fn(level: LogLevel, msg: &str)->()){
        use std::ffi::{c_void, CStr};
        use std::os::raw::{c_char, c_int};
        use std::ptr;
        use alpm::alpm_sys::*;

        extern "C" {
            fn vasprintf(
                str: *const *mut c_char,
                fmt: *const c_char,
                args: *mut __va_list_tag,
            ) -> c_int;
            fn free(ptr: *mut c_void);
        }

        fn blank(_level: LogLevel, _msg: &str){}
        static mut CB: fn(level: LogLevel, msg: &str) = blank;
        unsafe {
            CB = cb;
        }

        unsafe extern "C" fn c_logcb(
            level: alpm_loglevel_t,
            fmt: *const c_char,
            args: *mut __va_list_tag,
        ) {
            let buff = ptr::null_mut();
            let n = vasprintf(&buff, fmt, args);
            if n != -1 {
                let s = CStr::from_ptr(buff);
                let level = LogLevel::from_bits(level).unwrap();
                CB(level, &s.to_string_lossy());
                free(buff as *mut c_void);
            }
        }

        unsafe { alpm_option_set_logcb(self.alpm.as_alpm_handle_t(), Some(c_logcb)) };
    }
    fn set_questioncb(&self,cb:fn(question: &Question)->()){
        use std::ptr;
        use alpm::alpm_sys::*;

        static mut C_ALPM_HANDLE: *mut alpm_handle_t = ptr::null_mut();
        unsafe {
            C_ALPM_HANDLE = self.alpm.as_alpm_handle_t();
        }

        fn blank(_question: &Question){}
        static mut CB: fn(question: &Question) = blank;
        unsafe {
            CB = cb;
        }

        unsafe extern "C" fn c_questioncb(question: *mut alpm_question_t) {
            let mut question = Question::new(C_ALPM_HANDLE, question);
            CB(&mut question);
        }

        unsafe { alpm_option_set_questioncb(self.alpm.as_alpm_handle_t(), Some(c_questioncb)) };
    }
    fn set_progresscb(&self,cb:fn(progress: Progress, pkgname: &str, percent: i32, howmany: usize, current: usize)->()){
        use std::ffi::CStr;
        use std::mem::transmute;
        use std::os::raw::{c_char, c_int};
        use alpm::alpm_sys::*;

        fn blank(_progress: Progress, _pkgname: &str, _percent: i32, _howmany: usize, _current: usize){}
        static mut CB: fn(progress: Progress, pkgname: &str, percent: i32, howmany: usize, current: usize) = blank;
        unsafe {
            CB = cb;
        }

        unsafe extern "C" fn c_progresscb(
            progress: alpm_progress_t,
            pkgname: *const c_char,
            percent: c_int,
            howmany: usize,
            current: usize,
        ) {
            let pkgname = CStr::from_ptr(pkgname);
            let pkgname = pkgname.to_str().unwrap();
            let progress = transmute::<alpm_progress_t, Progress>(progress);
            CB(progress, &pkgname, percent as i32, howmany, current);
        }

        unsafe { alpm_option_set_progresscb(self.alpm.as_alpm_handle_t(), Some(c_progresscb)) };
    }
}