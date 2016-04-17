#[macro_use]
extern crate log;
extern crate nix;
extern crate message_queue;

use nix::unistd::*;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::sys::ioctl::libc::pid_t;
use message_queue::logger::FileLogger;
use message_queue::worker::*;

const LOG_FILE_NAME: &'static str = "/Users/Steed/Projects/rust/message_queue/log";

fn main() {
    let mut pids: [pid_t; 2] = [0; 2];
    for i in 0..2 {
        let pid = fork();

        match pid.expect("Error forking process!") {
            Fork::Parent(child) => {
                pids[i] = child;

                println!("Hello from the parent! - Child: {} - Me: {}", child, getpid());
                continue;
            },
            Fork::Child => {
                println!("Hello from the child! - Me: {}", getpid());
                return;
            }
        }
    }

    if let Ok(_) = FileLogger::init(LOG_FILE_NAME) {
        info!("Application and children started.")
    }

    for pid in &pids {
        if let WaitStatus::Exited(c_pid, rc) = waitpid(*pid, Option::None).expect("Error waiting") {
            warn!("Child {} exited with return code {}.", c_pid, rc);
        }
    }

    warn!("Application ending.");
}
