use std::fs;

fn main() {
    let sol = solve1("input.txt").unwrap();
    println!("{}", sol);

    let sol2 = solve2("input.txt").unwrap();
    println!("{}", sol2);
}

fn solve1(path: &str) -> Option<i32> {
    let input = read_input(path);
    let combinations = naive_combinations(&input[..]);
    combinations
        .iter()
        .filter(|(x, y)| (x + y) == 2020)
        .next()
        .map(|(x, y)| x * y)
}

fn solve2(path: &str) -> Option<i32> {
    let input = read_input(path);
    let combinations = naive_combinations_triple(&input[..]);
    combinations
        .iter()
        .filter(|(x, y, z)| (x + y + z) == 2020)
        .next()
        .map(|(x, y, z)| x * y * z)
}

fn read_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("Error reading file")
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().expect("Error parsing number"))
        .collect()
}

/// For loop implementation of the unique_combinations of `input`
fn naive_combinations<T: Copy>(input: &[T]) -> Vec<(T, T)> {
    let mut result = Vec::<(T, T)>::new();
    for (i, x) in input.iter().enumerate() {
        for y in &input[i + 1..] {
            result.push((*x, *y));
        }
    }
    result
}

// TODO(alvaro): Make this version work
// /// Returns all the unique combinations of the elements in `input`
fn unique_combinations<T: Copy>(input: &[T]) -> Vec<(T, T)> {
    input
        .into_iter()
        .enumerate()
        .flat_map(|(i, x)| &input[i + 1..].into_iter().map(|y| (*x, *y)).into_iter())
        .collect()
}

/// For loop implementation of the unique_combinations_triple of `input`
fn naive_combinations_triple<T: Copy>(input: &[T]) -> Vec<(T, T, T)> {
    let mut result = Vec::<(T, T, T)>::new();
    for (i, x) in input.iter().enumerate() {
        for y in &input[i + 1..] {
            for z in &input[i + 2..] {
                result.push((*x, *y, *z));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1_works() {
        assert_eq!(solve1("sample.txt"), Some(514579));
    }

    #[test]
    fn sample_2_works() {
        assert_eq!(solve2("sample.txt"), Some(241861950));
    }
}
