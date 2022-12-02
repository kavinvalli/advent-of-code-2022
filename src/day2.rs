use std::fs;

pub fn day2() {
    let contents =
        fs::read_to_string("inputs/day2.txt").expect("Should have been able to read file");

    let lines = contents.split("\n");

    let mut player1 = 0;
    let mut player2 = 0;

    for line in lines {
        if line != "" {
            let mut inputs = line.split_whitespace();
            let inputs2: (&str, &str) = (inputs.next().unwrap(), inputs.next().unwrap());
            println!("{:?}", inputs2);

            match inputs2.0 {
                "A" => {
                    player1 += 1;
                    if inputs2.1 == "Z" {
                        player1 += 6;
                    } else if inputs2.1 == "Y" {
                        player2 += 6;
                    } else {
                        player1 += 3;
                        player2 += 3;
                    }
                }
                "B" => {
                    player1 += 2;
                    if inputs2.1 == "X" {
                        player1 += 6;
                    } else if inputs2.1 == "Z" {
                        player2 += 6;
                    } else {
                        player1 += 3;
                        player2 += 3;
                    }
                }
                "C" => {
                    player1 += 3;
                    if inputs2.1 == "Y" {
                        player1 += 6;
                    } else if inputs2.1 == "X" {
                        player2 += 6;
                    } else {
                        player1 += 3;
                        player2 += 3;
                    }
                }
                _ => println!("Ye galat hai idk"),
            }

            match inputs2.1 {
                "X" => player2 += 1,
                "Y" => player2 += 2,
                "Z" => player2 += 3,
                _ => println!("Ye galat hai idk"),
            }
        }
    }

    println!("{} - {}", player1, player2);
}

pub fn two() {
    let contents =
        fs::read_to_string("inputs/day2.txt").expect("Should have been able to read file");

    let lines = contents.split("\n");

    let mut player1 = 0;
    let mut player2 = 0;

    for line in lines {
        if line != "" {
            let mut inputs = line.split_whitespace();
            let inputs2: (&str, &str) = (inputs.next().unwrap(), inputs.next().unwrap());
            match inputs2.0 {
                "A" => {
                    // ROCK
                    player1 += 1;
                    if inputs2.1 == "X" {
                        player1 += 6;
                        player2 += 3; // SCISSOR
                    } else if inputs2.1 == "Y" {
                        player1 += 3;
                        player2 += 3;
                        player2 += 1; // ROCK
                    } else {
                        // Win by player 2 means he choses paper
                        player2 += 6 + 2;
                    }
                }
                "B" => {
                    // PAPER;
                    player1 += 2;
                    if inputs2.1 == "X" {
                        player1 += 6;
                        player2 += 1; // ROCK
                    } else if inputs2.1 == "Y" {
                        player1 += 3;
                        player2 += 3;
                        player2 += 2; // PAPER
                    } else {
                        // Win by player 2 means he choses SCISSOR
                        player2 += 6 + 3;
                    }
                }
                "C" => {
                    // SCISSOR
                    player1 += 3;
                    if inputs2.1 == "X" {
                        player1 += 6;
                        player2 += 2; // PAPER
                    } else if inputs2.1 == "Y" {
                        player1 += 3;
                        player2 += 3;
                        player2 += 3; // SCISSOR
                    } else {
                        // Win by player 2 means he choses ROCK
                        player2 += 6 + 1;
                    }
                }
                // "X" => player2 += 1,
                // "Y" => player2 += 2,
                // "Z" => player2 += 3,
                _ => println!("Ye galat hai idk"),
            }
        }
    }

    println!("{} - {}", player1, player2);
}
