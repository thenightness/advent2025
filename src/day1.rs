pub fn solu1() {
    let mut safe = 50;
    let mut count = 0;
    let data: String = std::fs::read_to_string("src/data/day1data").expect("Unable to read file");
    for n in data.lines() {
        let mut iter = n.chars();
        let direction = iter.next().unwrap();
        let turn: i32 = iter.collect::<String>().parse().unwrap();

        match direction {
            'L' => safe -= turn,
            'R' => safe += turn,
            _ => println!("Invalid direction"),
        }
        while safe < 0 {
            safe += 100;
        }while safe >= 100 {
            safe -= 100;
        }
        if safe == 0 {
            count += 1;
        }
        println!("{} {} {} {}", direction, turn, safe, count);
    }
    println!("Total times safe was zero: {}", count);
}