use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Error};

/*
        A: Rock     1
        B: Paper    2
        C: Sciccors 3

        X: Rock     1
        Y: Paper    2
        Z: Sciccors 3

        Loss 0
        Draw 3
        Win  6
 */


fn build_game_scores() -> HashMap<String, (i32, i32)> {
    return HashMap::from([
        ("A X".to_string(), (4, 4)),
        ("A Y".to_string(), (1, 8)),
        ("A Z".to_string(), (7, 3)),
        ("B X".to_string(), (8, 1)),
        ("B Y".to_string(), (5, 5)),
        ("B Z".to_string(), (2, 9)),
        ("C X".to_string(), (3, 7)),
        ("C Y".to_string(), (9, 2)),
        ("C Z".to_string(), (6, 6)),
    ]);
}

fn reader (file: &str) -> Result<BufReader<File>, Error> {
    let input = File::open(file.to_string())?;
    let reader = BufReader::new(input);
    Ok(reader)
}

fn part1 () -> Result<i32, Error> {
    let game_scores = build_game_scores();
    let mut my_score = 0;

    for line in reader("02_day.txt")?.lines() {
        let (_theirs, mine) = game_scores.get(&line?).unwrap();
        my_score += mine;
    }


    Ok(my_score)
}

fn part2 () -> Result<i32, Error> {
    let strategy = HashMap::from([
        ("A".to_string(), ["C", "A", "B"]),
        ("B".to_string(), ["A", "B", "C"]),
        ("C".to_string(), ["B", "C", "A"])
    ]);

    let mut score = 0;

    for line in reader("02_day.txt")?.lines() {
        let play;
        let game = line?;

        let v: Vec<&str> = game.split(" ").collect();

        let (theirs, mine) = (v[0], v[1]);

        if mine == "X" {
            play = strategy.get(theirs).unwrap()[0];
        } else if mine == "Y" {
            play = strategy.get(theirs).unwrap()[1];
            score += 3;
        } else {
            play = strategy.get(theirs).unwrap()[2];
            score += 6;
        }

        if play == "A" {
            score += 1;
        } else if play == "B" {
            score += 2;
        } else {
            score += 3;
        }
    }

    Ok(score)
}

fn main() -> Result<(), Error> {
    println!("Part 1 score: {}", part1()?);
    println!("Part 1 score: {}", part2()?);
    Ok(())
}
