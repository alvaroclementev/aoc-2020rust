use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let result = solve1("input.txt")?;
    println!("{result}");

    let result = solve2("input.txt")?;
    println!("{result}");
    Ok(())
}

fn solve1(path: &str) -> io::Result<u64> {
    let groups = collect_groups(path)?;

    let sum = groups
        .into_iter()
        .map(|group| group.iter().flat_map(|s| s.chars()).collect::<HashSet<_>>())
        .map(|set| set.len() as u64)
        .sum();

    Ok(sum)
}

fn solve2(path: &str) -> io::Result<u64> {
    let groups = collect_groups(path)?;

    let sum = groups
        .into_iter()
        .map(|group| {
            let members = group.len();
            let mut counter: HashMap<char, usize> = HashMap::new();

            for char in group.iter().flat_map(|s| s.chars()) {
                counter.entry(char).and_modify(|e| *e += 1).or_insert(1);
            }

            // Count all the values that are equal to the number of members
            counter.values().filter(|&c| *c == members).count() as u64
        })
        .sum();

    Ok(sum)
}

fn collect_groups(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                // Start a new group
                groups.push(group);
                group = Vec::new();
            } else {
                group.push(line);
            }
        } else {
            break;
        }
    }
    // Add the last group as well
    if !group.is_empty() {
        groups.push(group);
    }
    Ok(groups)
}
