advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    let mut total = 0;
    input
        .split('\n')
        .filter(|el| !el.is_empty())
        .for_each(|line| {
            match line.split("   ").collect::<Vec<&str>>()[..] {
                [num1, num2] => {
                    list1.push(num1.parse::<i32>().unwrap_or(0));
                    list2.push(num2.parse::<i32>().unwrap_or(0));
                }
                _ => panic!("invalid line '{}'", line),
            };
        });

    while !list1.is_empty() || !list2.is_empty() {
        let num1 = list1.clone().into_iter().min().unwrap_or(0);
        let num2 = list2.clone().into_iter().min().unwrap_or(0);
        total += (num1 - num2).abs();
        list1.remove(list1.iter().position(|&el| el == num1)?);
        list2.remove(list2.iter().position(|&el| el == num2)?);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    input
        .split('\n')
        .filter(|el| !el.is_empty())
        .for_each(|line| {
            match line.split("   ").collect::<Vec<&str>>()[..] {
                [num1, num2] => {
                    list1.push(num1.parse().unwrap_or(0));
                    list2.push(num2.parse().unwrap_or(0));
                }
                _ => panic!("invalid line '{}'", line),
            };
        });

    Some(list1.iter().fold(0, |acc, num| {
        acc + num * list2.iter().filter(|&el| el == num).count()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
