extern crate futures;
extern crate tokio_core;
extern crate tokio_process;

use std::process::Command;

use tokio_core::reactor::Core;
use tokio_process::CommandExt;

fn main() {
    let mut core = Core::new().unwrap();

    let mut command = Command::new("raspivid");
    command.arg("-t").arg("0").arg("-o").arg("-");

    let child = command
        .spawn_async(&core.handle())
        .expect("failed to spawn");

    match core.run(child) {
        Ok(status) => println!("exit status: {}", status),
        Err(e) => panic!("failed to run process: {}", e)
    }

}
