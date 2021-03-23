// libc provides all of the definitions necessary to easily interoperate with C code (or "C-like" code) on each of the platforms that Rust supports.
// documentation : https://docs.rs/libc/0.2.90/libc/#types
// ref : https://doc.rust-lang.org/nomicon/ffi.html

extern crate libc;
use libc::{c_int, c_char};

// for flushing 
// ref: https://users.rust-lang.org/t/flush-the-standard-output-on-terminal/1018/2

use std::io::{stdin, stdout, Write}; 

// crate to return current directory as PathBuf
extern crate nix; 
use nix::unistd;

//A module for working with process
// ref: https://doc.rust-lang.org/std/process/index.html
use std::process::Command; 

const CMDLINE_MAX: usize = 512; 

fn sys(command:&str) {
   
    let mut child = Command::new(command)
    .spawn()
    .unwrap();

    child.wait().expect("command didn't run");
}

fn main() {
   
    loop {
        
        let mut input = String::new(); 
        let _n1: Vec<c_char> ; 
        let mut _ret: c_int ; 

        /* Print prompt */
        print!("rusty$ ");
        let _ = stdout().flush(); 

        /* Get command line 
            ref: https://users.rust-lang.org/t/how-to-get-user-input/5176/2
        */
        stdin().read_line(&mut input).unwrap();

        if input.len() > CMDLINE_MAX {
            println!("Error: Command line max reached"); 
            continue; 
        }
        if let Some('\n')=input.chars().next_back() {
            input.pop();
        }
        if let Some('\r')=input.chars().next_back() {
            input.pop();
        }

        let command = input.trim(); 
       

        /* Builtin Command */

        /* Exit implementation*/
        if input == "exit" || input == "Exit" || input == "EXIT"{
            println!("exiting...");
            println!("+ completed 'exit' [0]");
            break;
        } 
        /* pwd implementation*/
        if input == "pwd" {
            let dir = unistd::getcwd().unwrap(); 
            println!("{:?}", dir); 
            println!("+ completed 'pwd' [0]");
            continue;
        }

        /* clear implementation
            ref:https://rosettacode.org/wiki/Terminal_control/Clear_the_screen#Rust
        */
        if input == "clear" {
            print!("\x1B[2J");
            println!("+ completed 'clear' [0]");
        }
        

        sys(command);
        println!("+ completed '{}' [0]", command);

        
    }
}

