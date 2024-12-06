use std::fs::File;

mod five;
mod four;
mod one;
mod six;
mod three;
mod two;

fn main() {
    println!("--- DAY 1 ---");
    println!("{}", one::first::execute("inputs/one"));
    println!("{}", one::second::execute("inputs/one"));

    println!("--- DAY 2 ---");
    println!("{}", two::first::execute(file("inputs/two")));
    println!("{}", two::second::execute(file("inputs/two")));

    println!("--- DAY 3 ---");
    println!("{}", three::first::execute(file("inputs/three")));
    println!("{}", three::second::execute(file("inputs/three")));

    println!("--- DAY 4 ---");
    println!("{}", four::first::execute(file("inputs/four")));
    println!("{}", four::second::execute(file("inputs/four")));

    println!("--- DAY 5 ---");
    println!("{}", five::first::execute("inputs/five"));
    println!("{}", five::second::execute("inputs/five"));

    println!("--- DAY 6 ---");
    println!("{}", six::first::execute("inputs/six"));
}

fn file(name: &str) -> File {
    File::open(name).unwrap()
}
