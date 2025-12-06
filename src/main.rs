use std::env;
mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Ok(day) = &args[1].parse::<i32>()else {
        println!("Please provide a day number as an argument.");
        return;
    };

    println!("{}", day);

    match day{
        1=>day1::solu1(),
        _=>println!("Idiot!"),
    }
}
