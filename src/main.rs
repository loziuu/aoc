use std::fs::File;

mod one;
mod three;
mod two;

fn main() {
    println!("--- DAY 1 ---");
    println!("{}", one::first::execute("inputs/one"));
    println!("{}", one::second::execute("inputs/one"));

    println!("--- DAY 2 ---");
    println!("{}", two::first::execute(File::open("inputs/two").unwrap()));
    println!(
        "{}",
        two::second::execute(File::open("inputs/two").unwrap())
    );

    println!("--- DAY 3 ---");
    println!(
        "{}",
        three::first::execute(File::open("inputs/three").unwrap())
    );
    println!(
        "{}",
        three::second::execute(File::open("inputs/three").unwrap())
    );
}
