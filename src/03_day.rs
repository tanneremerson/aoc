use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufRead, Error};

fn reader (file: &str) -> Result<BufReader<File>, Error> {
    let input = File::open(file.to_string())?;
    let reader = BufReader::new(input);
    Ok(reader)
}

static UPPER_OFFSET: u32 = 38;
static LOWER_OFFSET: u32 = 96;

fn index_items (pack: &str) -> HashSet<u32> {
   return pack
        .as_bytes()
        .to_vec()
        .into_iter()
        .map(Into::<u32>::into)
        .collect();
}

fn get_item_priority (item: &u32) -> u32 {
    if (65..91).contains(item) {
        return item - UPPER_OFFSET
    } else {
        return item - LOWER_OFFSET
    }
}


fn part1 () -> Result<u32, Error> {
    let mut total_priority = 0;

    for pack_contents in reader("03_day.txt")?.lines() {
        let pack = pack_contents?;
        let half = pack.len() / 2;

        let left = index_items(&pack[0..half]);

        for item in pack[half..].chars() {
            let i = &(item as u32);

            if left.contains(i) {
                total_priority += get_item_priority(i);
                break;
            }
        }
    }

    Ok(total_priority)
}

fn part2 () -> Result<u32, Error> {
    let mut total_priority = 0;

    let mut packs = Vec::new();

    for pack_contents in reader("03_day.txt")?.lines() {
        let pack = pack_contents?;
        packs.push(pack);
    }

    let mut idx = 0;

    while idx < packs.len() {
        let (left, middle, right) = (&packs[idx], &packs[idx + 1], &packs[idx + 2]);

        let left_contents = index_items(left);
        let middle_contents = index_items(middle);

        for item in right.chars() {
            let i = &(item as u32);
            if left_contents.contains(i) && middle_contents.contains(i) {
                total_priority += get_item_priority(i);
                break;
            }
        }

        idx += 3;
    }

    Ok(total_priority)
}

fn main() -> Result<(), Error> {
    println!("Part 1 score: {}", part1()?);
    println!("Part 1 score: {}", part2()?);
    Ok(())
}
