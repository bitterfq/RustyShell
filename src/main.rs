// libc provides all of the definitions necessary to easily interoperate with C code (or "C-like" code) on each of the platforms that Rust supports.
// documentation : https://docs.rs/libc/0.2.90/libc/#types
// ref : https://doc.rust-lang.org/nomicon/ffi.html

extern crate libc;
use libc::{c_int, c_char};

// for flushing 
// ref: https://users.rust-lang.org/t/flush-the-standard-output-on-terminal/1018/2

use std::io::Write; 
use std::io::stdout; 
use std::io::stdin;

// for accepting cmdline args
// ref: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
use std::env; 

const CMDLINE_MAX: i32 = 512; 

/*
    A struct that holds (in order):
    
    1. The inital command
    2. The arguments of the command
    3. The ending null terminator

*/

struct ParsedInput {

    cd_active: c_int,
    total_args: c_int, 
    arg_error: c_int, 
    cd: c_int, 
    c: Vec<String>

}

fn main() {
    
    // returns itr of cmdline args
    // ref: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let mut cmd: *mut i8; 

    loop {

        let n1: Vec<c_char> ; 
        let mut ret: c_int ; 

        /* Print prompt */
        println!("rusty$ ");
        stdout().flush(); 

        /* Get command line */
        libc::fgets(cmd, CMDLINE_MAX, );
        
    }
}

