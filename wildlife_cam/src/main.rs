extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate bytes;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use futures::sync::mpsc;
use futures::{Sink, Future, Stream};
use std::net::SocketAddr;
use tokio_core::reactor::Core;
use std::io::{self, Read, Write};
use std::thread;
use std::process::Command;
use std::process::Stdio;
use configuration::Configuration;

mod tcp;
mod configuration;

/// TODO
/// - Make this into a server, rather than a client.
/// - Save in a buffer, and run allow incoming connections to read
/// - Chunk into h264 frames
/// - Work in frames, rather than byte-stream?

fn main() {
    let configuration = Configuration::new().unwrap();
    println!("Starting with {:?}", configuration);

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let address: SocketAddr = configuration.server.address.parse().unwrap();

    let (stdin_tx, stdin_rx) = mpsc::channel(0);
    thread::spawn(move || run_child_process(&configuration, stdin_tx));

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

fn run_child_process(configuration: &Configuration, mut tx: mpsc::Sender<Vec<u8>>) {
    let mut command = get_command(configuration);
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

        // todo - split into frames here

        tx = match tx.send(buf).wait() {
            Ok(tx) => tx,
            Err(_) => break,
        };
    }
}

fn get_command(configuration: &Configuration) -> Command {
    let mut command = Command::new(&configuration.command.program);

    for x in configuration.command.arguments.split_whitespace() {
        command.arg(x);
    }

    command
}
