use regex::Regex;
use std::collections::HashMap;

fn main() {
    let sol = solve1("input.txt");
    println!("{}", sol);

    let sol2 = solve2("input.txt");
    println!("{}", sol2);
}

fn solve1(path: &str) -> usize {
    let optional = Some(vec!["cid"]);
    read_input(path)
        .into_iter()
        .filter(|p| has_required_fields(p, optional.as_ref()))
        .count()
}

fn solve2(path: &str) -> usize {
    let optional = Some(vec!["cid"]);
    read_input(path)
        .into_iter()
        .filter(|p| is_password_valid(p, optional.as_ref()))
        .count()
}

type Password = Vec<String>;

fn read_input(path: &str) -> Vec<Password> {
    std::fs::read_to_string(path)
        .expect("failed to read file")
        .split("\n\n")
        .map(|b| {
            b.split_whitespace()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

const ALL_FIELDS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn has_required_fields(password: &Password, optional_fields: Option<&Vec<&str>>) -> bool {
    let required: Vec<&str> = match optional_fields {
        Some(optional) => ALL_FIELDS[..]
            .iter()
            .filter(|&x| !optional.contains(x))
            .copied()
            .collect(),
        None => ALL_FIELDS[..].iter().copied().collect(),
    };
    let password_fields: Vec<_> = password
        .iter()
        .map(|x| x.split(':').next().expect("should have ':'"))
        .collect();
    required.into_iter().all(|x| password_fields.contains(&x))
}

fn validate_number(value: &str, min: u32, max: u32) -> Option<bool> {
    let num = value.parse::<u32>().ok()?;
    Some(num >= min && num <= max)
}

fn validate_height(value: &str) -> Option<bool> {
    if let Some(stripped) = value.strip_suffix("cm") {
        validate_number(stripped, 150, 193)
    } else if let Some(stripped) = value.strip_suffix("in") {
        validate_number(stripped, 59, 76)
    } else {
        None
    }
}

fn validate_hair(value: &str) -> Option<bool> {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    Some(re.is_match(value))
}

fn validate_eye(value: &str) -> Option<bool> {
    const OPTIONS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    Some(OPTIONS.contains(&value))
}

fn validate_passport(value: &str) -> Option<bool> {
    let re = Regex::new(r"^\d{9}$").unwrap();
    Some(re.is_match(value))
}

#[derive(Debug)]
struct ParsedPassword {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl ParsedPassword {
    pub fn is_valid(&self) -> Option<bool> {
        if !validate_number(&self.byr, 1920, 2002)? {
            return Some(false);
        }
        if !validate_number(&self.iyr, 2010, 2020)? {
            return Some(false);
        }
        if !validate_number(&self.eyr, 2020, 2030)? {
            return Some(false);
        }
        // Validate the height
        if !validate_height(&self.hgt)? {
            return Some(false);
        }
        if !validate_hair(&self.hcl)? {
            return Some(false);
        }
        if !validate_eye(&self.ecl)? {
            return Some(false);
        }
        if !validate_passport(&self.pid)? {
            return Some(false);
        }
        Some(true)
    }
}

impl From<&Vec<String>> for ParsedPassword {
    fn from(password: &Vec<String>) -> Self {
        let mut field_map: HashMap<&str, String> = HashMap::new();
        for field in password.iter() {
            let parts: Vec<&str> = field.split(':').collect();
            assert!(parts.len() == 2);
            let (field, value) = (parts[0], parts[1]);
            field_map.insert(field, value.to_string());
        }
        ParsedPassword {
            byr: field_map.remove("byr").unwrap(),
            iyr: field_map.remove("iyr").unwrap(),
            eyr: field_map.remove("eyr").unwrap(),
            hgt: field_map.remove("hgt").unwrap(),
            hcl: field_map.remove("hcl").unwrap(),
            ecl: field_map.remove("ecl").unwrap(),
            pid: field_map.remove("pid").unwrap(),
        }
    }
}

fn is_password_valid(password: &Password, optional_fields: Option<&Vec<&str>>) -> bool {
    if !has_required_fields(password, optional_fields) {
        return false;
    }
    // Check that the fields are valid themselves
    let parsed_password = ParsedPassword::from(password);
    parsed_password.is_valid().unwrap_or(false)
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
        assert_eq!(solve2("sample.txt"), 2);
    }
}
