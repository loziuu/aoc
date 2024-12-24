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
mod twentythree;
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
    println!("{}", six::second::execute("inputs/six"));

    println!("--- DAY 7 ---");
    println!("{}", seven::first::execute("inputs/seven"));
    println!("{}", seven::second::execute("inputs/seven"));

    println!("--- DAY 8 ---");
    println!("{}", eight::first::execute("inputs/eight"));
    println!("{}", eight::second::execute("inputs/eight"));

    println!("--- DAY 9 ---");
    println!("{}", nine::first::execute("inputs/nine"));
    println!("{}", nine::second::execute("inputs/nine"));

    println!("--- DAY 10 ---");
    println!("{}", ten::first::execute("inputs/ten"));
    println!("{}", ten::second::execute("inputs/ten"));

    println!("--- DAY 11 ---");
    println!("{}", eleven::first::execute("inputs/eleven", 25));
    println!("{}", eleven::first::execute("inputs/eleven", 75));

    println!("--- DAY 12 ---");
    println!("{}", twelve::first::execute("inputs/twelve"));
    println!("{}", twelve::second::execute("inputs/twelve"));

    println!("--- DAY 13 ---");
    println!("{}", thirteen::first::execute("inputs/thirteen"));
    println!("{}", thirteen::second::execute("inputs/thirteen"));

    println!("--- DAY 14 ---");
    println!("{}", fourteen::first::execute("inputs/fourteen"));

    println!("--- DAY 15 ---");
    println!("{}", fifteen::first::execute("inputs/fifteen"));
    println!("{}", fifteen::second::execute("inputs/fifteen_test_2"));

    println!("--- DAY 16 ---");
    println!("{}", sixteen::first::execute("inputs/sixteen"));
    println!("{}", sixteen::second::execute("inputs/sixteen"));

    println!("--- DAY 17 ---");
    println!("{}", seventeen::first::execute("inputs/seventeen"));

    println!("--- DAY 18 ---");
    println!("{}", eighteen::first::execute("inputs/eighteen"));
    println!("{:?}", eighteen::second::execute("inputs/eighteen"));

    println!("--- DAY 19 ---");
    println!("{}", nineteen::first::execute("inputs/nineteen"));
    println!("{}", nineteen::second::execute("inputs/nineteen"));

    println!("--- DAY 23 ---");
    println!("{}", twentythree::first::execute("inputs/twentythree"));
    println!("{}", twentythree::second::execute("inputs/twentythree"));
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
