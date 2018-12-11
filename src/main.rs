mod utils;
mod day_1;

fn main() {
    let day_1 = day_1::parts(include_str!("inputs/day_1.txt"));
    println!("1-1 {}", day_1.0);
    println!("1-2 {}", day_1.1);
}
