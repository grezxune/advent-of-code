enum Choices {
    Rock,
    Paper,
    Scissors,
    Error
}

pub fn run() {
    let input = include_str!("input.txt");
    let rounds: Vec<&str> = input.split("\n").collect();

    round_one(&rounds);
    round_two(&rounds);
}

fn round_one (rounds: &Vec<&str>) {
    let mut total: i32 = 0;

    for round in rounds.iter() {
        let choices: Vec<&str> = round.split(" ").collect();
        if choices.len() == 2 {
            total += calculate_round(match choices[0] {
                "A" => Choices::Rock,
                "B" => Choices::Paper,
                "C" => Choices::Scissors,
                _ => Choices::Error
            }, match choices[1] {
                "X" => Choices::Rock,
                "Y" => Choices::Paper,
                "Z" => Choices::Scissors,
                _ => Choices::Error
            });
        }
    }

    println!("input {}", total);
}

fn round_two (rounds: &Vec<&str>) {
    let mut total: i32 = 0;

    for round in rounds.iter() {
        let choices: Vec<&str> = round.split(" ").collect();

        if choices.len() == 2 {
            let opponent = choices[0];
            let outcome = choices[1];

            match outcome {
                "X" => total += calculate_round(match opponent {
                    "A" => Choices::Rock,
                    "B" => Choices::Paper,
                    "C" => Choices::Scissors,
                    _ => Choices::Error
                }, match opponent {
                    "A" => Choices::Scissors,
                    "B" => Choices::Rock,
                    "C" => Choices::Paper,
                    _ => Choices::Error
                }),
                "Y" => total += calculate_round(match opponent {
                    "A" => Choices::Rock,
                    "B" => Choices::Paper,
                    "C" => Choices::Scissors,
                    _ => Choices::Error
                }, match opponent {
                    "A" => Choices::Rock,
                    "B" => Choices::Paper,
                    "C" => Choices::Scissors,
                    _ => Choices::Error
                }),
                "Z" => total += calculate_round(match opponent {
                    "A" => Choices::Rock,
                    "B" => Choices::Paper,
                    "C" => Choices::Scissors,
                    _ => Choices::Error
                }, match opponent {
                    "A" => Choices::Paper,
                    "B" => Choices::Scissors,
                    "C" => Choices::Rock,
                    _ => Choices::Error
                }),
                _ => {},
            }
        }
    }

    println!("Round 2: {}", total);
}


fn calculate_round (a: Choices, b: Choices) -> i32 {
    match a {
        Choices::Rock => // Rock
            match b {
                Choices::Rock => 4, // Rock (1) + (3) -> 4
                Choices::Paper => 8, // Paper (2) + (6) -> 8
                Choices::Scissors => 3, // Scissors (3) + (0) -> 3
                _ => 0
            },
        Choices::Paper =>  // Paper
            match b {
                Choices::Rock => 1, // Rock (1) + (0) -> 1
                Choices::Paper => 5, // Paper (2) + (3) -> 5
                Choices::Scissors => 9, // Scissors (3) + (6) -> 9
                _ => 0
            },
        Choices::Scissors => // Scissors
            match b {
                Choices::Rock => 7, // Rock (1) + (6) -> 7
                Choices::Paper => 2, // Paper (2) + (0) -> 2
                Choices::Scissors => 6, // Scissors (3) + (3) -> 6
                _ => 0
            },
        _ => 0
    }
}
