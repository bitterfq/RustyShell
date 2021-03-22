// libc provides all of the definitions necessary to easily interoperate with C code (or "C-like" code) on each of the platforms that Rust supports.
// documentation : https://docs.rs/libc/0.2.90/libc/#types
// ref : https://doc.rust-lang.org/nomicon/ffi.html

extern crate libc;
use libc::{c_int, c_char};

// for flushing 
// ref: https://users.rust-lang.org/t/flush-the-standard-output-on-terminal/1018/2

use std::io::{stdin, stdout, Write}; 

// for accepting cmdline args
// ref: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
use std::env; 

// crate to return current directory as PathBuf
extern crate nix; 
use nix::unistd;

const CMDLINE_MAX: usize = 512; 

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
        
        let mut s = String::new(); 
        let n1: Vec<c_char> ; 
        let mut ret: c_int ; 

        /* Print prompt */
        print!("rusty$ ");
        let _ = stdout().flush(); 

        /* Get command line 
            ref: https://users.rust-lang.org/t/how-to-get-user-input/5176/2
        */
        stdin().read_line(&mut s).expect("Command not recognized");
        if s.len() > CMDLINE_MAX {
            println!("Error: Command line max reached"); 
            continue; 
        }
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        /* Builtin Command */

        /* Exit implementation*/
        if s == "exit" || s == "Exit" || s == "EXIT"{
            println!("exiting...");
            println!("+ completed 'exit' [0]");
            break;
        } 
        /* pwd implementation*/
        if s == "pwd" {
            let dir = unistd::getcwd().unwrap(); 
            println!("{:?}", dir); 
            println!("+ completed 'pwd' [0]");
            continue;
        }

        /* clear implementation
            ref:https://rosettacode.org/wiki/Terminal_control/Clear_the_screen#Rust
        */
        if s == "clear" {
            print!("\x1B[2J");
            println!("+ completed 'clear' [0]");
        }
    }
}

