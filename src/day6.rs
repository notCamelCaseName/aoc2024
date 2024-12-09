use std::collections::HashSet;

fn parse(data: &String) -> Vec<Vec<char>> {
    data.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn rotate_right((dx, dy): &(isize, isize)) -> (isize, isize) {
    (-dy, *dx)
}

pub fn part1(data: &String) -> i32 {
    let mut res = 0;
    let mut parsed_data = parse(data);
    let mut player_position: (isize, isize) = (0, 0);
    let mut player_dir = (1, 0);
    'outer: for y in 0..parsed_data.len() {
        for x in 0..parsed_data[y].len() {
            if parsed_data[y][x] == '^' {
                player_position = (x as isize, y as isize);
                player_dir = (0, -1);
                break 'outer;
            }
            if parsed_data[y][x] == 'v' {
                player_position = (x as isize, y as isize);
                player_dir = (0, 1);
                break 'outer;
            }
            if parsed_data[y][x] == '<' {
                player_position = (x as isize, y as isize);
                player_dir = (-1, 0);
                break 'outer;
            }
            if parsed_data[y][x] == '>' {
                player_position = (x as isize, y as isize);
                player_dir = (1, 0);
                break 'outer;
            }
        }
    }
    loop {
        let new_player_position = (
            player_position.0 + player_dir.0,
            player_position.1 + player_dir.1,
        );
        if !(new_player_position.0 >= 0
            && new_player_position.0 < parsed_data[0].len() as isize
            && new_player_position.1 >= 0
            && new_player_position.1 < parsed_data.len() as isize)
        {
            break;
        }
        if parsed_data[new_player_position.1 as usize][new_player_position.0 as usize] == '#' {
            player_dir = rotate_right(&player_dir);
            continue;
        } else if parsed_data[new_player_position.1 as usize][new_player_position.0 as usize] == '.'
        {
            res += 1;
        }
        parsed_data[player_position.1 as usize][player_position.0 as usize] = 'X';
        player_position = new_player_position;
    }
    res + 1
}

fn check_loop(mut parsed_data: Vec<Vec<char>>) -> bool {
    let mut player_position: (isize, isize) = (0, 0);
    let mut player_dir = (1, 0);
    'outer: for y in 0..parsed_data.len() {
        for x in 0..parsed_data[y].len() {
            if parsed_data[y][x] == '^' {
                player_position = (x as isize, y as isize);
                player_dir = (0, -1);
                break 'outer;
            }
            if parsed_data[y][x] == 'v' {
                player_position = (x as isize, y as isize);
                player_dir = (0, 1);
                break 'outer;
            }
            if parsed_data[y][x] == '<' {
                player_position = (x as isize, y as isize);
                player_dir = (-1, 0);
                break 'outer;
            }
            if parsed_data[y][x] == '>' {
                player_position = (x as isize, y as isize);
                player_dir = (1, 0);
                break 'outer;
            }
        }
    }
    let mut position_history: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
    loop {
        if parsed_data[player_position.1 as usize][player_position.0 as usize] == 'X' {
            if position_history.contains(&(player_position, player_dir)) {
                return true;
            }
        }
        let new_player_position = (
            player_position.0 + player_dir.0,
            player_position.1 + player_dir.1,
        );
        if !(new_player_position.0 >= 0
            && new_player_position.0 < parsed_data[0].len() as isize
            && new_player_position.1 >= 0
            && new_player_position.1 < parsed_data.len() as isize)
        {
            break;
        }
        if parsed_data[new_player_position.1 as usize][new_player_position.0 as usize] == '#' {
            player_dir = rotate_right(&player_dir);
            continue;
        }
        parsed_data[player_position.1 as usize][player_position.0 as usize] = 'X';
        position_history.insert((player_position, player_dir));
        player_position = new_player_position;
    }
    false
}

pub fn part2(data: &String) -> i32 {
    let parsed_data = parse(data);
    let mut res = 0;
    for y in 0..parsed_data.len() {
        for x in 0..parsed_data[y].len() {
            if parsed_data[y][x] == '.' {
                let mut new_data = parsed_data.clone();
                new_data[y][x] = '#';
                if check_loop(new_data) {
                    res += 1;
                }
            }
        }
    }
    res
}
