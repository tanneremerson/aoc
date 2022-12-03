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


fn strategy_1() -> HashMap<String, i32> {
    return HashMap::from([
        ("A X".to_string(), 4),
        ("A Y".to_string(), 8),
        ("A Z".to_string(), 3),
        ("B X".to_string(), 1),
        ("B Y".to_string(), 5),
        ("B Z".to_string(), 9),
        ("C X".to_string(), 7),
        ("C Y".to_string(), 2),
        ("C Z".to_string(), 6),
    ]);
}

fn strategy_2() -> HashMap<String, i32> {
    return HashMap::from([
        ("A X".to_string(), 3),
        ("A Y".to_string(), 4),
        ("A Z".to_string(), 8),
        ("B X".to_string(), 1),
        ("B Y".to_string(), 5),
        ("B Z".to_string(), 9),
        ("C X".to_string(), 2),
        ("C Y".to_string(), 6),
        ("C Z".to_string(), 7),
    ]);
}

fn reader (file: &str) -> Result<BufReader<File>, Error> {
    let input = File::open(file.to_string())?;
    let reader = BufReader::new(input);
    Ok(reader)
}

fn part1 (strategy: &dyn Fn() -> HashMap<String, i32>) -> Result<i32, Error> {
    let mut my_points = 0;
    let game_to_points = strategy();

    for round in reader("02_day.txt")?.lines() {
        my_points += game_to_points.get(&round?).unwrap();
    }


    Ok(my_score)
}

fn main() -> Result<(), Error> {
    println!("Part 1 score: {}", part1(&strategy_1)?);
    println!("Part 1 score: {}", part1(&strategy_2)?);
    Ok(())
}
