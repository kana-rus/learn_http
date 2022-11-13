use std::{
    net::TcpStream,
    io::{Read, Error},
};
use crate::http1::utils::consts::BUFF_SIZE;


pub(crate) struct TcpReader {
    stream: TcpStream,
    buf: [u8; BUFF_SIZE],
    pos: usize,
}

impl TcpReader {
    pub fn new(mut stream: TcpStream) -> Result<Self, Error> {
        let mut buf = [0; BUFF_SIZE];
        stream.read(&mut buf)?;
        Ok(Self { stream, buf, pos: 0 })
    }
    pub fn read_line(&mut self) -> Option<&[u8]> {
        if self.pos >= BUFF_SIZE { return None }
        for eol in self.pos..BUFF_SIZE-1 {
            if self.buf[eol]   == b'\r'  
            && self.buf[eol+1] == b'\n' {
                let line = &self.buf[self.pos..eol];
                self.pos = eol + 2;
                return Some(line)
            }
        }
        None
    }
}
