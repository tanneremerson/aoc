use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn part1 () -> Result<i32, Error> {
    let path = "01_day.txt";

    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut cur: i32 = 0;
    let mut max: i32 = -1;

    for line in reader.lines() {
        let calories = line?;

        if calories.is_empty() {
            if cur > max {
                max = cur;
            }
            cur = 0;
        } else {
            let more: i32 = calories.parse().unwrap();
            cur += more;
        }
    }

    return Ok(max)
}

fn part2 (top_n: usize) -> Result<i32, Error> {
    let path = "01_day.txt";

    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut sum: i32 = 0;
    let mut vec = vec![];

    for line in reader.lines() {
        let calories = line?;
        if calories.is_empty() {
            vec.push(sum);
            sum = 0;
        } else {
            sum += calories.parse::<i32>().unwrap();
        }
    }

    vec.sort_unstable_by(|a, b| b.cmp(a));

    return Ok(
        vec
            .iter()
            .take(top_n)
            .fold(0, |total, next| total + next)
    )
}

fn main() -> Result<(), Error> {
    println!("The most calories carried by an elf is: {}", part1()?);

    println!("The most calories carried by an elf is: {}", part2(1)?);
    println!("The calories carried by the tope 3 elfs: {}", part2(3)?);

    Ok(())
}
