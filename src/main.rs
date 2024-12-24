use std::{fs::File, time::Instant};

mod eight;
mod eighteen;
mod eleven;
mod fifteen;
mod five;
mod four;
mod fourteen;
mod nine;
mod nineteen;
mod one;
mod seven;
mod seventeen;
mod six;
mod sixteen;
mod ten;
mod thirteen;
mod three;
mod twelve;
mod twentyfour;
mod twentythree;
mod two;

fn main() {
    println!("--- DAY 24 ---");
    println!("{}", twentyfour::first::execute("inputs/twentyfour"));
    println!("{}", twentyfour::second::execute("inputs/twentyfour"));
}

fn file(name: &str) -> File {
    File::open(name).unwrap()
}

fn elapsed<T>(func: T)
where
    T: FnOnce() -> i32,
{
    let start = Instant::now();

    func();

    println!("Took: {:?}", start.elapsed());
}
