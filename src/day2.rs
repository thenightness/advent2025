pub fn solu(){
    let mut invalid = 0;
    let data: String = std::fs::read_to_string("src/data/day2data").expect("Unable to read file");

    for n in data.split(',') {
        let begin = n.split('-').next().unwrap().parse::<i128>().unwrap();
        let end = n.split('-').nth(1).unwrap().parse::<i128>().unwrap();
        println!("- - - - - - - - - {} {}", begin, end);

        for i in begin..=end{
            let count = i.to_string().chars().count();
            if count % 2 == 0 {
                let first_half: String = i.to_string().chars().take(count / 2).collect();
                let second_half: String = i.to_string().chars().skip(count / 2).collect();
                if first_half == second_half {
                    println!("Invalid ticket found: {}", i);
                    invalid += i;
                }
            }
        }
    }
    println!("Total invalid sum: {}", invalid);
}