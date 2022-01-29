use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let sol = solve1("input.txt");
    println!("{}", sol);

    let sol2 = solve2("input.txt");
    println!("{}", sol2);
}

fn solve1(path: &str) -> usize {
    read_input(path).count_trees_on_path(3, 1)
}

fn solve2(path: &str) -> usize {
    let area = read_input(path);
    let directions = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    directions
        .into_iter()
        .map(|(right, down)| area.count_trees_on_path(right, down))
        .product()
}

type Point = (usize, usize);

/// Area covered by trees
struct Area {
    rows: usize,
    cols: usize,
    trees: HashSet<Point>,
}

impl Area {
    pub fn new(rows: usize, cols: usize, trees: Vec<Point>) -> Self {
        Area {
            rows,
            cols,
            trees: HashSet::from_iter(trees.into_iter()),
        }
    }

    pub fn count_trees_on_path(&self, right: usize, down: usize) -> usize {
        let mut y = 0usize;
        let mut x = 0usize;
        let mut tree_count = 0usize;
        // NOTE(alvaro): the area repeats indefinitely to the right
        while y < self.rows {
            if self.is_tree((x, y)) {
                tree_count += 1;
            }

            // Update the point coordinates
            y += down;
            x = (x + right) % self.cols;
        }
        tree_count
    }

    fn is_tree(&self, point: Point) -> bool {
        self.trees.contains(&point)
    }
}

/// Returns the locations of all the trees found in the input map
fn read_input(path: &str) -> Area {
    let input: Vec<String> = fs::read_to_string(path)
        .expect("Error reading file")
        .lines()
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.trim().to_string())
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    let trees: Vec<Point> = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    Area::new(rows, cols, trees)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_tree() {
        let area = Area::new(3, 3, vec![(2, 1)]);
        assert!(area.is_tree((2, 1)));
        assert!(!area.is_tree((1, 1)));
    }

    #[test]
    fn is_tree_sample() {
        let area = read_input("sample.txt");
        assert!(area.is_tree((2, 0)));
        assert!(area.is_tree((4, 1)));
        assert!(!area.is_tree((0, 0)));
    }

    #[test]
    fn sample_1_works() {
        assert_eq!(solve1("sample.txt"), 7);
    }

    #[test]
    fn sample_2_works() {
        assert_eq!(solve2("sample.txt"), 336);
    }
}
