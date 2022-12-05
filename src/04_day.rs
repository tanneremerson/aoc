use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn reader (file: &str) -> Result<BufReader<File>, Error> {
    let input = File::open(file.to_string())?;
    let reader = BufReader::new(input);
    Ok(reader)
}

fn split_to_tuple (s: &str, on: char) -> (&str, &str) {
    let v: Vec<&str> = s
        .split(on)
        .take(2)
        .collect();
    return (v[0], v[1]);
}

fn part1 () -> Result<u32, Error> {
    let mut overlap = 0;
    for line in reader("04_day.txt")?.lines() {
        let sections = line?;

        let (first_section, second_section) = split_to_tuple(&sections, ',');
        let (elf_1_left, elf_1_right) = split_to_tuple(&first_section, '-');
        let (elf_2_left, elf_2_right) = split_to_tuple(&second_section, '-');

        let range1 = elf_1_left.parse().unwrap()..(elf_1_right.parse::<u32>().unwrap() + 1);
        let range2 = elf_2_left.parse().unwrap()..(elf_2_right.parse::<u32>().unwrap() + 1);

        let (left1, right1): (u32, u32) = (
            elf_1_left.parse().unwrap(), elf_1_right.parse().unwrap()
        );
        let (left2, right2): (u32, u32) = (
            elf_2_left.parse().unwrap(), elf_2_right.parse().unwrap()
        );

        if range1.contains(&left2) && range1.contains(&right2) {
            overlap += 1;
        } else if range2.contains(&left1) && range2.contains(&right1) {
            overlap += 1;
        }
    }

    Ok(overlap)
}

fn part2 () -> Result<u32, Error> {
    let mut overlap = 0;
    for line in reader("04_day.txt")?.lines() {
        let sections = line?;

        let (first_section, second_section) = split_to_tuple(&sections, ',');
        let (elf_1_left, elf_1_right) = split_to_tuple(&first_section, '-');
        let (elf_2_left, elf_2_right) = split_to_tuple(&second_section, '-');

        let range1 = elf_1_left.parse().unwrap()..(elf_1_right.parse::<u32>().unwrap() + 1);
        let range2 = elf_2_left.parse().unwrap()..(elf_2_right.parse::<u32>().unwrap() + 1);

        let (left1, right1): (u32, u32) = (
            elf_1_left.parse().unwrap(), elf_1_right.parse().unwrap()
        );
        let (left2, right2): (u32, u32) = (
            elf_2_left.parse().unwrap(), elf_2_right.parse().unwrap()
        );

        if range1.contains(&left2) || range1.contains(&right2) {
            overlap += 1;
        } else if range2.contains(&left1) || range2.contains(&right1) {
            overlap += 1;
        }
    }

    Ok(overlap)
}

fn main() -> Result<(), Error> {
    println!("Part 1 score: {}", part1()?);
    println!("Part 2 score: {}", part2()?);
    Ok(())
}
