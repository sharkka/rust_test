

/**
 * @name   newland.rs
 * @brief  a small piece of cake
 * @author ANYZ
 * @date   2022-10-18 14:24:36
 */

pub mod ohio {
use std::io::Write;
pub fn file_write() {
    use std::fs::File;
    println!("write into file");
    let filepath = String::from("foo.txt");
    let fp = File::create(&filepath);
    //let mut text = String::new();
    let text = String::from(
        "I've seen things you people wouldn't believe.
Attack ships on fire off the shoulder of Orion.
I've watched c-beams glitter in the dark near the Tannhauser Gate.
All those ... moments will be lost in time, like tears...in rain.
Time to die ......"
    );
    let mut fc = match fp {
        Ok(fc) => fc,
        Err(err) => panic!("error, {}", err),
    };
    fc.write(text.as_bytes()).unwrap();
}


use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

pub fn wserver() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


}

pub mod rstd {
use std::alloc;
//use std::alloc::Global;
use std::any;
use std::arch;
use std::array;
use std::ascii;
use std::borrow;
use std::boxed;
use std::cell;
use std::char;
use std::clone;
use std::cmp;
use std::collections;
use std::convert;
use std::default;
use std::env;
use std::error;
use std::f32;
use std::f64;
use std::ffi;
use std::fmt;
use std::fs;
use std::future;
use std::hash;
use std::hint;

use std::io;
use std::isize;
use std::iter;
use std::marker;
use std::mem;
use std::net;
use std::num;
use std::ops;
use std::option;
use std::os;
use std::panic;
use std::path;
use std::pin;
use std::prelude;
use std::primitive;
use std::process;
use std::ptr;
use std::rc;
use std::result;
use std::slice;
use std::str;
use std::string;
use std::sync;
use std::task;
use std::thread;
use std::time;

//
use std::assert;
use std::assert_eq;
use std::assert_ne;
use std::cfg;
use std::column;
use std::compile_error;
use std::concat;
use std::dbg;
use std::debug_assert;
use std::debug_assert_eq;
use std::debug_assert_ne;
//use std::env;
use std::eprint;
use std::eprintln;
use std::file;
use std::format;
use std::format_args;
use std::include;
use std::include_bytes;
use std::include_str;
use std::is_x86_feature_detected;
use std::line;
use std::matches;
use std::module_path;
use std::option_env;
//use std::panic;
use std::print;
use std::println;
use std::stringify;
use std::thread_local;
use std::todo;
use std::unimplemented;
use std::unreachable;
use std::vec;
use std::write;
use std::writeln;


#[cfg(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    )
)]
fn newland_arch() {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::_mm256_add_epi64;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::_mm256_add_epi64;

    unsafe {
        _mm256_add_epi64(..);
    }
}

use std::alloc::{ GlobalAlloc, System, Layout };
struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

fn newland_alloc() {
    let mut v = Vec::new();
    v.push(1);
}


pub fn newland_all() {
    newland_alloc();
}


















}
