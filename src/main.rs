use std::env;
mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Ok(day) = &args[1].parse::<i32>()else {
        println!("Please provide a day number as an argument.");
        return;
    };

    match day{
        1=>day1::solu(),
        2=>day2::solu(),
        3=>day3::solu(),
        _=>println!("Idiot!"),
    }
}
