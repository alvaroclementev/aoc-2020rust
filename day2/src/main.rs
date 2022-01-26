use std::fs;

fn main() {
    let sol = solve1("input.txt");
    println!("{}", sol);

    let sol2 = solve2("input.txt");
    println!("{}", sol2);
}

fn solve1(path: &str) -> usize {
    read_input(path)
        .into_iter()
        .filter(|(begin, end, letter, pwd)| validate_password1(*begin, *end, *letter, pwd))
        .count()
}

fn solve2(path: &str) -> usize {
    read_input(path)
        .into_iter()
        .filter(|(begin, end, letter, pwd)| validate_password2(*begin, *end, *letter, pwd))
        .count()
}

fn read_input(path: &str) -> Vec<(usize, usize, char, String)> {
    fs::read_to_string(path)
        .expect("Error reading file")
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(process_input)
        .collect()
}

fn process_input(line: &str) -> (usize, usize, char, String) {
    let words = line.split_whitespace().collect::<Vec<&str>>();
    assert!(words.len() == 3);

    let (range, letter, password) = (words[0], words[1], words[2]);

    // Process the range
    let nums = range
        .split("-")
        .take(2)
        .map(|x| x.parse::<usize>().expect("range elements must be numbers"))
        .collect::<Vec<usize>>();
    let (begin, end) = (nums[0], nums[1]);

    // Process the letter
    let letter: char = letter
        .chars()
        .next()
        .expect("should be a valid utf8 character");

    (begin, end, letter, password.to_string())
}

/// Check that `letter` occurs [`min`, `max`] times in `password`
fn validate_password1(min: usize, max: usize, letter: char, password: &str) -> bool {
    let count: usize = password.chars().filter(|&c| c == letter).count();
    count >= min && count <= max
}

/// Check that `letter` occurs exactly once in `min`
fn validate_password2(pos1: usize, pos2: usize, letter: char, password: &str) -> bool {
    let char1 = password.chars().nth(pos1 - 1).unwrap();
    let char2 = password.chars().nth(pos2 - 1).unwrap();

    let match1 = char1 == letter;
    let match2 = char2 == letter;

    (match1 && !match2) || (!match1 && match2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1_works() {
        assert_eq!(solve1("sample.txt"), 2);
    }

    #[test]
    fn sample_2_works() {
        assert_eq!(solve2("sample.txt"), 1);
    }
}
