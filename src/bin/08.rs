use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let char_grid: Vec<Vec<char>> = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let mut nodes = HashMap::new();
    for (i, line) in char_grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' {
                if !nodes.contains_key(c) {
                    nodes.insert(c, vec![]);
                }
                nodes.get_mut(c).unwrap().push((i, j));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for freq in nodes.keys() {
        for node in nodes.get(freq).unwrap() {
            for other in nodes.get(freq).unwrap() {
                if node != other {
                    let diff = (
                        other.0 as i32 - node.0 as i32,
                        other.1 as i32 - node.1 as i32,
                    );
                    let antinode = (other.0 as i32 + diff.0, other.1 as i32 + diff.1);
                    if 0 <= antinode.0
                        && (antinode.0 as usize) < char_grid.len()
                        && 0 <= antinode.1
                        && (antinode.1 as usize) < char_grid[0].len()
                    {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let char_grid: Vec<Vec<char>> = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let mut nodes = HashMap::new();
    for (i, line) in char_grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' {
                if !nodes.contains_key(c) {
                    nodes.insert(c, vec![]);
                }
                nodes.get_mut(c).unwrap().push((i, j));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for freq in nodes.keys() {
        for node in nodes.get(freq).unwrap() {
            for other in nodes.get(freq).unwrap() {
                if node != other {
                    let diff = (
                        other.0 as i32 - node.0 as i32,
                        other.1 as i32 - node.1 as i32,
                    );
                    let mut curr_antinode = (other.0 as i32, other.1 as i32);
                    while 0 <= curr_antinode.0
                        && (curr_antinode.0 as usize) < char_grid.len()
                        && 0 <= curr_antinode.1
                        && (curr_antinode.1 as usize) < char_grid[0].len()
                    {
                        antinodes.insert(curr_antinode);
                        curr_antinode = (curr_antinode.0 + diff.0, curr_antinode.1 + diff.1);
                    }
                    curr_antinode = (node.0 as i32, node.1 as i32);
                    while 0 <= curr_antinode.0
                        && (curr_antinode.0 as usize) < char_grid.len()
                        && 0 <= curr_antinode.1
                        && (curr_antinode.1 as usize) < char_grid[0].len()
                    {
                        antinodes.insert(curr_antinode);
                        curr_antinode = (curr_antinode.0 - diff.0, curr_antinode.1 - diff.1);
                    }
                }
            }
        }
    }

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
