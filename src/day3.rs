pub fn solu(){
    let mut sum = 0;
    let data: String = std::fs::read_to_string("src/data/day3data").expect("Unable to read file");
    for n in data.lines() {
        let part: Vec<i32> = n.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let mut summed: String = String::new();
        let mut values: Vec<[i32; 2]> = Vec::new();
        let mut highest: [i32; 2] = [0, 0];
        values.push(highest);

        for n in 0..12 {
            highest = [0, 0];
            let mut begin: [i32; 2] = values.last().unwrap().clone();
            if values.len() > 1 {
                begin[1] += 1;
            }
            for i in begin[1] as usize..part.len()-(11 - n) {
                if highest[0] < part[i] {
                    highest[0] = part[i];
                    highest[1] = i as i32;
                }
                if highest[0] == 9 {
                    break;
                }
            }
            values.push(highest);
        }
        values.remove(0);
        println!("Values: {:?}", values);

        for v in values {
            summed.push_str(&v[0].to_string());
        }
        println!("Summed: {}", summed);
        sum += summed.parse::<i128>().unwrap();
    }
    println!("Total sum: {}", sum);
}