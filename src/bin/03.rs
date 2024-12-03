use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    Some(
        input
            .split('\n')
            .filter(|el| !el.is_empty())
            .fold(0, |sum, line| {
                let mut s = 0;
                for (_, [num1_str, num2_str]) in mul_re.captures_iter(line).map(|c| c.extract()) {
                    let num1 = num1_str.parse().unwrap_or(0);
                    let num2 = num2_str.parse().unwrap_or(0);
                    s += num1 * num2;
                }
                sum + s
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"(mul|do|don't)\(([0-9]+)?,?([0-9]+)?\)").unwrap();
    let mut enabled = true;
    Some(
        input
            .split('\n')
            .filter(|el| !el.is_empty())
            .fold(0, |sum, line| {
                let mut s = 0;
                for capture in mul_re.captures_iter(line) {
                    match &capture[1] {
                        "mul" => {
                            if enabled {
                                let num1 = capture[2].parse().unwrap_or(0);
                                let num2 = capture[3].parse().unwrap_or(0);
                                s += num1 * num2;
                            }
                        }
                        "do" => enabled = true,
                        "don't" => enabled = false,
                        _ => {}
                    }
                }
                sum + s
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
