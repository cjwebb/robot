extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate bytes;

use futures::sync::mpsc;
use futures::{Sink, Future, Stream};
use std::net::SocketAddr;
use tokio_core::reactor::Core;
use std::io::{self, Read, Write};
use std::thread;
use std::process::Command;
use std::process::Stdio;

mod tcp;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    // todo - take socket address as argument
    let address: SocketAddr = "0.0.0.0:5001".parse().unwrap();
//    let address: SocketAddr = "192.168.1.95:5001".parse().unwrap();

    let (stdin_tx, stdin_rx) = mpsc::channel(0);
    thread::spawn(|| run_child_process(stdin_tx));

    let stdin_rx = stdin_rx.map_err(|_| panic!());
    let stdout = tcp::connect(&address, &handle, Box::new(stdin_rx));
    let mut out = io::stdout();
    let server = stdout.for_each(|chunk| {
        out.write_all(&chunk)
    });

    match core.run(server) {
        Ok(_) => println!("exited"),
        Err(e) => panic!("failed to run process: {}", e)
    }
}

fn run_child_process(mut tx: mpsc::Sender<Vec<u8>>) {
    let mut command = get_command();
    let mut child = command
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn");
    let mut child_stdout = child.stdout.take().unwrap();

    loop {
        let mut buf = vec![0; 1024];
        let n = match child_stdout.read(&mut buf) {
            Err(_) |
            Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx = match tx.send(buf).wait() {
            Ok(tx) => tx,
            Err(_) => break,
        };
    }
}

fn get_command() -> Command {
//    let mut command = Command::new("raspivid");
//    command.arg("-t").arg("0").arg("-o").arg("-");
    let mut command = Command::new("bash");
    command.arg("scripts/looping.sh");
    command
}
