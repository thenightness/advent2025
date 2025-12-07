pub fn solu(){
    let mut sum = 0;
    let data: String = std::fs::read_to_string("src/data/day3data").expect("Unable to read file");
    for n in data.lines() {
        let part: Vec<i32> = n.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let mut highest: [i32; 2] = [0, 0];
        let mut second: [i32; 2] = [0, 0];
        for n in 0..part.len()-1 {
            if highest[0] < part[n] {
                highest[0] = part[n];
                highest[1] = n as i32;
            }
            if highest[0] == 9 {
                break;
            }
        }
        for n in highest[1] as usize..part.len() {
            if second[0] < part[n] && n as i32 != highest[1] {
                second[0] = part[n];
                second[1] = n as i32;
            }
            if second[0] == 9 {
                break;
            }
        }
        sum += highest[0] * 10 + second[0]; 
        println!("{} {}", highest[0], second[0]);
    }
    println!("Total sum: {}", sum);
}