use crate::{free, Alpm, Conflict, Db, DepMissing, Depend, FileConflict, Group, Package};

use std::ffi::{c_void, CStr};
use std::iter::Iterator;
use std::marker::PhantomData;
use std::os::raw::c_char;

use alpm_sys::*;

pub unsafe trait AsAlpmListItem<'a> {
    fn as_alpm_list_item(handle: &'a Alpm, ptr: *mut c_void) -> Self;
}

impl<'a, T> Iterator for AlpmList<'a, T> where T: AsAlpmListItem<'a>  {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let data = self.next_data();

        match data {
            Some(data) => Some(T::as_alpm_list_item(self.handle, data)),
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
            let size = unsafe { alpm_list_count(self.current) };
            (size, Some(size))
    }
}

impl<'a, T> AlpmList<'a, T> {
    fn next_data(&mut self) -> Option<*mut c_void> {
        if self.current.is_null() {
            None
        } else {
            let data = unsafe { (*(self.current)).data };
            self.current = unsafe { alpm_list_next(self.current) };
            Some(data)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum FreeMethod {
    FreeList,
    FreeInner,
    FreeConflict,
    FreeFileConflict,
    FreeDepMissing,
    None,
}

#[derive(Debug)]
pub struct AlpmList<'a, T> {
    pub(crate) handle: &'a Alpm,
    pub(crate) list: *mut alpm_list_t,
    pub(crate) current: *mut alpm_list_t,
    pub(crate) free: FreeMethod,
    pub(crate) _marker: PhantomData<T>,
}

impl<'a, T> AlpmList<'a, T> {
    pub(crate) fn new(
        handle: &'a Alpm,
        list: *mut alpm_list_t,
        free: FreeMethod,
    ) -> AlpmList<'a, T> {
        AlpmList {
            handle,
            list,
            current: list,
            free,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> AlpmList<'a, T> {
        AlpmList {
            handle: self.handle,
            list: self.list,
            current: self.current,
            free: FreeMethod::None,
            _marker: self._marker,
        }
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for Package<'a> {
    fn as_alpm_list_item(handle: &'a Alpm, ptr: *mut c_void) -> Self {
        Package {
            pkg: ptr as *mut alpm_pkg_t,
            handle,
            drop: false,
        }
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for Group<'a> {
    fn as_alpm_list_item(handle: &'a Alpm, ptr: *mut c_void) -> Self {
        Group {
            inner: ptr as *mut alpm_group_t,
            handle,
        }
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for Depend<'a> {
    fn as_alpm_list_item(_handle: &'a Alpm, ptr: *mut c_void) -> Self {
        Depend {
            inner: ptr as *mut alpm_depend_t,
            drop: false,
            phantom: PhantomData,
        }
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for FileConflict {
    fn as_alpm_list_item(_handle: &'a Alpm, ptr: *mut c_void) -> Self {
        FileConflict {
            inner: ptr as *mut alpm_fileconflict_t,
        }
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for DepMissing {
    fn as_alpm_list_item(_handle: &'a Alpm, ptr: *mut c_void) -> Self {
        DepMissing {
            inner: ptr as *mut alpm_depmissing_t,
        }
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for Conflict {
    fn as_alpm_list_item(_handle: &'a Alpm, ptr: *mut c_void) -> Self {
        Conflict {
            inner: ptr as *mut alpm_conflict_t,
            drop: false,
        }
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for Db<'a> {
    fn as_alpm_list_item(handle: &'a Alpm, ptr: *mut c_void) -> Self {
        Db {
            db: ptr as *mut alpm_db_t,
            handle,
        }
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for &'a str {
    fn as_alpm_list_item(_handle: &'a Alpm, ptr: *mut c_void) -> Self {
        let s = unsafe { CStr::from_ptr(ptr as *mut c_char) };
        s.to_str().unwrap()
    }
}

unsafe impl<'a> AsAlpmListItem<'a> for String {
    fn as_alpm_list_item(_handle: &'a Alpm, ptr: *mut c_void) -> Self {
        let s = unsafe { CStr::from_ptr(ptr as *mut c_char) };
        s.to_str().unwrap().to_string()
    }
}

unsafe extern "C" fn fileconflict_free(ptr: *mut c_void) {
    alpm_fileconflict_free(ptr as *mut alpm_fileconflict_t);
}

unsafe extern "C" fn depmissing_free(ptr: *mut c_void) {
    alpm_depmissing_free(ptr as *mut alpm_depmissing_t);
}

unsafe extern "C" fn conflict_free(ptr: *mut c_void) {
    alpm_conflict_free(ptr as *mut alpm_conflict_t);
}

impl<'a, T> Drop for AlpmList<'a, T> {
    fn drop(&mut self) {
        match self.free {
            FreeMethod::None => {}
            FreeMethod::FreeList => {
                unsafe { alpm_list_free(self.list) };
            }
            FreeMethod::FreeInner => {
                unsafe { alpm_list_free_inner(self.list, Some(free)) };
                unsafe { alpm_list_free(self.list) };
            }
            FreeMethod::FreeConflict => {
                unsafe { alpm_list_free_inner(self.list, Some(conflict_free)) };
                unsafe { alpm_list_free(self.current) };
            }
            FreeMethod::FreeFileConflict => {
                unsafe { alpm_list_free_inner(self.list, Some(fileconflict_free)) };
                unsafe { alpm_list_free(self.current) };
            }
            FreeMethod::FreeDepMissing => {
                unsafe { alpm_list_free_inner(self.list, Some(depmissing_free)) };
                unsafe { alpm_list_free(self.current) };
            }
        }
    }
}
