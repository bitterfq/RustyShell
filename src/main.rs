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
use std::process::{Command, Child, Stdio}; 

// Type to support path operations
// ref: https://doc.rust-lang.org/std/path/struct.Path.html
use std::path::Path; 

// Module for manipulation of process's enviorments
use std::env; 

const CMDLINE_MAX: usize = 512; 



fn main() {
   
    loop {
        
        let mut input = String::new(); 
        let _n1: Vec<c_char> ; 
        let mut _ret: c_int ; 

        /* Print prompt */
        print!("rusty$ ");
        let _ = stdout().flush().unwrap(); 

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

        if input == "exit" {
            println!("exiting...");
            println!("+ completed 'exit' [0]");
            break;
        }

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None; 

        while let Some(command) = commands.next() {
            
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap(); 
            let args = parts;
             /* Builtin Command */

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
            
            match command {
                "cd" => {
                    // default to '/' as new directory if one was not provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}",e);
                    }
                    previous_command = None; 
                },
                
               

                command => {

                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    
                    println!("+ completed '{}' [0]", command);

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }    
    }
}

