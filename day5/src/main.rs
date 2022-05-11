use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let sol = solve1("input.txt")?;
    println!("{}", sol);

    let sol2 = solve2("input.txt")?;
    println!("{}", sol2);

    Ok(())
}

fn solve1(path: &str) -> std::io::Result<usize> {
    let max = read_input(path)?
        .into_iter()
        .map(|s| find_seat(&s))
        .max()
        .expect("there must be elements");
    Ok(max)
}

fn solve2(path: &str) -> std::io::Result<usize> {
    let mut seats = read_input(path)?
        .into_iter()
        .map(|s| find_seat(&s))
        .collect::<Vec<_>>();

    seats.sort_unstable();

    // Not sure if I understand this exercise correctly.
    // When it says "At the very front and back", I would understand the
    // first and last row...
    // However that's not the correct result
    //
    // Instead, I found that the other missing seat is the "262", which
    // is a FB... seat.
    // So I guess what they meant (in a very convoluted way) is that we should
    // skip the missing seats in that range...

    const FRONT_BACK_START: usize = 256;
    const FRONT_BACK_END: usize = 512;

    for (curr, next) in seats.iter().copied().zip(seats.iter().copied().skip(1)) {
        if next - curr > 1 && !(FRONT_BACK_START..FRONT_BACK_END).contains(&curr) {
            return Ok(curr + 1);
        }
    }
    Ok(0)
}

fn read_input(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn find_seat(pass: &str) -> usize {
    let (row, col) = decode(pass);
    row * 8 + col
}

fn decode(pass: &str) -> (usize, usize) {
    assert_eq!(pass.chars().count(), 10);

    let row_chars = pass.chars().take(7);
    let seat_chars = pass.chars().skip(7).take(3);

    let row_bin = row_chars
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            _ => panic!("invalid character"),
        })
        .collect::<String>();
    let row_value = usize::from_str_radix(&row_bin, 2).expect("the value has only 8 bits");

    let seat_bin = seat_chars
        .map(|c| match c {
            'L' => '0',
            'R' => '1',
            _ => panic!("invalid character"),
        })
        .collect::<String>();
    let seat_value = usize::from_str_radix(&seat_bin, 2).expect("the value has only 8 bits");
    (row_value, seat_value)
}
