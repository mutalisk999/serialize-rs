use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
use std::io::{BufRead, Write};
use std::error::Error;
use std::hash::Hash;

pub trait Serialize {
    fn serialize(&self, w: &mut dyn Write)-> Result<(), Box<dyn Error>>;
}

pub trait DeSerialize {
    fn deserialize(&mut self, r: &mut dyn BufRead)-> Result<(), Box<dyn Error>>;
}

impl Serialize for bool {
    fn serialize(&self, w: &mut dyn Write)-> Result<(), Box<dyn Error>> {
        if *self {
            w.write_all(&[0x1u8])?;
        } else {
            w.write_all(&[0x0u8])?;
        }
        Ok(())
    }
}

impl DeSerialize for bool {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 1];
        r.read_exact(&mut buffer)?;

        if buffer[0] == 0x0u8 {
            *self = false;
        } else if buffer[0] == 0x1u8 {
            *self = true;
        } else {
            Err("deserialize bool error: invalid bool value")?
        }
        Ok(())
    }
}

impl Serialize for char {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&[self.clone() as u8])?;
        Ok(())
    }
}

impl DeSerialize for char {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 1];
        r.read_exact(&mut buffer)?;
        *self = buffer[0] as char;
        Ok(())
    }
}

impl Serialize for i8 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&[self.clone() as u8])?;
        Ok(())
    }
}

impl DeSerialize for i8 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 1];
        r.read_exact(&mut buffer)?;
        *self = buffer[0] as i8;
        Ok(())
    }
}

impl Serialize for u8 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&[self.clone()])?;
        Ok(())
    }
}

impl DeSerialize for u8 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 1];
        r.read_exact(&mut buffer)?;
        *self = buffer[0];
        Ok(())
    }
}

impl Serialize for i16 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&i16::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for i16 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 2];
        r.read_exact(&mut buffer)?;
        *self = i16::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for u16 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&u16::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for u16 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 2];
        r.read_exact(&mut buffer)?;
        *self = u16::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for i32 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&i32::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for i32 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 4];
        r.read_exact(&mut buffer)?;
        *self = i32::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for u32 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&u32::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for u32 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 4];
        r.read_exact(&mut buffer)?;
        *self = u32::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for i64 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&i64::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for i64 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 8];
        r.read_exact(&mut buffer)?;
        *self = i64::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for u64 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&u64::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for u64 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 8];
        r.read_exact(&mut buffer)?;
        *self = u64::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for i128 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&i128::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for i128 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 16];
        r.read_exact(&mut buffer)?;
        *self = i128::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for u128 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&u128::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for u128 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 16];
        r.read_exact(&mut buffer)?;
        *self = u128::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for f32 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&f32::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for f32 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 4];
        r.read_exact(&mut buffer)?;
        *self = f32::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for f64 {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        w.write_all(&f64::to_be_bytes(self.clone()))?;
        Ok(())
    }
}

impl DeSerialize for f64 {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0x0u8; 8];
        r.read_exact(&mut buffer)?;
        *self = f64::from_be_bytes(buffer);
        Ok(())
    }
}

impl Serialize for str {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for c in self.chars() {
            c.serialize(w)?;
        }
        Ok(())
    }
}

impl Serialize for String {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for c in self.chars() {
            c.serialize(w)?;
        }
        Ok(())
    }
}

impl DeSerialize for String {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        if length == 0 {
            *self = String::from("");
        } else {
            // vec new with capacity
            let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
            // set size
            buffer.resize(length as usize, 0x0 as u8);
            r.read_exact(&mut buffer)?;
            *self = buffer.iter().map(|x| *x as char).collect::<String>();
        }
        Ok(())
    }
}

impl<T> Serialize for [T]
    where T: Serialize {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for v in self.iter() {
            v.serialize(w)?;
        }
        Ok(())
    }
}

impl<T> Serialize for Option<T>
    where T: Serialize {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        match self {
            Some(v) => {
                true.serialize(w)?;
                v.serialize(w)?;
            },
            None => {
                false.serialize(w)?;
            }
        }
        Ok(())
    }
}

impl<T> DeSerialize for Option<T>
    where T: DeSerialize {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut b: bool = false;
        b.deserialize(r)?;

        if !b {
            *self = None;
        } else {
            match self {
                Some(t) => {
                    t.deserialize(r)?;
                },
                _ => {
                    return Err("can not deserialize to None type")?;
                }
            }
        }
        Ok(())
    }
}

