const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn check_word(table: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut res = 0;
    for (dx, dy) in DIRECTIONS {
        'direction: for (i, c) in WORD.iter().enumerate() {
            if let Some(line) = table.get((y as isize + i as isize * dy) as usize) {
                if line.get((x as isize + i as isize * dx) as usize) != Some(c) {
                    break 'direction;
                } else if i == 3 {
                    res += 1;
                }
            } else {
                break 'direction;
            }
        }
    }
    res
}

fn check_xmas(table: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if table[y][x] != 'A' {
        return false;
    }
    let mut m_count = 0;
    let mut s_count = 0;
    for (dx, dy) in DIRECTIONS.iter().filter(|(dx, dy)| dx * dy != 0) {
        // Only get diagonals
        if let Some(line) = table.get((y as isize + dy) as usize) {
            if line.get((x as isize + dx) as usize) == Some(&'M') {
                m_count += 1;
            } else if line.get((x as isize + dx) as usize) == Some(&'S') {
                s_count += 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    m_count == 2 && s_count == 2 && table[y-1][x-1] != table[y+1][x+1]
}

pub fn part1(data: &String) -> u32 {
    let mut res = 0;
    let table = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for y in 0..table.len() {
        for x in 0..table[y].len() {
            res += check_word(&table, x, y);
        }
    }
    res
}

pub fn part2(data: &String) -> u32 {
    let mut res = 0;
    let table = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for y in 0..table.len() {
        for x in 0..table[y].len() {
            if check_xmas(&table, x, y) {
                res += 1;
            }
        }
    }
    res
}
