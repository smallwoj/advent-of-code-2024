use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let char_grid: Vec<Vec<char>> = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut pos = (0, 0);
    for (i, row) in char_grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                pos = (i as i32, j as i32);
            }
        }
    }
    let mut dir = (-1, 0);
    let mut visited = HashSet::new();
    while 0 <= pos.0
        && pos.0 < char_grid.len() as i32
        && 0 <= pos.1
        && pos.1 < char_grid[0].len() as i32
    {
        visited.insert(pos);
        if 0 <= pos.0 + dir.0
            && pos.0 + dir.0 < char_grid.len() as i32
            && 0 <= pos.1 + dir.1
            && pos.1 + dir.1 < char_grid[0].len() as i32
        {
            match char_grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] {
                '.' | '^' => pos = (pos.0 + dir.0, pos.1 + dir.1),
                '#' => {
                    dir = match dir {
                        (0, -1) => (-1, 0),
                        (1, 0) => (0, -1),
                        (0, 1) => (1, 0),
                        (-1, 0) => (0, 1),
                        _ => panic!("wrong"),
                    }
                }
                _ => panic!("wrong"),
            }
        } else {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_grid: Vec<Vec<char>> = input
        .split('\n')
        .filter(|el| !el.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut start_pos = (0, 0);
    for (i, row) in char_grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                start_pos = (i as i32, j as i32);
            }
        }
    }
    let mut pos = start_pos;
    let mut dir = (-1, 0);
    let mut path = HashSet::new();
    while 0 <= pos.0
        && pos.0 < char_grid.len() as i32
        && 0 <= pos.1
        && pos.1 < char_grid[0].len() as i32
    {
        path.insert(pos);
        if 0 <= pos.0 + dir.0
            && pos.0 + dir.0 < char_grid.len() as i32
            && 0 <= pos.1 + dir.1
            && pos.1 + dir.1 < char_grid[0].len() as i32
        {
            match char_grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] {
                '.' | '^' => pos = (pos.0 + dir.0, pos.1 + dir.1),
                '#' => {
                    dir = match dir {
                        (0, -1) => (-1, 0),
                        (1, 0) => (0, -1),
                        (0, 1) => (1, 0),
                        (-1, 0) => (0, 1),
                        _ => panic!("wrong"),
                    }
                }
                _ => panic!("wrong"),
            }
        } else {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }
    path.remove(&start_pos);

    let mut loops = 0;
    for obj_pos in path {
        let mut local_grid = char_grid.clone();
        local_grid[obj_pos.0 as usize][obj_pos.1 as usize] = '#';
        let mut pos = start_pos;
        let mut dir = (-1, 0);
        let mut visited = HashSet::new();
        while 0 <= pos.0
            && pos.0 < local_grid.len() as i32
            && 0 <= pos.1
            && pos.1 < local_grid[0].len() as i32
        {
            if !visited.insert((pos, dir)) {
                loops += 1;
                break;
            }
            if 0 <= pos.0 + dir.0
                && pos.0 + dir.0 < local_grid.len() as i32
                && 0 <= pos.1 + dir.1
                && pos.1 + dir.1 < local_grid[0].len() as i32
            {
                match local_grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] {
                    '.' | '^' => pos = (pos.0 + dir.0, pos.1 + dir.1),
                    '#' => {
                        dir = match dir {
                            (0, -1) => (-1, 0),
                            (1, 0) => (0, -1),
                            (0, 1) => (1, 0),
                            (-1, 0) => (0, 1),
                            _ => panic!("wrong"),
                        }
                    }
                    _ => panic!("wrong"),
                }
            } else {
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
        }
    }
    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