impl<T> Serialize for Vec<T>
    where T: Serialize {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for v in self.iter() {
            v.serialize(w)?;
        }
        Ok(())
    }
}

impl<T> DeSerialize for Vec<T>
    where T: DeSerialize + Default {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        let mut vec: Vec<T> = Vec::new();
        if length != 0 {
            for _ in 0..length {
                let mut v: T = T::default();
                v.deserialize(r)?;
                vec.push(v);
            }
        }
        *self = vec;
        Ok(())
    }
}

impl<T> Serialize for VecDeque<T>
    where T: Serialize {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for v in self.iter() {
            v.serialize(w)?;
        }
        Ok(())
    }
}

impl<T> DeSerialize for VecDeque<T>
    where T: DeSerialize + Default {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        let mut vec_deque: VecDeque<T> = VecDeque::new();
        if length != 0 {
            for _ in 0..length {
                let mut v: T = T::default();
                v.deserialize(r)?;
                vec_deque.push_back(v);
            }
        }
        *self = vec_deque;
        Ok(())
    }
}

impl<T> Serialize for LinkedList<T>
    where T: Serialize {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for v in self.iter() {
            v.serialize(w)?;
        }
        Ok(())
    }
}

impl<T> DeSerialize for LinkedList<T>
    where T: DeSerialize + Default {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        let mut list: LinkedList<T> = LinkedList::new();
        if length != 0 {
            for _ in 0..length {
                let mut v: T = T::default();
                v.deserialize(r)?;
                list.push_back(v);
            }
        }
        *self = list;
        Ok(())
    }
}

impl<K,V> Serialize for HashMap<K,V>
    where K: Serialize + Hash, V: Serialize {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for (k,v) in self.iter() {
            k.serialize(w)?;
            v.serialize(w)?;
        }
        Ok(())
    }
}

impl<K,V> DeSerialize for HashMap<K,V>
    where K: DeSerialize + Default + Hash + Eq, V: DeSerialize + Default {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        let mut hash_map: HashMap<K,V> = HashMap::new();
        if length != 0 {
            for _ in 0..length {
                let mut k: K = K::default();
                let mut v: V = V::default();
                k.deserialize(r)?;
                v.deserialize(r)?;
                hash_map.insert(k,v);
            }
        }
        *self = hash_map;
        Ok(())
    }
}

impl<K,V> Serialize for BTreeMap<K,V>
    where K: Serialize + Ord, V: Serialize + Default {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for (k,v) in self.iter() {
            k.serialize(w)?;
            v.serialize(w)?;
        }
        Ok(())
    }
}

impl<K,V> DeSerialize for BTreeMap<K,V>
    where K: DeSerialize + Default + Ord, V: DeSerialize + Default{
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        let mut btree_map: BTreeMap<K,V> = BTreeMap::new();
        if length != 0 {
            for _ in 0..length {
                let mut k: K = K::default();
                let mut v: V = V::default();
                k.deserialize(r)?;
                v.deserialize(r)?;
                btree_map.insert(k,v);
            }
        }
        *self = btree_map;
        Ok(())
    }
}

impl<K> Serialize for HashSet<K>
    where K: Serialize + Hash {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for k in self.iter() {
            k.serialize(w)?;
        }
        Ok(())
    }
}

impl<K> DeSerialize for HashSet<K>
    where K: DeSerialize + Default + Hash + Eq {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        let mut hash_set: HashSet<K> = HashSet::new();
        if length != 0 {
            for _ in 0..length {
                let mut k: K = K::default();
                k.deserialize(r)?;
                hash_set.insert(k);
            }
        }
        *self = hash_set;
        Ok(())
    }
}

impl<K> Serialize for BTreeSet<K>
    where K: Serialize + Ord {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for k in self.iter() {
            k.serialize(w)?;
        }
        Ok(())
    }
}

impl<K> DeSerialize for BTreeSet<K>
    where K: DeSerialize + Default + Ord {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        let mut btree_set: BTreeSet<K> = BTreeSet::new();
        if length != 0 {
            for _ in 0..length {
                let mut k: K = K::default();
                k.deserialize(r)?;
                btree_set.insert(k);
            }
        }
        *self = btree_set;
        Ok(())
    }
}

