use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let [rules_str, pages_str] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        panic!("invalid fmt");
    };
    let rules = rules_str
        .split('\n')
        .filter(|el| !el.is_empty())
        .map(|rule_str| {
            let [num1, num2] = rule_str
                .split('|')
                .map(|num| num.parse().unwrap_or(0))
                .collect::<Vec<u32>>()[..]
            else {
                panic!("invalid fmt");
            };
            (num1, num2)
        })
        .collect::<Vec<(u32, u32)>>();
    Some(
        pages_str
            .split('\n')
            .filter(|el| !el.is_empty())
            .fold(0, |sum, line| {
                let sequence = line
                    .split(',')
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<u32>>();
                let mut valid = true;
                for i in 0..sequence.len() {
                    if !rules
                        .iter()
                        .filter(|(num, _)| *num == sequence[i])
                        .all(|(_, num)| {
                            if let Some(j) = sequence.iter().position(|new_num| num == new_num) {
                                i < j
                            } else {
                                true
                            }
                        })
                    {
                        valid = false;
                    }
                }
                if valid {
                    sum + sequence[sequence.len() / 2]
                } else {
                    sum
                }
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let [rules_str, pages_str] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        panic!("invalid fmt");
    };
    let rules = rules_str
        .split('\n')
        .filter(|el| !el.is_empty())
        .map(|rule_str| {
            let [num1, num2] = rule_str
                .split('|')
                .map(|num| num.parse().unwrap_or(0))
                .collect::<Vec<u32>>()[..]
            else {
                panic!("invalid fmt");
            };
            (num1, num2)
        })
        .collect::<Vec<(u32, u32)>>();
    Some(
        pages_str
            .split('\n')
            .filter(|el| !el.is_empty())
            .map(|line| {
                line.split(',')
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<u32>>()
            })
            .filter(|sequence| {
                let mut valid = true;
                for i in 0..sequence.len() {
                    if !rules
                        .iter()
                        .filter(|(num, _)| *num == sequence[i])
                        .all(|(_, num)| {
                            if let Some(j) = sequence.iter().position(|new_num| num == new_num) {
                                i < j
                            } else {
                                true
                            }
                        })
                    {
                        valid = false;
                    }
                }
                !valid
            })
            .map(|sequence| {
                sequence
                    .into_iter()
                    .sorted_by(|a, b| {
                        match rules.iter().find(|(num1, num2)| num1 == a && num2 == b) {
                            Some(_) => Ordering::Less,
                            None => {
                                match rules.iter().find(|(num1, num2)| num1 == b && num2 == a) {
                                    Some(_) => Ordering::Greater,
                                    None => Ordering::Equal,
                                }
                            }
                        }
                    })
                    .collect::<Vec<u32>>()
            })
            .fold(0, |sum, sequence| sum + sequence[sequence.len() / 2]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
