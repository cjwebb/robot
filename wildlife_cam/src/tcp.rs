/// This module is taken from a Tokio example, found here:
/// https://github.com/tokio-rs/tokio-core/blob/master/examples/connect.rs

extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate bytes;

use std::io;
use std::net::SocketAddr;

use bytes::BytesMut;
use futures::{Future, Stream};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_io::AsyncRead;
use tokio_io::codec::{Encoder, Decoder};

pub fn connect(address: &SocketAddr,
               handle: &Handle,
               stream_in: Box<Stream<Item=Vec<u8>, Error=io::Error>>)
               -> Box<Stream<Item=BytesMut, Error=io::Error>> {
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
        buf.extend(&data[..]);
        Ok(())
    }
}