impl<T> Serialize for BinaryHeap<T>
    where T: Serialize + Ord {
    fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let length = self.len() as u32;
        length.serialize(w)?;

        for k in self.iter() {
            k.serialize(w)?;
        }
        Ok(())
    }
}

impl<T> DeSerialize for BinaryHeap<T>
    where T: DeSerialize + Default + Ord {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
        let mut length :u32 = 0u32;
        length.deserialize(r)?;

        let mut binary_heap: BinaryHeap<T> = BinaryHeap::new();
        if length != 0 {
            for _ in 0..length {
                let mut t: T = T::default();
                t.deserialize(r)?;
                binary_heap.push(t);
            }
        }
        *self = binary_heap;
        Ok(())
    }
}

#[macro_export]
macro_rules! serialize_struct {
    ($struct_name:ty, $($member_name:ident),*) => {
        impl Serialize for $struct_name {
            fn serialize(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
                $(
                    self.$member_name.serialize(w)?;
                )*
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! deserialize_struct {
    ($struct_name:ty, $($member_name:ident),*) => {
        impl DeSerialize for $struct_name {
            fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
                $(
                    self.$member_name.deserialize(r)?;
                )*
                Ok(())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{Serialize, DeSerialize};
    use std::io::{BufWriter, Cursor, Write, BufRead};
    use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
    use std::error::Error;

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

    #[test]
    fn test_serialize_i16() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (-0x010ai16).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 2);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0xfeu8);
        assert_eq!(*(buf.buffer().get(1).unwrap()) as u8, 0xf6u8);
    }

    #[test]
    fn test_deserialize_i16() {
        let mut buf = Cursor::new(vec![(0xfe) as u8, (0xf6) as u8]);
        let mut val: i16 = 0x0 as i16;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, -0x010ai16);
    }

    #[test]
    fn test_serialize_u16() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (0x0102u16).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 2);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0x01u8);
        assert_eq!(*(buf.buffer().get(1).unwrap()) as u8, 0x02u8);
    }

    #[test]
    fn test_deserialize_u16() {
        let mut buf = Cursor::new(vec![(0x01) as u8, (0x02) as u8]);
        let mut val: u16 = 0x0 as u16;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, 0x0102u16);
    }

    #[test]
    fn test_serialize_i32() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (-0x01020304i32).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 4);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0xfeu8);
        assert_eq!(*(buf.buffer().get(1).unwrap()) as u8, 0xfdu8);
        assert_eq!(*(buf.buffer().get(2).unwrap()) as u8, 0xfcu8);
        assert_eq!(*(buf.buffer().get(3).unwrap()) as u8, 0xfcu8);
    }

    #[test]
    fn test_deserialize_i32() {
        let mut buf = Cursor::new(vec![(0xfe) as u8, (0xfd) as u8,
                                       (0xfc) as u8, (0xfc) as u8]);
        let mut val: i32 = 0x0 as i32;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, -0x01020304i32);
    }

    #[test]
    fn test_serialize_u32() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (0x01020304u32).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 4);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0x01u8);
        assert_eq!(*(buf.buffer().get(1).unwrap()) as u8, 0x02u8);
        assert_eq!(*(buf.buffer().get(2).unwrap()) as u8, 0x03u8);
        assert_eq!(*(buf.buffer().get(3).unwrap()) as u8, 0x04u8);
    }

    #[test]
    fn test_deserialize_u32() {
        let mut buf = Cursor::new(vec![(0x01) as u8, (0x02) as u8,
                                       (0x03) as u8, (0x04) as u8]);
        let mut val: u32 = 0x0 as u32;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, 0x01020304u32);
    }

    #[test]
    fn test_serialize_i64() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (-0x0102030405060708i64).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0xfeu8);
        assert_eq!(*(buf.buffer().get(1).unwrap()) as u8, 0xfdu8);
        assert_eq!(*(buf.buffer().get(2).unwrap()) as u8, 0xfcu8);
        assert_eq!(*(buf.buffer().get(3).unwrap()) as u8, 0xfbu8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as u8, 0xfau8);
        assert_eq!(*(buf.buffer().get(5).unwrap()) as u8, 0xf9u8);
        assert_eq!(*(buf.buffer().get(6).unwrap()) as u8, 0xf8u8);
        assert_eq!(*(buf.buffer().get(7).unwrap()) as u8, 0xf8u8);
    }

    #[test]
    fn test_deserialize_i64() {
        let mut buf = Cursor::new(vec![(0xfe) as u8, (0xfd) as u8,
                                       (0xfc) as u8, (0xfb) as u8,
                                       (0xfa) as u8, (0xf9) as u8,
                                       (0xf8) as u8, (0xf8) as u8]);
        let mut val: i64 = 0x0 as i64;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, -0x0102030405060708i64);
    }

    #[test]
    fn test_serialize_u64() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (0x0102030405060708u64).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0x01u8);
        assert_eq!(*(buf.buffer().get(1).unwrap()) as u8, 0x02u8);
        assert_eq!(*(buf.buffer().get(2).unwrap()) as u8, 0x03u8);
        assert_eq!(*(buf.buffer().get(3).unwrap()) as u8, 0x04u8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as u8, 0x05u8);
        assert_eq!(*(buf.buffer().get(5).unwrap()) as u8, 0x06u8);
        assert_eq!(*(buf.buffer().get(6).unwrap()) as u8, 0x07u8);
        assert_eq!(*(buf.buffer().get(7).unwrap()) as u8, 0x08u8);
    }

    #[test]
    fn test_deserialize_u64() {
        let mut buf = Cursor::new(vec![(0x01) as u8, (0x02) as u8,
                                       (0x03) as u8, (0x04) as u8,
                                       (0x05) as u8, (0x06) as u8,
                                       (0x07) as u8, (0x08) as u8]);
        let mut val: u64 = 0x0 as u64;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, 0x0102030405060708u64);
    }

    #[test]
    fn test_serialize_i128() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (-0x01020304050607080102030405060708i128).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 16);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0xfeu8);
        assert_eq!(*(buf.buffer().get(1).unwrap()) as u8, 0xfdu8);
        assert_eq!(*(buf.buffer().get(2).unwrap()) as u8, 0xfcu8);
        assert_eq!(*(buf.buffer().get(3).unwrap()) as u8, 0xfbu8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as u8, 0xfau8);
        assert_eq!(*(buf.buffer().get(5).unwrap()) as u8, 0xf9u8);
        assert_eq!(*(buf.buffer().get(6).unwrap()) as u8, 0xf8u8);
        assert_eq!(*(buf.buffer().get(7).unwrap()) as u8, 0xf7u8);
        assert_eq!(*(buf.buffer().get(8).unwrap()) as u8, 0xfeu8);
        assert_eq!(*(buf.buffer().get(9).unwrap()) as u8, 0xfdu8);
        assert_eq!(*(buf.buffer().get(10).unwrap()) as u8, 0xfcu8);
        assert_eq!(*(buf.buffer().get(11).unwrap()) as u8, 0xfbu8);
        assert_eq!(*(buf.buffer().get(12).unwrap()) as u8, 0xfau8);
        assert_eq!(*(buf.buffer().get(13).unwrap()) as u8, 0xf9u8);
        assert_eq!(*(buf.buffer().get(14).unwrap()) as u8, 0xf8u8);
        assert_eq!(*(buf.buffer().get(15).unwrap()) as u8, 0xf8u8);
    }

    #[test]
    fn test_deserialize_i128() {
        let mut buf = Cursor::new(vec![(0xfe) as u8, (0xfd) as u8,
                                       (0xfc) as u8, (0xfb) as u8,
                                       (0xfa) as u8, (0xf9) as u8,
                                       (0xf8) as u8, (0xf7) as u8,
                                       (0xfe) as u8, (0xfd) as u8,
                                       (0xfc) as u8, (0xfb) as u8,
                                       (0xfa) as u8, (0xf9) as u8,
                                       (0xf8) as u8, (0xf8) as u8]);
        let mut val: i128 = 0x0 as i128;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, -0x01020304050607080102030405060708i128);
    }

    #[test]
    fn test_serialize_u128() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (0x01020304050607080102030405060708u128).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 16);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0x01u8);
        assert_eq!(*(buf.buffer().get(1).unwrap()) as u8, 0x02u8);
        assert_eq!(*(buf.buffer().get(2).unwrap()) as u8, 0x03u8);
        assert_eq!(*(buf.buffer().get(3).unwrap()) as u8, 0x04u8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as u8, 0x05u8);
        assert_eq!(*(buf.buffer().get(5).unwrap()) as u8, 0x06u8);
        assert_eq!(*(buf.buffer().get(6).unwrap()) as u8, 0x07u8);
        assert_eq!(*(buf.buffer().get(7).unwrap()) as u8, 0x08u8);
        assert_eq!(*(buf.buffer().get(8).unwrap()) as u8, 0x01u8);
        assert_eq!(*(buf.buffer().get(9).unwrap()) as u8, 0x02u8);
        assert_eq!(*(buf.buffer().get(10).unwrap()) as u8, 0x03u8);
        assert_eq!(*(buf.buffer().get(11).unwrap()) as u8, 0x04u8);
        assert_eq!(*(buf.buffer().get(12).unwrap()) as u8, 0x05u8);
        assert_eq!(*(buf.buffer().get(13).unwrap()) as u8, 0x06u8);
        assert_eq!(*(buf.buffer().get(14).unwrap()) as u8, 0x07u8);
        assert_eq!(*(buf.buffer().get(15).unwrap()) as u8, 0x08u8);
    }

    #[test]
    fn test_deserialize_u128() {
        let mut buf = Cursor::new(vec![(0x01) as u8, (0x02) as u8,
                                       (0x03) as u8, (0x04) as u8,
                                       (0x05) as u8, (0x06) as u8,
                                       (0x07) as u8, (0x08) as u8,
                                       (0x01) as u8, (0x02) as u8,
                                       (0x03) as u8, (0x04) as u8,
                                       (0x05) as u8, (0x06) as u8,
                                       (0x07) as u8, (0x08) as u8]);
        let mut val: u128 = 0x0 as u128;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, 0x01020304050607080102030405060708u128);
    }

    #[test]
    fn test_serialize_deserialize_f32() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (0.12345 as f32).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 4);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: f32 = 0.0 as f32;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, 0.12345f32);
    }

    #[test]
    fn test_serialize_deserialize_f64() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = (0.123456789012345 as f64).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: f64 = 0.0 as f64;
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, 0.123456789012345f64);
    }

    #[test]
    fn test_serialize_str() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = ("abcd" as &'static str).serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as char, 'a');
        assert_eq!(*(buf.buffer().get(5).unwrap()) as char, 'b');
        assert_eq!(*(buf.buffer().get(6).unwrap()) as char, 'c');
        assert_eq!(*(buf.buffer().get(7).unwrap()) as char, 'd');
    }

    #[test]
    fn test_serialize_deserialize_string() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = String::from("abcd").serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as char, 'a');
        assert_eq!(*(buf.buffer().get(5).unwrap()) as char, 'b');
        assert_eq!(*(buf.buffer().get(6).unwrap()) as char, 'c');
        assert_eq!(*(buf.buffer().get(7).unwrap()) as char, 'd');

        let mut buf = Cursor::new(buf.buffer());
        let mut val: String = String::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, String::from("abcd"));
    }

    // with generic type
    #[test]
    fn test_serialize_slice() {
        let l = ['a', 'b', 'c', 'd'];
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = l.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as char, 'a');
        assert_eq!(*(buf.buffer().get(5).unwrap()) as char, 'b');
        assert_eq!(*(buf.buffer().get(6).unwrap()) as char, 'c');
        assert_eq!(*(buf.buffer().get(7).unwrap()) as char, 'd');
    }

    // with generic type
    #[test]
    fn test_serialize_deserialize_option() {
        // serialize/deserialize Some
        let o: Option<String> = Some(String::from("abcd"));
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = o.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 9);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 1);
        assert_eq!(*(buf.buffer().get(5).unwrap()) as char, 'a');
        assert_eq!(*(buf.buffer().get(6).unwrap()) as char, 'b');
        assert_eq!(*(buf.buffer().get(7).unwrap()) as char, 'c');
        assert_eq!(*(buf.buffer().get(8).unwrap()) as char, 'd');

        let mut buf = Cursor::new(buf.buffer());
        let mut val: Option<String> = Some(String::new());
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, Some(String::from("abcd")));

        // serialize/deserialize None
        let o: Option<String> = None;
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = o.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 1);
        assert_eq!(*(buf.buffer().get(0).unwrap()) as u8, 0);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: Option<String> = Some(String::new());
        let _ = val.deserialize(&mut buf);
        assert_eq!(val, None);

        // error: try to deserialize to None type
        let o: Option<String> = Some(String::from("abcd"));
        let mut buf = BufWriter::new(Vec::new());
        let _ = o.serialize(&mut buf);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: Option<String> = None;
        let r = val.deserialize(&mut buf);
        match r {
            Err(e) => {
                assert_eq!(e.to_string(), String::from("can not deserialize to None type"));
            },
            _ => {}
        }
    }

    #[test]
    fn test_serialize_deserialize_vector() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let _ = vec!['a','b','c','d'].serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as char, 'a');
        assert_eq!(*(buf.buffer().get(5).unwrap()) as char, 'b');
        assert_eq!(*(buf.buffer().get(6).unwrap()) as char, 'c');
        assert_eq!(*(buf.buffer().get(7).unwrap()) as char, 'd');

        let mut buf = Cursor::new(buf.buffer());
        let mut val: Vec<char> = Vec::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.len(), 4);
        assert_eq!(val[0], 'a');
        assert_eq!(val[1], 'b');
        assert_eq!(val[2], 'c');
        assert_eq!(val[3], 'd');
    }

    #[test]
    fn test_serialize_deserialize_vector_deque() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let mut vec_deque: VecDeque<char> = VecDeque::new();
        vec_deque.push_back('a');
        vec_deque.push_back('b');
        vec_deque.push_back('c');
        vec_deque.push_back('d');
        let _ = vec_deque.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as char, 'a');
        assert_eq!(*(buf.buffer().get(5).unwrap()) as char, 'b');
        assert_eq!(*(buf.buffer().get(6).unwrap()) as char, 'c');
        assert_eq!(*(buf.buffer().get(7).unwrap()) as char, 'd');

        let mut buf = Cursor::new(buf.buffer());
        let mut val: VecDeque<char> = VecDeque::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.len(), 4);
        assert_eq!(val[0], 'a');
        assert_eq!(val[1], 'b');
        assert_eq!(val[2], 'c');
        assert_eq!(val[3], 'd');
    }

    #[test]
    fn test_serialize_deserialize_linked_list() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let mut linked_list: LinkedList<char> = LinkedList::new();
        linked_list.push_back('a');
        linked_list.push_back('b');
        linked_list.push_back('c');
        linked_list.push_back('d');
        let _ = linked_list.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);
        assert_eq!(*(buf.buffer().get(4).unwrap()) as char, 'a');
        assert_eq!(*(buf.buffer().get(5).unwrap()) as char, 'b');
        assert_eq!(*(buf.buffer().get(6).unwrap()) as char, 'c');
        assert_eq!(*(buf.buffer().get(7).unwrap()) as char, 'd');

        let mut buf = Cursor::new(buf.buffer());
        let mut val: LinkedList<char> = LinkedList::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.len(), 4);
        assert_eq!(val.pop_front().unwrap(), 'a');
        assert_eq!(val.pop_front().unwrap(), 'b');
        assert_eq!(val.pop_front().unwrap(), 'c');
        assert_eq!(val.pop_front().unwrap(), 'd');
    }

    #[test]
    fn test_serialize_deserialize_hash_map() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let mut hash_map: HashMap<char, i8> = HashMap::new();
        hash_map.insert('a', 0i8);
        hash_map.insert('b', 1i8);
        hash_map.insert('c', 2i8);
        hash_map.insert('d', 3i8);
        let _ = hash_map.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 12);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: HashMap<char, i8> = HashMap::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.len(), 4);
        assert_eq!(*val.get(&'a').unwrap(), 0i8);
        assert_eq!(*val.get(&'b').unwrap(), 1i8);
        assert_eq!(*val.get(&'c').unwrap(), 2i8);
        assert_eq!(*val.get(&'d').unwrap(), 3i8);
    }

    #[test]
    fn test_serialize_deserialize_btree_map() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let mut btree_map: BTreeMap<char, i8> = BTreeMap::new();
        btree_map.insert('a', 0i8);
        btree_map.insert('b', 1i8);
        btree_map.insert('c', 2i8);
        btree_map.insert('d', 3i8);
        let _ = btree_map.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 12);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: BTreeMap<char, i8> = BTreeMap::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.len(), 4);
        assert_eq!(*val.get(&'a').unwrap(), 0i8);
        assert_eq!(*val.get(&'b').unwrap(), 1i8);
        assert_eq!(*val.get(&'c').unwrap(), 2i8);
        assert_eq!(*val.get(&'d').unwrap(), 3i8);
    }

    #[test]
    fn test_serialize_deserialize_hash_set() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let mut hash_set: HashSet<char> = HashSet::new();
        hash_set.insert('a');
        hash_set.insert('b');
        hash_set.insert('c');
        hash_set.insert('d');
        let _ = hash_set.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: HashSet<char> = HashSet::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.len(), 4);
        assert_eq!(*val.get(&'a').unwrap(), 'a');
        assert_eq!(*val.get(&'b').unwrap(), 'b');
        assert_eq!(*val.get(&'c').unwrap(), 'c');
        assert_eq!(*val.get(&'d').unwrap(), 'd');
    }

    #[test]
    fn test_serialize_deserialize_btree_set() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let mut btree_set: BTreeSet<char> = BTreeSet::new();
        btree_set.insert('a');
        btree_set.insert('b');
        btree_set.insert('c');
        btree_set.insert('d');
        let _ = btree_set.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: BTreeSet<char> = BTreeSet::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.len(), 4);
        assert_eq!(*val.get(&'a').unwrap(), 'a');
        assert_eq!(*val.get(&'b').unwrap(), 'b');
        assert_eq!(*val.get(&'c').unwrap(), 'c');
        assert_eq!(*val.get(&'d').unwrap(), 'd');
    }

    #[test]
    fn test_serialize_deserialize_binary_heap() {
        let mut buf = BufWriter::new(Vec::new());
        assert_eq!(buf.buffer().len(), 0);
        let mut binary_heap: BinaryHeap<char> = BinaryHeap::new();
        binary_heap.push('c');
        binary_heap.push('a');
        binary_heap.push('d');
        binary_heap.push('b');
        let _ = binary_heap.serialize(&mut buf);
        assert_eq!(buf.buffer().len(), 8);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: BinaryHeap<char> = BinaryHeap::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.len(), 4);
        assert_eq!(val.pop().unwrap(), 'd');
        assert_eq!(val.pop().unwrap(), 'c');
        assert_eq!(val.pop().unwrap(), 'b');
        assert_eq!(val.pop().unwrap(), 'a');
    }

    #[test]
    fn test_serialize_deserialize_struct() {
        #[derive(Debug)]
        struct Xxxx
        {
            a: i32,
            b: String,
            c: Option<f32>
        }

        impl Xxxx {
            fn new() -> Xxxx {
                Xxxx {
                    a: 0i32,
                    b: String::new(),
                    c: Some(0.0f32)
                }
            }
        }

        impl Serialize for Xxxx {
            fn serialize(&self, w: &mut dyn Write)-> Result<(), Box<dyn Error>> {
                self.a.serialize(w)?;
                self.b.serialize(w)?;
                self.c.serialize(w)?;
                Ok(())
            }
        }

        impl DeSerialize for Xxxx {
            fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error>> {
                self.a.deserialize(r)?;
                self.b.deserialize(r)?;
                self.c.deserialize(r)?;
                Ok(())
            }
        }

        let mut x = Xxxx::new();
        x.a = 100;
        x.b = String::from("hello world");
        x.c = Some(0.123456f32);
        let mut buf = BufWriter::new(Vec::new());
        let _ = x.serialize(&mut buf);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: Xxxx = Xxxx::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.a, 100);
        assert_eq!(val.b, String::from("hello world"));
        assert_eq!(val.c, Some(0.123456f32));
    }

    #[test]
    fn test_serialize_deserialize_struct_with_macro() {
        #[derive(Debug)]
        struct Xxxx
        {
            a: i32,
            b: String,
            c: Option<f32>
        }

        impl Xxxx {
            fn new() -> Xxxx {
                Xxxx {
                    a: 0i32,
                    b: String::new(),
                    c: Some(0.0f32)
                }
            }
        }

        serialize_struct!(Xxxx, a, b, c);
        deserialize_struct!(Xxxx, a, b, c);

        let mut x = Xxxx::new();
        x.a = 100;
        x.b = String::from("hello world");
        x.c = Some(0.123456f32);
        let mut buf = BufWriter::new(Vec::new());
        let _ = x.serialize(&mut buf);

        let mut buf = Cursor::new(buf.buffer());
        let mut val: Xxxx = Xxxx::new();
        let _ = val.deserialize(&mut buf);
        assert_eq!(val.a, 100);
        assert_eq!(val.b, String::from("hello world"));
        assert_eq!(val.c, Some(0.123456f32));
    }
}
