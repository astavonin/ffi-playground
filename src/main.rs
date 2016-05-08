#![feature(link_args)]

extern crate libc;
use std::{slice, ptr};

type DataType = libc::c_int;
type SizeType = libc::c_int;
type ResultType = libc::c_int;

#[link(name="cpp2rs")]
extern {
    fn foo_alloc(buff: *mut *const DataType, size: *mut SizeType) -> ResultType;
    fn foo_free(buff: *const DataType);
}

struct Foo {
    data:   *const DataType,
    len:    SizeType
}

impl Foo {
    fn new() -> Foo {
        let mut foo = Foo {
            data: ptr::null_mut(),
            len: 0
        };
        unsafe { foo_alloc(&mut foo.data, &mut foo.len) };
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

fn main() {
    let foo = Foo::new();
    for (i, item) in foo.as_slice().iter().enumerate() {
        println!("{}:\t{}", i, item);
    }
}

