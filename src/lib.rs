use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{BufRead, Write};
use std::error::Error;

use lazy_static::*;

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
    fn serialize(&self, w: &mut dyn Write)-> Result<bool, Box<dyn Error>>;
}

trait DeSerialize {
    fn deserialize(&mut self, r: &mut dyn BufRead)-> Result<bool, Box<dyn Error>>;
}

impl Serialize for bool {
    fn serialize(&self, w: &mut dyn Write)-> Result<bool, Box<dyn Error>> {
        if *self {
            w.write_all(&[0x1u8])?;
        } else {
            w.write_all(&[0x0u8])?;
        }
        Ok(true)
    }
}

impl DeSerialize for bool {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<bool, Box<dyn Error>> {
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

// impl Serialize for char {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for i8 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for u8 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for i16 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for u16 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for i32 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for u32 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for i64 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for u64 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for i128 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for u128 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for f32 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for f64 {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for &str {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for String {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for &[T] {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for Vec<T> {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for VecDeque<T> {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for LinkedList<T> {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for HashMap<K,V> {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for BTreeMap<K,V> {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for HashSet<K> {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for BTreeSet<K> {
//     fn serialize() {
//         unimplemented!()
//     }
// }
//
// impl Serialize for BinaryHeap<T> {
//     fn serialize() {
//         unimplemented!()
//     }
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
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, true);
    }
}
