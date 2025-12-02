use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    dir: Direction,
    clicks: i128,
}

fn parse_rotation(line: &str) -> Option<Rotation> {
    let s = line.trim();
    if s.is_empty() {
        return None;
    }

    let (first_char, rest) = s.split_at(1);
    let dir = match first_char {
        "L" | "l" => Direction::Left,
        "R" | "r" => Direction::Right,
        _ => return None,
    };

    let clicks = rest.trim().parse::<i128>().ok()?;
    Some(Rotation { dir, clicks })
}

fn clicks_hit_zero(current: i64, rot: &Rotation) -> i128 {
    if rot.clicks == 0 {
        return 0;
    }

    let pos = current.rem_euclid(100) as i128;

    match rot.dir {
        Direction::Right => {
            // 99 -> 0
            let dist_to_zero = if pos == 99 { 1 } else { 100 - pos };
            if rot.clicks < dist_to_zero {
                0
            } else {
                1 + (rot.clicks - dist_to_zero) / 100
            }
        }
        Direction::Left => {
            // we hit 0 whenm leading exactly on it
            let dist_to_zero = if pos == 0 { 100 } else { pos };
            if rot.clicks < dist_to_zero {
                0
            } else {
                1 + (rot.clicks - dist_to_zero) / 100
            }
        }
    }
}

fn apply_rotation(current: i64, rot: &Rotation) -> i64 {
    let clicks = (rot.clicks % 100) as i64;
    // 0,1,2....49,50,51,52....99
    match rot.dir {
        Direction::Left => (current - clicks).rem_euclid(100),
        Direction::Right => (current + clicks).rem_euclid(100),
    }
}

fn solve<P: AsRef<Path>>(path: P) -> io::Result<(i128, i128)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut position: i64 = 50;
    let mut part1: i128 = 0;
    let mut part2: i128 = 0;

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() {
            continue;
        }

        if let Some(rot) = parse_rotation(&line) {
            // part 2
            part2 += clicks_hit_zero(position, &rot);
            // part 1
            position = apply_rotation(position, &rot);
            if position == 0 {
                part1 += 1;
            }
        } else {
            eprintln!("Could not parse line: {}", line)
        }
    }

    Ok((part1, part2))
}

fn main() -> io::Result<()> {
    println!("Starting....");
    let (part1, part2) = solve("input.txt")?;
    println!("Part 1 {}", part1);
    println!("Part 2 {}", part2);

    Ok(())
}
