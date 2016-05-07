#![feature(link_args)]

extern crate libc;

type DataType = libc::c_int;
type SizeType = libc::c_int;
type ResultType = libc::c_int;

#[link(name="cpp2rs")]
extern {
    fn foo_alloc(buff: *mut *mut DataType, size: *mut SizeType) -> ResultType;
    fn foo_free(buff: *mut *mut DataType, size: SizeType);
}

fn foo_alloc_ex() -> Option<Vec<DataType>> {

    let mut buff: *mut DataType = std::ptr::null_mut();
    let mut len: SizeType = 0;
    let ret = unsafe { foo_alloc(&mut buff, &mut len) };

    if ret == 0 {
        println!("Buffer len {}", len);
    } else {
        println!("Failed with code {}", ret);
    }
    return None;
}

fn main() {
    foo_alloc_ex();
    println!("Hello, world!");
}
