extern crate byteorder;

use byteorder::{BigEndian, WriteBytesExt};

use std::io::{Read, Cursor, Bytes};
use std::fs::File;

pub struct InputBuffer<T>{
    pub data: Cursor<T>
}

#[allow(dead_code)]
impl<T> InputBuffer<T> where 
    Cursor<T>: Read
{
    pub fn from_file(path: String) -> InputBuffer<Bytes<File>>{
        let file = std::fs::File::open(path).expect("Error while reading input file");

        return InputBuffer{
            data: Cursor::new(file.bytes())
        };
    }

    pub fn from_vector_u8(vector: Vec<u8>) -> InputBuffer<Vec<u8>>{
        return InputBuffer{
            data: Cursor::new(vector)
        };
    }

    pub fn from_vector_u32(vector: Vec<u32>) -> InputBuffer<Vec<u8>>{
        let mut res: Vec<u8> = vec!(); 

        for n in vector{
            res.write_u32::<BigEndian>(n).unwrap();
        }

        return InputBuffer::<T>::from_vector_u8(res);
    }

    pub fn read_u32(&mut self) -> u32{
        let mut buf: [u8; 4] = [0; 4];
        self.data.read_exact(&mut buf).expect("Error while reading input stream of data");

        return u32::from_be_bytes(buf);
    }

    pub fn read_u16(&mut self) -> u16{
        let mut buf: [u8; 2] = [0; 2];
        self.data.read_exact(&mut buf).expect("Error while reading input stream of data");

        return u16::from_be_bytes(buf);
    }

    pub fn read_u8(&mut self) -> u8{
        let mut buf: [u8; 1] = [0; 1];
        self.data.read_exact(&mut buf).expect("Error while reading input stream of data");

        return buf[0];
    }
}