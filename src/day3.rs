use std::fs;

pub fn one() {
    let contents =
        fs::read_to_string("inputs/day3.txt").expect("Should have been able to read file");

    let rucksacks = contents.split("\n");

    let mut sum_priorities = 0;

    for sack in rucksacks {
        if sack != "" {
            let (half1, half2) = sack.split_at(sack.len() / 2);
            let chars = half1.chars();
            for char in chars {
                let yay = half2.find(char);
                if yay != None {
                    // println!("{:?}", char);
                    if char.is_lowercase() {
                        sum_priorities += char as u32 - 96;
                    } else {
                        sum_priorities += char as u32 - 38;
                    }
                    break;
                }
            }
        }
    }
    println!("{}", sum_priorities)
}

pub fn two() {
    let contents =
        fs::read_to_string("inputs/day3.txt").expect("Should have been able to read file");

    let rucksacks: Vec<&str> = contents.split("\n").collect();

    let mut sum_priorities = 0;

    for x in (0..rucksacks.len()).step_by(3) {
        let chars = rucksacks[x].chars();
        for char in chars {
            let yay1 = rucksacks[x + 1].find(char);
            let yay2 = rucksacks[x + 2].find(char);
            if yay1 != None && yay2 != None {
                if char.is_lowercase() {
                    sum_priorities += char as u32 - 96;
                } else {
                    sum_priorities += char as u32 - 38;
                }
                break;
            }
        }
        // println!("{}", x);
    }
    println!("{}", sum_priorities)
}
