fn parse(data: &String) -> Vec<(isize, Vec<isize>)> {
    data.lines()
        .map(|line| {
            let split = line.find(":").unwrap();
            let expected_result = line[0..split].parse::<isize>().unwrap();
            let operands = line[split + 2..line.len()]
                .split_whitespace()
                .map(|token| token.parse::<isize>().unwrap())
                .collect::<Vec<isize>>();
            (expected_result, operands)
        })
        .collect()
}

fn try_calc(expected_result: isize, products: &Vec<isize>, mut operands: u64) -> bool {
    let mut res = products[0];
    for p in &products[1..products.len()] {
        if (operands % 2) == 0 {
            res += p;
        } else {
            res *= p;
        }
        operands >>= 1;
    }
    res == expected_result
}

fn try_calc_3_ops(expected_result: isize, products: &Vec<isize>, mut operands: u64) -> bool {
    let mut res = products[0];
    for p in &products[1..products.len()] {
        if (operands % 3) == 0 {
            res += p;
        } else if (operands % 3) == 1 {
            res *= p;
        } else {
            res = format!("{res}{p}").parse::<isize>().unwrap()
        }
        operands /= 3;
    }
    res == expected_result
}

pub fn part1(data: &String) -> i64 {
    let parsed_data = parse(data);
    parsed_data
        .iter()
        .map(|(expected_result, products)| {
            let possibilities = 2_u64.pow(products.len() as u32 - 1);
            let mut res = 0;
            for attempt in 0..possibilities {
                if try_calc(*expected_result, products, attempt) {
                    res += *expected_result as i64;
                    break;
                }
            }
            res
        })
        .sum()
}

pub fn part2(data: &String) -> i64 {
    let parsed_data = parse(data);
    parsed_data
        .iter()
        .map(|(expected_result, products)| {
            let possibilities = 3_u64.pow(products.len() as u32 - 1);
            let mut res = 0;
            for attempt in 0..possibilities {
                if try_calc_3_ops(*expected_result, products, attempt) {
                    res += *expected_result as i64;
                    break;
                }
            }
            res
        })
        .sum()
}
