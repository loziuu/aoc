use std::{fs::File, time::Instant};

mod five;
mod four;
mod one;
mod six;
mod three;
mod two;

fn main() {
    println!("--- DAY 1 ---");
    println!("{}", one::first::execute("inputs/one"));
    elapsed(|| one::first::execute("inputs/one"));
    println!("{}", one::second::execute("inputs/one"));
    elapsed(|| one::second::execute("inputs/one"));

    println!("--- DAY 2 ---");
    println!("{}", two::first::execute(file("inputs/two")));
    elapsed(|| two::first::execute(file("inputs/two")) as i32);
    println!("{}", two::second::execute(file("inputs/two")));
    elapsed(|| two::second::execute(file("inputs/two")) as i32);

    println!("--- DAY 3 ---");
    println!("{}", three::first::execute(file("inputs/three")));
    elapsed(|| three::first::execute(file("inputs/three")) as i32);
    println!("{}", three::second::execute(file("inputs/three")));
    elapsed(|| three::second::execute(file("inputs/three")) as i32);

    println!("--- DAY 4 ---");
    println!("{}", four::first::execute(file("inputs/four")));
    elapsed(|| four::first::execute(file("inputs/four")) as i32);
    println!("{}", four::second::execute(file("inputs/four")));
    elapsed(|| four::second::execute(file("inputs/four")) as i32);

    println!("--- DAY 5 ---");
    println!("{}", five::first::execute("inputs/five"));
    elapsed(|| five::first::execute("inputs/five"));
    println!("{}", five::second::execute("inputs/five"));
    elapsed(|| five::second::execute("inputs/five"));

    println!("--- DAY 6 ---");
    println!("{}", six::first::execute("inputs/six"));
    elapsed(|| three::first::execute(file("inputs/three")) as i32);
    println!("{}", six::second::execute("inputs/six"));
    elapsed(|| three::first::execute(file("inputs/three")) as i32);
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
