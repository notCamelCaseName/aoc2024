use std::collections::VecDeque;

fn generate_disk_image(data: &String) -> Vec<isize> {
    data.trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            vec![if i % 2 == 0 { i as isize / 2 } else { -1 }; c.to_digit(10).unwrap() as usize]
        })
        .flatten()
        .collect()
}

pub fn part1(data: &String) -> usize {
    let mut disk_image = generate_disk_image(data);
    let mut i = 0;
    while i < disk_image.len() {
        if disk_image[i] == -1 {
            disk_image[i] = disk_image.pop().unwrap();
            while *disk_image.last().unwrap() == -1 {
                disk_image.pop();
            }
        }
        i += 1;
    }
    disk_image
        .iter()
        .enumerate()
        .map(|(i, n)| i * *n as usize)
        .sum()
}

pub fn part2(data: &String) -> usize {
    let mut parsed_data = data
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            (
                if i % 2 == 0 { i as isize / 2 } else { -1 },
                c.to_digit(10).unwrap() as usize,
            )
        })
        .collect::<VecDeque<(isize, usize)>>();
    let mut disk_image: Vec<isize> = Vec::new();
    'outer: while parsed_data.len() > 0 {
        let mut current_value = parsed_data.pop_front().unwrap();
        if current_value.0 == -1 {
            for i in (0..parsed_data.len()).rev() {
                if parsed_data[i].0 != -1 && parsed_data[i].1 <= current_value.1 {
                    current_value.1 -= parsed_data[i].1;
                    disk_image.append(&mut vec![
                        parsed_data[i].0 as isize;
                        parsed_data[i].1 as usize
                    ]);
                    parsed_data[i].0 = -1;
                    parsed_data.push_front(current_value);
                    continue 'outer;
                }
            }
        }
        for _ in 0..current_value.1 {
            disk_image.push(current_value.0);
        }
    }
    disk_image
        .iter()
        .enumerate()
        .map(|(i, n)| {
            if *n == -1 {
                0
            } else {
                i * *n as usize
            }
        })
        .sum()
}
