#![feature(allocator_api)]

use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{BufRead, Write};
use std::error::Error;

use lazy_static::*;
use std::alloc::Global;

lazy_static! {
    static ref TEST_BEFORE: AtomicBool = AtomicBool::new(false);
    static ref TEST_RESULT: AtomicBool = AtomicBool::new(false);
}

fn is_little_endian() -> bool {
    if !TEST_BEFORE.deref().load(Ordering::Relaxed) {
        unsafe {
            union Value {
                value_u16: u16,
                value_u8s: [u8; 2],
            }

            let val = Value { value_u16: 0x0102u16 };

            if val.value_u8s[0] == 0x01u8 && val.value_u8s[1] == 0x02u8 {
                TEST_RESULT.deref().store(false, Ordering::Relaxed);
            } else if val.value_u8s[0] == 0x02u8 && val.value_u8s[1] == 0x01u8 {
                TEST_RESULT.deref().store(true, Ordering::Relaxed);
            } else {
                panic!("is_little_endian: maybe something wrong");
            }
        }
        TEST_BEFORE.deref().store(true, Ordering::Relaxed);
    }
    TEST_RESULT.deref().load(Ordering::Relaxed)
}

trait Serialize {
    fn serialize(&self, w: &mut dyn Write)-> Result<bool, Box<dyn Error, Global>>;
}

trait DeSerialize {
    fn deserialize(&mut self, r: &mut dyn BufRead)-> Result<bool, Box<dyn Error, Global>>;
}

impl Serialize for bool {
    fn serialize(&self, w: &mut dyn Write)-> Result<bool, Box<dyn Error, Global>> {
        if *self {
            w.write_all(&[0x1u8])?;
        } else {
            w.write_all(&[0x0u8])?;
        }
        Ok(true)
    }
}

impl DeSerialize for bool {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<bool, Box<dyn Error, Global>> {
        let mut buffer = [0x0u8; 1];
        r.read(&mut buffer)?;

        if buffer[0] == 0x0u8 {
            *self = false;
        } else if buffer[0] == 0x1u8 {
            *self = true;
        } else {
            Err("deserialize bool error: invalid bool value")?
        }
        Ok(true)
    }
}

impl Serialize for char {
    fn serialize(&self, w: &mut dyn Write) -> Result<bool, Box<dyn Error, Global>> {
        w.write_all(&[self.clone() as u8])?;
        Ok(true)
    }
}

impl DeSerialize for char {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<bool, Box<dyn Error, Global>> {
        let mut buffer = [0x0u8; 1];
        r.read(&mut buffer)?;
        *self = buffer[0] as char;
        Ok(true)
    }
}

impl Serialize for i8 {
    fn serialize(&self, w: &mut dyn Write) -> Result<bool, Box<dyn Error, Global>> {
        w.write_all(&[self.clone() as u8])?;
        Ok(true)
    }
}

impl DeSerialize for i8 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<bool, Box<dyn Error, Global>> {
        let mut buffer = [0x0u8; 1];
        r.read(&mut buffer)?;
        *self = buffer[0] as i8;
        Ok(true)
    }
}

impl Serialize for u8 {
    fn serialize(&self, w: &mut dyn Write) -> Result<bool, Box<dyn Error, Global>> {
        w.write_all(&[self.clone()])?;
        Ok(true)
    }
}

impl DeSerialize for u8 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<bool, Box<dyn Error, Global>> {
        let mut buffer = [0x0u8; 1];
        r.read(&mut buffer)?;
        *self = buffer[0];
        Ok(true)
    }
}

// impl Serialize for i16 {
// }
//
// impl Serialize for u16 {
// }
//
// impl Serialize for i32 {
// }
//
// impl Serialize for u32 {
// }
//
// impl Serialize for i64 {
// }
//
// impl Serialize for u64 {
// }
//
// impl Serialize for i128 {
// }
//
// impl Serialize for u128 {
// }
//
// impl Serialize for f32 {
// }
//
// impl Serialize for f64 {
// }
//
// impl Serialize for &str {
// }
//
// impl Serialize for String {
// }
//
// impl Serialize for &[T] {
// }
//
// impl Serialize for Vec<T> {
// }
//
// impl Serialize for VecDeque<T> {
// }
//
// impl Serialize for LinkedList<T> {
// }
//
// impl Serialize for HashMap<K,V> {
// }
//
// impl Serialize for BTreeMap<K,V> {
// }
//
// impl Serialize for HashSet<K> {
// }
//
// impl Serialize for BTreeSet<K> {
// }
//
// impl Serialize for BinaryHeap<T> {
// }

#[cfg(test)]
mod tests {
    use crate::{is_little_endian, Serialize, DeSerialize};
    use std::io::{BufWriter, Cursor};

    #[test]
    fn test_is_little_endian() {
        assert_eq!(is_little_endian(), true);
    }

    #[test]
    fn test_serialize_bool() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = true.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 1);
        assert_eq!(*(buf.buffer().get(0).unwrap()), 0x1u8);
    }

    #[test]
    fn test_deserialize_bool() {
        let mut buf = Cursor::new(vec![0x01u8]);
        let mut val: bool = false;
        let r = val.deserialize(&mut buf);
        match r {
            Err(e) => assert!(false, "{}", e.to_string()),
            _ => {}
        }
        assert_eq!(val, true);
    }

    #[test]
    fn test_serialize_char() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = 'a'.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 1);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as char, 'a');
    }

    #[test]
    fn test_deserialize_char() {
        let mut buf = Cursor::new(vec!['a' as u8]);
        let mut val: char = 0x0 as char;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, 'a');
    }

    #[test]
    fn test_serialize_i8() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (-128i8).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 1);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as i8, -128i8);
    }

    #[test]
    fn test_deserialize_i8() {
        let mut buf = Cursor::new(vec![(-128i8) as u8]);
        let mut val: i8 = 0x0 as i8;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, -128i8);
    }

    #[test]
    fn test_serialize_u8() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (0xffu8).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 1);
        assert_eq!(*(buf.buffer().get(0).unwrap()), 0xffu8);
    }

    #[test]
    fn test_deserialize_u8() {
        let mut buf = Cursor::new(vec![0xffu8]);
        let mut val: u8 = 0x0 as u8;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, 0xffu8);
    }
}
