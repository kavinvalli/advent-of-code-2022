use std::fs;

pub fn day1() {
    let contents =
        fs::read_to_string("inputs/day1.txt").expect("Should have been able to read file");

    let lines = contents.split("\n");

    let mut calories: Vec<i32> = Vec::new();

    let mut sum: i32 = 0;
    for line in lines {
        if line == "" {
            calories.push(sum);
            sum = 0;
        } else {
            let line: i32 = line.parse().expect("This is not a number hmmm");
            sum += line;
        }
    }

    let max = *calories.iter().max().unwrap();

    println!("{:p}, - {:?}", &calories, calories);
    println!("{max}");

    let mut calories_sorted = calories;
    calories_sorted.sort();
    calories_sorted.reverse();

    let mut top_3 = 0;
    for i in 0..3 {
        top_3 += calories_sorted[i];
    }

    println!("{:p}, - {:?}", &calories_sorted, calories_sorted);
    println!("{top_3}");
}
