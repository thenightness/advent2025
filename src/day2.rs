pub fn solu(){
    let mut invalid = 0;
    let mut round_invalid = 0;
    let mut truth: bool;
    let data: String = std::fs::read_to_string("src/data/day2data").expect("Unable to read file");

    for n in data.split(',') {
        let begin = n.split('-').next().unwrap().parse::<i128>().unwrap();
        let end = n.split('-').nth(1).unwrap().parse::<i128>().unwrap();
        println!("- - - - - - - - - {} {}", begin, end);

        for i in begin..=end{
            let count = i.to_string().chars().count();
            for l in 1..count{
                if count % l == 0 {
                    let mut pieces: Vec<String> = Vec::new();
                    println!("Checking number: {} with length {}", i, l);
                    for p in 0..count/l {
                        let piece: String = i.to_string().chars().skip(p*l).take(l).collect();
                        pieces.push(piece);
                        println!("{} - {} - {}", count, l, pieces[0]);
                    }
                    truth = true;
                    for p in 1..pieces.len(){
                        if pieces[p] != pieces[0] {
                            truth = false;
                        }
                    }
                    if truth {
                        round_invalid = i;
                    }
                }
            }
            if round_invalid > 0 {
                invalid += round_invalid;
                round_invalid = 0;
                println!("Invalid ticket found: {}", i);
            }
        }
    }
    println!("Total invalid sum: {}", invalid);
}