use std::fmt;
use std::io::Error;

type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

type Result<T> = std::result::Result<T, std::io::Error>;

fn takes_long_type(f: Thunk) {

}

// fn returns_long_type() -> Thunk {

// }

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
}
