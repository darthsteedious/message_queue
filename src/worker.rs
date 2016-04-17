extern crate nix;

use std::os::unix::io::RawFd;
use nix::sys::ioctl::libc::pid_t;
use nix::unistd::{getpid};

pub enum Action {
    Create(String, i32),
    Read(String),
    Update(String, i32),
    Delete(String)
}

pub struct Message {
    origin: u32,
    pid: pid_t,
    action: Action
}

pub struct Worker {
    id: u32,
    pid: pid_t,
    request_pipe: (RawFd, RawFd),
    response_pipe: (RawFd, RawFd)
}

impl Worker {
    pub fn send_message(message: Message) {

    }
    pub fn await_message() -> Message {
        let id: u32 = 1;
        Message { origin: id, pid: getpid(), action: Action::Read("foo".to_string()) }
    }
}
