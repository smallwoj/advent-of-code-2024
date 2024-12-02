advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|el| !el.is_empty())
            .fold(0, |sum, line| {
                let list = line
                    .split(' ')
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<i32>>();
                let increasing = list[1] - list[0] > 0;
                for i in 1..list.len() {
                    let diff = (list[i] - list[i - 1]).abs();
                    if (1..4).contains(&diff) {
                        match (increasing, list[i] - list[i - 1] > 0) {
                            (true, false) => return sum,
                            (false, true) => return sum,
                            (_, _) => {}
                        }
                    } else {
                        return sum;
                    }
                }
                sum + 1
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|el| !el.is_empty())
            .fold(0, |sum, line| {
                let list = line
                    .split(' ')
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<i32>>();
                for remove in 0..list.len() {
                    let mut new_list = list.clone();
                    new_list.remove(remove);
                    let mut valid = true;
                    let increasing = new_list[1] - new_list[0] > 0;
                    for i in 1..new_list.len() {
                        let diff = (new_list[i] - new_list[i - 1]).abs();
                        if (1..4).contains(&diff) {
                            match (increasing, new_list[i] - new_list[i - 1] > 0) {
                                (true, false) => valid = false,
                                (false, true) => valid = false,
                                (_, _) => {}
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        return sum + 1;
                    }
                }
                sum
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
