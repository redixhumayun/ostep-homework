
# Overview

There are now two simulators related to this chapter. The first,
`fork.py`, is a simple tool to show what a process tree looks like
when processes are created and destroyed. Read more about it
[here](README-fork.md).

The second is a program, `generator.py`, that creates real C programs
that use `fork()`, `wait()`, and `exit()` to show how `fork` works in
running programs. Read more about it [here](README-generator.md).


## Questions

1. Write a program that calls fork(). Before calling fork(), have the main process access a variable (e.g., x) and set its value to something (e.g., 100). What value is the variable in the child process? What happens to the variable when both the child and parent change the value of x?

The value in the child process will be the same because fork does COW semantics which means both parent and child processes share the same memory pages where `x` is stored.
When both child & parent change it, they see independent copies because when the value is modified, a new copy is created only within the process that made the change. Hence, Copy-On-Write.

2.  Write a program that opens a file (with the open() system call) and then calls fork() to create a new process. Can both the child and parent access the file descriptor returned by open()? What happens when they are writing to the file concurrently, i.e., at the same time?

Yes, both can access the same fd because of COW.
Technically, there is no sync mechanism to protect so the writes should be interleaved but I don't see that in practise.
Apparently `O_APPEND` would make the writes atomic by placing the cursor at the end of the file before the write begins.

3. Write another program using fork(). The child process should print “hello”; the parent process should print “goodbye”. You should try to ensure that the child process always prints first; can you do this without calling wait() in the parent?

Possible to do this using Unix pipes as a signaling mechanism.

4. Write a program that calls fork() and then calls some form of exec() to run the program /bin/ls. See if you can try all of the variants of exec(), including (on Linux) execl(), execle(), execlp(), execv(), execvp(), and execvpe(). Why do you think there are so many variants of the same basic call?

Each of them have different behaviour in terms of searching for a binary to execute.

5. Now write a program that uses wait() to wait for the child process to finish in the parent. What does wait() return? What happens if you use wait() in the child?

`wait()` returns the status of the child process that has finished.
Using `wait()` in the child returns -1 because there's nothing to wait on.

6. Write a slight modification of the previous program, this time using waitpid() instead of wait(). When would waitpid() be useful?

When a parent has multiple children processes and wants to wait on one of them specifically.

7. Write a program that creates a child process, and then in the child closes standard output (STDOUT FILENO). What happens if the child calls printf() to print some output after closing the descriptor?

Oddly enough Rust's `println` macro seems to work even after the `stdout` fd has been closed. Unsure why.

8. Write a program that creates two children, and connects the standard output of one to the standard input of the other, using the pipe() system call.

Look at code