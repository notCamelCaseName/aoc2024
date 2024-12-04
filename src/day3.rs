fn parse_mul(data: &String, index: usize) -> u32 {
    let start_index = index + 4;
    let a = data
        .chars()
        .skip(start_index)
        .take_while(|c| c.is_numeric())
        .collect::<String>();
    if data.get(start_index + a.len()..start_index + a.len() + 1) != Some(",") {
        return 0;
    }
    let b = data
        .chars()
        .skip(start_index + a.len() + 1)
        .take_while(|c| c.is_numeric())
        .collect::<String>();
    if data.get(start_index + a.len() + b.len() + 1..start_index + a.len() + b.len() + 2)
        != Some(")")
    {
        return 0;
    }
    a.parse::<u32>().expect(&format!("{}", a)) * b.parse::<u32>().expect(&format!("{}", b))
}
pub fn part1(data: &String) -> u32 {
    data.match_indices("mul(").map(|(i, _)| {parse_mul(data, i)}).sum()
}

pub fn part2(data: &String) -> u32 {
    let mut res = 0;
    let mut do_flag = true;
    for i in 0..data.len() {
        if data.get(i..i+4) == Some("mul(") && do_flag {
            res += parse_mul(data, i);
        } else if data.get(i..i+4) == Some("do()") {
            do_flag = true;
        } else if data.get(i..i+7) == Some("don't()") {
            do_flag = false;
        }
    }
    res
}
