#[macro_use]
extern crate log;
extern crate nix;
extern crate message_queue;

use std::io::{Result};

use nix::unistd::*;
use nix::sys::wait::{waitpid};
use message_queue::logger::FileLogger;

const LOG_FILE_NAME: &'static str = "/Users/Steed/Projects/rust/message_queue/log";

fn main() {
    let pipe1_req = pipe();
    let pipe1_res = pipe();

    let pipe2_req = pipe();
    let pipe2_res = pipe();

    for i in 0..2 {
        let pid = fork();

        if let Ok(fork_result) = pid {
            match fork_result {
                Fork::Parent(child) => {

                },
                Fork::Child => {

                }
            }
        } else {
            error!("Error forking process!");
        }
    }
    // if let Ok(_) = FileLogger::init("/Users/Steed/Projects/rust/message_queue/log") {
    //     info!("Application starting.");
    //
    //     debug!("Application running.");
    //     error!("Fake error!");
    //
    //     warn!("Application exiting.");
    // }
}

fn fork_workers() -> Result<()> {


    Ok(())
}
