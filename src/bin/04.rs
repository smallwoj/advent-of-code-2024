advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let char_grid: Vec<Vec<char>> = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let mut num = 0;
    for i in 0..char_grid.len() {
        for j in 0..char_grid[i].len() {
            let sequence = "XMAS";
            let mut stack;
            let mut valid;
            // top check
            stack = sequence.chars().peekable();
            valid = true;
            for k in j..(j + 4) {
                if (0..char_grid.len()).contains(&i) && (0..char_grid[i].len()).contains(&k) {
                    if Some(char_grid[i][k]) == stack.peek().copied() {
                        stack.next();
                    } else {
                        valid = false;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                num += 1;
            }
            // top-right check
            stack = sequence.chars().peekable();
            valid = true;
            for k in 0..4 {
                if (0..char_grid.len()).contains(&(i + k))
                    && (0..char_grid[i].len()).contains(&(j + k))
                {
                    if Some(char_grid[i + k][j + k]) == stack.peek().copied() {
                        stack.next();
                    } else {
                        valid = false;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                num += 1;
            }
            // right check
            stack = sequence.chars().peekable();
            valid = true;
            for k in 0..4 {
                if (0..char_grid.len()).contains(&(i + k)) && (0..char_grid[i].len()).contains(&(j))
                {
                    if Some(char_grid[i + k][j]) == stack.peek().copied() {
                        stack.next();
                    } else {
                        valid = false;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                num += 1;
            }
            // bottom-right check
            stack = sequence.chars().peekable();
            valid = true;
            for k in 0..4 {
                if (0..char_grid.len()).contains(&(i + k))
                    && (0..char_grid[i].len() as i32).contains(&(j as i32 - k as i32))
                {
                    if Some(char_grid[i + k][j - k]) == stack.peek().copied() {
                        stack.next();
                    } else {
                        valid = false;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                num += 1;
            }
            // bottom check
            stack = sequence.chars().peekable();
            valid = true;
            for k in 0..4 {
                if (0..char_grid.len()).contains(&(i))
                    && (0..char_grid[i].len() as i32).contains(&(j as i32 - k as i32))
                {
                    if Some(char_grid[i][j - k]) == stack.peek().copied() {
                        stack.next();
                    } else {
                        valid = false;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                num += 1;
            }
            // bottom-left check
            stack = sequence.chars().peekable();
            valid = true;
            for k in 0..4 {
                if (0..char_grid.len() as i32).contains(&(i as i32 - k as i32))
                    && (0..char_grid[i].len() as i32).contains(&(j as i32 - k as i32))
                {
                    if Some(char_grid[i - k][j - k]) == stack.peek().copied() {
                        stack.next();
                    } else {
                        valid = false;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                num += 1;
            }
            // left check
            stack = sequence.chars().peekable();
            valid = true;
            for k in 0..4 {
                if (0..char_grid.len() as i32).contains(&(i as i32 - k as i32))
                    && (0..char_grid[i].len()).contains(&(j))
                {
                    if Some(char_grid[i - k][j]) == stack.peek().copied() {
                        stack.next();
                    } else {
                        valid = false;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                num += 1;
            }
            // top-left check
            stack = sequence.chars().peekable();
            valid = true;
            for k in 0..4 {
                if (0..char_grid.len() as i32).contains(&(i as i32 - k as i32))
                    && (0..char_grid[i].len()).contains(&(j + k))
                {
                    if Some(char_grid[i - k][j + k]) == stack.peek().copied() {
                        stack.next();
                    } else {
                        valid = false;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                num += 1;
            }
        }
    }
    Some(num)
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_grid: Vec<Vec<char>> = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let mut num = 0;
    for i in 1..(char_grid.len() - 1) {
        for j in 1..(char_grid[i].len() - 1) {
            let mut valid = 0;
            if char_grid[i][j] == 'A' {
                // top-right start check
                if char_grid[i + 1][j + 1] == 'M' && char_grid[i - 1][j - 1] == 'S' {
                    valid += 1;
                }
                // bottom-right check
                if char_grid[i + 1][j - 1] == 'M' && char_grid[i - 1][j + 1] == 'S' {
                    valid += 1;
                }
                // bottom-left check
                if char_grid[i - 1][j - 1] == 'M' && char_grid[i + 1][j + 1] == 'S' {
                    valid += 1;
                }
                // top-left check
                if char_grid[i - 1][j + 1] == 'M' && char_grid[i + 1][j - 1] == 'S' {
                    valid += 1;
                }
                if valid == 2 {
                    num += 1;
                }
            }
        }
    }
    Some(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
