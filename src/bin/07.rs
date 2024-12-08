use itertools::FoldWhile::{Continue, Done};
use itertools::{repeat_n, Itertools};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .fold(0, |sum, line| {
                let [test_num_str, numbers_str] = line.split(": ").collect_vec()[..] else {
                    panic!("wrong");
                };
                let test_num = test_num_str.parse().unwrap_or(0);
                let numbers = numbers_str
                    .split(' ')
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<_>>();
                sum + repeat_n("+*".chars(), numbers.len() - 1)
                    .multi_cartesian_product()
                    .fold_while(0, |sum, operations| {
                        let mut op_iter = operations.iter();
                        let mut result = numbers[0];
                        for i in 1..numbers.len() {
                            result = match op_iter.next() {
                                Some('+') => result + numbers[i],
                                Some('*') => result * numbers[i],
                                _ => panic!("wrong"),
                            };
                            if result > test_num {
                                Continue(sum);
                            }
                        }
                        if result == test_num {
                            Done(sum + result)
                        } else {
                            Continue(sum)
                        }
                    })
                    .into_inner()
            }),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .fold(0, |sum, line| {
                let [test_num_str, numbers_str] = line.split(": ").collect_vec()[..] else {
                    panic!("wrong");
                };
                let test_num = test_num_str.parse().unwrap_or(0);
                let numbers = numbers_str
                    .split(' ')
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<_>>();
                sum + repeat_n("+*|".chars(), numbers.len() - 1)
                    .multi_cartesian_product()
                    .fold_while(0, |sum, operations| {
                        let mut op_iter = operations.iter();
                        let mut result = numbers[0];
                        for i in 1..numbers.len() {
                            result = match op_iter.next() {
                                Some('+') => result + numbers[i],
                                Some('*') => result * numbers[i],
                                Some('|') => {
                                    format!("{}{}", result, numbers[i]).parse().unwrap_or(0)
                                }
                                _ => panic!("wrong"),
                            };
                            if result > test_num {
                                Continue(sum);
                            }
                        }
                        if result == test_num {
                            Done(sum + result)
                        } else {
                            Continue(sum)
                        }
                    })
                    .into_inner()
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
