extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate bytes;

use futures::sync::mpsc;
use futures::{Sink, Future, Stream};
use std::process::Command;
use std::net::SocketAddr;
use tokio_core::reactor::Core;
use std::io::{self, Read, Write};
use std::thread;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let address: SocketAddr = "0.0.0.0:5001".parse().unwrap();

    // spawn thread to handle something?
    let (stdin_tx, stdin_rx) = mpsc::channel(0);
    thread::spawn(|| read_stdin(stdin_tx));
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

fn read_stdin(mut tx: mpsc::Sender<Vec<u8>>) {
    let mut stdin = io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf) {
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

mod tcp {
    use std::io;
    use std::net::SocketAddr;

    use bytes::{BufMut, BytesMut};
    use futures::{Future, Stream};
    use tokio_core::net::TcpStream;
    use tokio_core::reactor::Handle;
    use tokio_io::AsyncRead;
    use tokio_io::codec::{Encoder, Decoder};

    pub fn connect(address: &SocketAddr,
                   handle: &Handle,
                   stream_in: Box<Stream<Item = Vec<u8>, Error = io::Error>>)
        -> Box<Stream<Item = BytesMut, Error = io::Error>> {
        let tcp = TcpStream::connect(address, handle);
        let handle = handle.clone();

        Box::new(tcp.map(move |stream| {
            let (sink, stream) = stream.framed(Bytes).split();
            handle.spawn(stream_in.forward(sink).then(|result| {
                if let Err(e) = result {
                    panic!("failed to write to socket: {}", e)
                }
                Ok(())
            }));
            stream
        }).flatten_stream())
    }

    // Simple codec for sending bytes over TCP
    struct Bytes;

    impl Decoder for Bytes {
        type Item = BytesMut;
        type Error = io::Error;

        fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
            if buf.len() > 0 {
                let len = buf.len();
                Ok(Some(buf.split_to(len)))
            } else {
                Ok(None)
            }
        }

        fn decode_eof(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
            self.decode(buf)
        }
    }

    impl Encoder for Bytes {
        type Item = Vec<u8>;
        type Error = io::Error;

        fn encode(&mut self, data: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
            buf.put(&data[..]);
            Ok(())
        }
    }
}