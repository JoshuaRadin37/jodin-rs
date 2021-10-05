use crate::symbols::Symbol;
use byteorder::{ByteOrder, LittleEndian};
use std::collections::VecDeque;

trait ConstantPoolTrait: From<VecDeque<u8>> + Into<Vec<u8>> {
    fn get(&self, key: usize) -> &ConstantPoolEntry;
    fn num_entries(&self) -> usize;
}

#[derive(Debug, Default)]
pub struct ConstantPool {
    pub entries: Vec<ConstantPoolEntry>,
}

impl From<ConstantPool> for Vec<u8> {
    fn from(c: ConstantPool) -> Self {
        let mut buffer = vec![];
        buffer.extend((c.entries.len() as u16).to_le_bytes());
        let vector = c.entries;
        for entry in vector {
            entry.append_to_vector(&mut buffer);
        }
        buffer
    }
}

#[derive(Debug)]
pub enum ConstantPoolEntry {
    UTF8(String),
    Symbol(Symbol),
}

impl ConstantPoolEntry {
    fn append_to_vector(self, buffer: &mut Vec<u8>) {
        match self {
            ConstantPoolEntry::UTF8(s) => {
                let len = s.len() as u16;
                buffer.extend(len.to_le_bytes());
                let bytes = s.as_bytes();
                buffer.extend(bytes);
            }
            ConstantPoolEntry::Symbol(sym) => {
                let s = sym.to_string();
                let len = s.len() as u16;
                buffer.extend(len.to_le_bytes());
                let bytes = s.as_bytes();
                buffer.extend(bytes);
            }
        }
    }

    fn parse_utf8(vec: &mut VecDeque<u8>) -> String {
        let len = ConstantPoolEntry::parse_u16(vec) as usize;
        let buffer = vec.drain(0..len).collect::<Vec<_>>();
        String::from_utf8(buffer).expect("Invalid UTF-8 formatted string")
    }

    fn parse_u16(vec: &mut VecDeque<u8>) -> u16 {
        let bytes = vec.drain(0..2).collect::<Vec<_>>();
        let short = LittleEndian::read_u16(&*bytes);
        short
    }

    fn parse_symbol(vec: &mut VecDeque<u8>) -> Symbol {
        let string_utf8 = Self::parse_utf8(vec);
        string_utf8
            .parse()
            .expect(format!("{} is not a valid symbol", string_utf8).as_str())
    }
}

impl From<&mut VecDeque<u8>> for ConstantPoolEntry {
    fn from(vector: &mut VecDeque<u8>) -> Self {
        let tag = vector.pop_front().unwrap();
        match tag {
            1 => {
                let string = ConstantPoolEntry::parse_utf8(vector);
                ConstantPoolEntry::UTF8(string)
            }
            2 => {
                let symbol = ConstantPoolEntry::parse_symbol(vector);
                ConstantPoolEntry::Symbol(symbol)
            }
            _ => {
                panic!("Invalid tag: {}", tag)
            }
        }
    }
}
