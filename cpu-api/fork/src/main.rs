#![allow(dead_code)]
use std::{ffi::CString, io::Write, os::fd::RawFd};

use libc::{O_CREAT, O_TRUNC, O_WRONLY};

extern crate libc;

fn q1() {
    let mut x = 2;
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
        } else if pid == 0 {
            println!("New child process with x {}", x);
            x = 200;
            println!("The value of x in child {}", x);
        } else {
            println!("Continuing execution in main process");
            x = 300;
            println!("The value of x in parent {}", x);
        }
    }
}

fn q2() {
    unsafe {
        let c_path = CString::new("./test_file").expect("CString::new failed");
        let f = libc::open(c_path.as_ptr(), O_CREAT | O_WRONLY | O_TRUNC);
        let buf = CString::new("Hello World\n").expect("failed to create CString");
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
        } else if pid == 0 {
            println!("new child with f {}", f);
            libc::write(f, buf.as_ptr() as *const libc::c_void, buf.count_bytes());
        } else {
            println!("continuing main with f {}", f);
            libc::write(f, buf.as_ptr() as *const libc::c_void, buf.count_bytes());
        }
        libc::close(f);
    }
}

fn q3() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
        } else if pid == 0 {
            println!("New child process -> hello");
        } else {
            let mut status: i32 = 0;
            let pid_t = libc::wait(&mut status as *mut i32);
            println!("child finished with {}", pid_t);
            println!("Continuing execution in main process -> goodbye");
        }
    }
}

fn q3_unix_pipe() {
    let mut pipe_fds: [RawFd; 2] = [0; 2];
    unsafe {
        if libc::pipe(pipe_fds.as_mut_ptr()) == -1 {
            eprintln!("error creating pipes");
            return;
        }
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
        } else if pid == 0 {
            libc::close(pipe_fds[0]);
            println!("new child process -> hello");
            let message = CString::new("done").expect("failed to create CString");
            libc::write(
                pipe_fds[1],
                message.as_ptr() as *const libc::c_void,
                message.count_bytes(),
            );
        } else {
            libc::close(pipe_fds[1]);
            let buffer = [0u8; 4];
            libc::read(
                pipe_fds[0],
                buffer.as_ptr() as *mut libc::c_void,
                buffer.len(),
            );
            println!("goodbye");
            libc::close(pipe_fds[0]);
        }
    }
}

fn q4() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("failed to fork");
        } else if pid == 0 {
            let path = CString::new("/bin/ls").unwrap();
            let arg0 = CString::new("").unwrap();
            let result = libc::execl(path.as_ptr(), arg0.as_ptr());
            println!("result of exec {}", result);
        } else {
            let status = 0;
            let pid_t = libc::wait(status as *mut i32);
            println!("child process -> {}", pid_t);
        }
    }
}

fn q5() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
        } else if pid == 0 {
            println!("New child process -> hello");
            let mut status: i32 = 0;
            let pid_t = libc::wait(&mut status as *mut i32);
            println!("child of child finished with {}", pid_t);
        } else {
            println!("Continuing execution in main process -> goodbye");
        }
    }
}

fn q6() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
        } else if pid == 0 {
            println!("New child process -> hello");
        } else {
            let status: i32 = 0;
            let options: i32 = 0;
            let pid_t = libc::waitpid(pid as i32, status as *mut i32, options as i32);
            println!("child finished with {}", pid_t);
            println!("Continuing execution in main process -> goodbye");
        }
    }
}

fn q7() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Fork failed");
        } else if pid == 0 {
            println!("New child process -> hello");
            std::io::stdout().flush().unwrap();
            libc::close(1);
            println!("attempting to write from child after closing stdout");
        } else {
            let status: i32 = 0;
            let options: i32 = 0;
            let pid_t = libc::waitpid(pid as i32, status as *mut i32, options as i32);
            println!("child finished with {}", pid_t);
            println!("Continuing execution in main process -> goodbye");
        }
    }
}

fn q8() {
    unsafe {
        let mut pipes: [RawFd; 2] = [0; 2];
        libc::pipe(pipes.as_mut_ptr());

        let child_1 = libc::fork();
        if child_1 == 0 {
            libc::close(pipes[0]);
            libc::dup2(pipes[1], 1);
            libc::close(pipes[1]);

            let cmd = CString::new("/bin/echo").unwrap();
            let arg1 = CString::new("Hello from child_1").unwrap();
            libc::execl(
                cmd.as_ptr(),
                cmd.as_ptr(),
                arg1.as_ptr(),
                std::ptr::null::<i8>(),
            );
            eprintln!("execl failed for child_1");
            libc::exit(1);
        }

        let child_2 = libc::fork();
        if child_2 == 0 {
            // In child_2 process
            libc::close(pipes[1]); // Close the write end of the pipe
            libc::dup2(pipes[0], 0); // Duplicate the read end of the pipe to stdin
            libc::close(pipes[0]); // Close the original read end of the pipe

            // Execute a command that reads from stdin
            let cmd = CString::new("/usr/bin/wc").unwrap();
            libc::execl(cmd.as_ptr(), cmd.as_ptr(), std::ptr::null::<i8>());
            eprintln!("execl failed for child_2");
            libc::exit(1);
        }

        libc::close(pipes[0]); // Close the read end of the pipe
        libc::close(pipes[1]); // Close the write end of the pipe

        // Wait for both children to finish
        let mut status: i32 = 0;
        libc::waitpid(child_1, &mut status as *mut i32, 0);
        libc::waitpid(child_2, &mut status as *mut i32, 0);
    }
}

fn main() {
    q8();
}
