use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct RockPaperScissors {
    game_number: u32,
    raw: String,
    opponent_choice: char,
    my_choice: char,
}

// this is really verbose, but it works
impl RockPaperScissors {
    fn outcome_score(&self) -> i32 {
        // win = 6, draw = 3, lose = 0
        if self.my_choice == self.opponent_choice {
            3
        } else if self.my_choice == 'R' && self.opponent_choice == 'S' {
            6
        } else if self.my_choice == 'S' && self.opponent_choice == 'P' {
            6
        } else if self.my_choice == 'P' && self.opponent_choice == 'R' {
            6
        } else {
            0
        }
    }

    fn shape_score(&self) -> i32 {
        // shape score = (1 for Rock, 2 for Paper, and 3 for Scissors)
        if self.my_choice == 'R' {
            1
        } else if self.my_choice == 'P' {
            2
        } else {
            3
        }
    }

    fn round_score(&self) -> i32 {
        // single round score = (outcome_score) + (shape score)
        self.outcome_score() + self.shape_score()
    }
}

// this is a translation layer to work out from 
// (A or B or C) (X or Y or Z) what the right response should be
fn translate_outcome_needed_to_my_choice(opponent_choice: &char, outcome_needed: &char) -> char {
    if *opponent_choice == 'A' {
        // opponent = 'R'
        if *outcome_needed == 'X' {
            // lose
            'S'
        } else if *outcome_needed == 'Y' {
            // draw
            'R'
        } else {
            // win
            'P'
        }
    } else if *opponent_choice == 'B' {
        // Opponent = 'P'
        if *outcome_needed == 'X' {
            // lose
            'R'
        } else if *outcome_needed == 'Y' {
            // draw
            'P'
        } else {
            // win
            'S'
        }
    } else {
        // Opponent = 'S'
        if *outcome_needed == 'X' {
            // lose
            'P'
        } else if *outcome_needed == 'Y' {
            // draw
            'S'
        } else {
            // win
            'R'
        }
    }
}

fn translate_choice(c: &char) -> char {
    if *c == 'A' {
        'R'
    // } else if *c == 'X' {
    //     'R'
    } else if *c == 'B' {
        'P'
    } else if *c == 'C' {
        'S'
    // } else if *c == 'Y' {
    //     'P'
    } else {
        'Q' // this shouldn't happen
    }
}

fn main() {
    println!("Advent of code 2022, Day 2");

    // vector of RockPaperScissors
    let mut games: Vec<RockPaperScissors> = Vec::new();
    let mut i: u32 = 1;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let oc = line.as_ref().unwrap().chars().nth(0).unwrap();
            let outcome_needed = line.as_ref().unwrap().chars().nth(2).unwrap();

            games.push(RockPaperScissors {
                game_number: i,
                raw: line.unwrap(),
                opponent_choice: translate_choice(&oc),
                my_choice: translate_outcome_needed_to_my_choice(&oc, &outcome_needed),
            });

            i += 1;
        }
    }

    let mut total_score: i32 = 0;
    for game in games {
        println!(
            "Game[{:03}]: {:?} = {}",
            game.game_number,
            game,
            game.round_score()
        );
        total_score += game.round_score();
    }
    println!("Total score: {}", total_score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
