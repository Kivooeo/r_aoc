mod lib;
use lib::day01::Day1;
fn main() {
    println!("Hello, world!");
    let x: Day1 = lib::day01::Day1::new();
    dbg!(x.solve_test_part_two());
    // dbg!(x.solve_input());
}
