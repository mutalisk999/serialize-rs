# serialize-rs


## Use serialize-rs as a crate

add dependencies to Cargo.toml

```
[dependencies]
serialize-rs = { git = "https://github.com/mutalisk999/serialize-rs.git"}
```

## Example for Serialize/DeSerialize User Define Type (Complex Struct Type)

```
#![feature(allocator_api)]

extern crate serialize_rs;

use std::io::{BufRead, Write, BufWriter, Cursor};
use std::error::Error;
use serialize_rs::{Serialize, DeSerialize};
use std::alloc::Global;

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
    fn serialize(&self, w: &mut dyn Write)-> Result<(), Box<dyn Error, Global>> {
        self.a.serialize(w)?;
        self.b.serialize(w)?;
        self.c.serialize(w)?;
        Ok(())
    }
}

impl DeSerialize for Xxxx {
    fn deserialize(&mut self, r: &mut dyn BufRead) -> Result<(), Box<dyn Error, Global>> {
        self.a.deserialize(r)?;
        self.b.deserialize(r)?;
        self.c.deserialize(r)?;
        Ok(())
    }
}

fn main() {
    let mut x = Xxxx::new();
    x.a = 100;
    x.b = String::from("hello world");
    x.c = Some(0.123456f32);
    let mut buf = BufWriter::new(Vec::new());
    let _ = x.serialize(&mut buf);

    let mut buf = Cursor::new(buf.buffer());
    let mut val: Xxxx = Xxxx::new();
    println!("{:?}", val);
    let _ = val.deserialize(&mut buf);
    println!("{:?}", val);
}

```

output print

```
Xxxx { a: 0, b: "", c: Some(0.0) }
Xxxx { a: 100, b: "hello world", c: Some(0.123456) }
```
