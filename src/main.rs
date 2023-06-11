use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    ferris_says_lib_test();
    // exercice1();
    // exercice2();
}

fn exercice1() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles -= ready;

    println!("{} missiles left", missiles);
}

fn area_of(x: i32, y: i32) -> i32 {
    x*y
}

fn exercice2() {
    let width: i32 = 4;
    let height = 7;
    let depth = 10;
    let area = area_of(width, height);

    println!("Area is {}", area);

    let volume = volume_of(width, height, depth);

    println!("Volume is {}", volume);
}

fn volume_of(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}

fn ferris_says_lib_test() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
