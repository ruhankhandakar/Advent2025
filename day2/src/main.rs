use std::fs::read_to_string;

// Part1
fn is_exact_double(product_id: u64) -> bool {
    let string = product_id.to_string(); // 1212 -> "1212"
    let length = string.len();

    if length % 2 != 0 {
        return false;
    }
    let half = length / 2;
    return &string[..half] == &string[half..];
}
// Part2
fn is_repeated(product_id: u64) -> bool {
    let string = product_id.to_string(); // 1212 -> "1212"
    let length = string.len();

    for pattern_len in 1..length / 2 {
        if length % pattern_len != 0 {
            continue; // skip if doesn't divide evenly
        }

        let pattern = &string[..pattern_len]; // "1212", pattern_len = 2, "12", pattern_len = 3; "121"
        let repeat_count = length / pattern_len;

        if repeat_count < 2 {
            continue; // we need at least 2 repeast to invalid
        }

        let repeated_string = pattern.repeat(repeat_count);

        if repeated_string == string {
            return true;
        }
    }
    false
}

fn main() {
    let input_path = "input.txt";
    let input = read_to_string(input_path).expect("Could not read input.txt");

    let mut sum_part1 = 0u64;
    let mut sum_part2 = 0u64;

    for range in input.trim().split(",") {
        let range = range.trim();

        let (start_str, end_str) = range.split_once("-").unwrap();
        let start: u64 = start_str.parse().unwrap();
        let end: u64 = end_str.parse().unwrap();

        for product_id in start..=end {
            if is_exact_double(product_id) {
                sum_part1 += product_id
            }

            if is_repeated(product_id) {
                sum_part2 += product_id
            }
        }
    }

    println!("Part 1 result: {}", sum_part1);
    println!("Part 2 result: {}", sum_part2);
}
