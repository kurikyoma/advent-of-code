use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Hand {
    rock,
    paper,
    scissors,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Outcome {
    win,
    lose,
    tie,
}

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day2\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score: u32 = 0;
    let mut total_score2: u32 = 0;

    for line in contents.split("\n") {
        let me_hand = get_type(line.trim().chars().last().unwrap());
        let me_strategy = get_strategy(line.trim().chars().last().unwrap());
        let them = get_type(line.trim().chars().nth(0).unwrap());

        let score = get_score(me_hand, get_outcome(them, me_hand));
        total_score += score;
        
        let score2 = get_score(get_hand(them, me_strategy), me_strategy);
        total_score2 += score2;
        println!("My hand {:?}, their hand {:?}, score {}.", me_hand, them, score);
    }

    println!("Final Score using guide as hands: {}", total_score);
    println!("Final Score using guide as outcomes: {}", total_score2);
}

fn get_type(letter: char) -> Hand {
    use Hand::*;
    return match letter {
        'A' | 'X' => rock,
        'B' | 'Y' => paper,
        'C' | 'Z' => scissors,
        _ => panic!("Unknown move: {}.", letter),
    };
}

fn get_strategy(letter: char) -> Outcome {
    use Outcome::*;
    return match letter {
        'X' => lose,
        'Y' => tie,
        'Z' => win,
        _ => panic!("Unknown strategy: {}.", letter),
    };
}

fn get_outcome(them: Hand, me: Hand) -> Outcome {
    use Hand::*;
    use Outcome::*;
    if me == rock {
        return match them {
            rock => tie,
            paper => lose,
            scissors => win,
        };
    }
    else if me == paper {
        return match them {
            rock => win,
            paper => tie,
            scissors => lose,
        };
    }
    else if me == scissors {
        return match them {
            rock => lose,
            paper => win,
            scissors => tie,
        };
    }
    else {
        panic!("WHAT AM I????");
    }

}

fn get_hand(them: Hand, o: Outcome) -> Hand {
    use Hand::*;
    use Outcome::*;
    if o == lose {
        return match them {
            rock => scissors,
            paper => rock,
            scissors => paper,
        };
    }
    else if o == tie {
        return match them {
            rock => rock,
            paper => paper,
            scissors => scissors,
        };
    }
    else if o == win {
        return match them {
            rock => paper,
            paper => scissors,
            scissors => rock,
        };
    }
    else {
        panic!("WHAT AM I????");
    }
}

fn get_score(h: Hand, o: Outcome) -> u32 {
    use Hand::*;
    use Outcome::*;
    let mut score: u32 = match h {
        rock => 1,
        paper => 2,
        scissors => 3,
    };

    score += match o {
        win => 6,
        lose => 0,
        tie => 3,
    };

    return score;
}
