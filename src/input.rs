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

    pub fn read(&mut self) -> u32{
        let mut buf: [u8; 4] = [0; 4];
        self.data.read_exact(&mut buf).expect("Error while reading input stream of data");

        return u32::from_be_bytes(buf);
    }
}