pub fn solu1() {
    let mut safe = 50;
    let mut count = 0;
    let data: String = std::fs::read_to_string("src/data/day1data").expect("Unable to read file");
    for n in data.lines() {
        let mut iter = n.chars();
        let direction = iter.next().unwrap();
        let turn: i32 = iter.collect::<String>().parse().unwrap();

        println!("- - - - - - - - - {} {} {} {}", direction, turn, safe, count);

        if direction == 'L' {
            for i in 0..turn {
                safe -= 1;
                if safe == 0 {
                    count += 1;
                    println!("Safe hit zero! Count: {} {}", count, i+1);
                }
                if safe == -1 {
                    safe = 99;
                }
            }
            } else  {
                for i in 0..turn {
                safe += 1;
                if safe == 100 {
                    safe = 0;
                }
                if safe == 0 {
                    count += 1;
                    println!("Safe hit zero! Count: {} {}", count, i+1);
                }
            }
        }
    }
    println!("Total times safe was zero: {}", count);
}