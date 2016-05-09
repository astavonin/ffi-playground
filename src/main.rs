#![feature(link_args)]

extern crate libc;
use std::{slice, ptr};
use std::os::raw;
use std::ffi::CStr;

type DataType = libc::c_int;
type SizeType = libc::c_int;
type ResultType = libc::c_int;

#[link(name="cpp2rs")]
extern {
    fn foo_alloc(key: ResultType, buff: *mut *const DataType, size: *mut SizeType) -> ResultType;
    fn foo_free(buff: *const DataType);
    fn foo_get_info(key: ResultType) -> *const raw::c_char;
}

struct Foo {
    data:   *const DataType,
    len:    SizeType,
    err:    ResultType
}

impl Foo {
    fn new() -> Foo {
        let mut foo = Foo {
            data: ptr::null_mut(),
            len: 0,
            err: 0
        };
        foo.err = unsafe { foo_alloc(0, &mut foo.data, &mut foo.len) };
        return foo;
    }

    fn as_slice(&self) -> &[DataType] {
        return unsafe { slice::from_raw_parts(self.data, self.len as usize) };
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        if self.data != ptr::null_mut() {
            unsafe { foo_free(self.data) }
        }
    }
}

fn allocate_foo(key: ResultType) -> Result<&'static [DataType], ResultType> {
    let mut data: *const DataType = ptr::null_mut();
    let mut len: SizeType = 0;
    match unsafe { foo_alloc(key, &mut data, &mut len) } {
        0 => {
            Ok(unsafe { slice::from_raw_parts(data, len as usize) })
        },
        err @ _ => {
            Err(err)
        }
    }
}

fn free_foo(arr: &[DataType]) {
    unsafe {
        foo_free(arr.as_ptr());
    }
}

fn get_info(key: ResultType) -> &'static str {
    unsafe {
        let raw_info = CStr::from_ptr(foo_get_info(key));
        match raw_info.to_str() {
            Ok(s) => s,
            Err(_) => "Unable to load data"
        }
    }
}

fn main() {
    let foo = Foo::new();
    for (i, item) in foo.as_slice().iter().enumerate() {
        println!("{}:\t{}", i, item);
    }

    match allocate_foo(0) {
        Ok(foo_arr) => {
            for (i, item) in foo_arr.iter().enumerate() {
                println!("{}:\t{}", i, item);
            }
            free_foo(foo_arr);
        },
        Err(err_code) => {
            println!("Error {}", err_code);
        }
    }

    println!("Info: {}", get_info(1));
}

