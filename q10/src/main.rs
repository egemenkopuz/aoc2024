use std::{collections::HashSet, fs};

static FILENAME: &str = "q10/res/input.txt";

fn read_file(path: &str) -> Option<(Vec<Vec<i32>>, Vec<(i32, i32)>)> {
    let mut out: Vec<Vec<i32>> = Vec::new();
    let mut trailheads: Vec<(i32, i32)> = Vec::new();

    fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            let mut row: Vec<i32> = Vec::new();
            line.chars().enumerate().for_each(|(j, c)| match c {
                '.' => row.push(-1),
                '0' => {
                    row.push(c.to_digit(10).unwrap() as i32);
                    trailheads.push((i as i32, j as i32));
                }
                _ => row.push(c.to_digit(10).unwrap() as i32),
            });
            out.push(row);
        });
    Some((out, trailheads))
}

fn check_coord(
    data: &Vec<Vec<i32>>,
    prev_pos: Option<(i32, i32)>,
    curr_pos: (i32, i32),
    target_v: i32,
) -> Vec<(i32, i32)> {
    let mut out = Vec::new();

    let max_x = &data.len() - 1;
    let max_y = &data[max_x].len() - 1;

    if curr_pos.0 < 0 || curr_pos.0 > max_x as i32 || curr_pos.1 < 0 || curr_pos.1 > max_y as i32 {
        return out;
    }

    let mut prev_v: Option<i32> = None;
    if prev_pos.is_some() {
        prev_v = Some(data[prev_pos.unwrap().0 as usize][prev_pos.unwrap().1 as usize]);
    }
    let curr_v = data[curr_pos.0 as usize][curr_pos.1 as usize];

    if curr_v == -1 {
        return out;
    }

    if prev_pos.is_none() || (prev_v.is_some() && prev_v.unwrap() + 1 == curr_v) {
        if curr_v == target_v {
            return Vec::from([curr_pos]);
        }

        out.append(&mut check_coord(
            &data,
            Some(curr_pos),
            (curr_pos.0 - 1, curr_pos.1),
            target_v,
        ));
        out.append(&mut check_coord(
            &data,
            Some(curr_pos),
            (curr_pos.0 + 1, curr_pos.1),
            target_v,
        ));
        out.append(&mut check_coord(
            &data,
            Some(curr_pos),
            (curr_pos.0, curr_pos.1 - 1),
            target_v,
        ));
        out.append(&mut check_coord(
            &data,
            Some(curr_pos),
            (curr_pos.0, curr_pos.1 + 1),
            target_v,
        ));
    }

    out
}

fn calc_score(data: &Vec<Vec<i32>>, trailheads: &Vec<(i32, i32)>, collect_unique: bool) -> u32 {
    let mut out = 0;
    let target_v = 9;

    for (t_x, t_y) in trailheads {
        if !collect_unique {
            let results = check_coord(&data, None, (*t_x, *t_y), target_v)
                .into_iter()
                .collect::<HashSet<(i32, i32)>>();
            out += results.len() as u32;
        } else {
            let results = check_coord(&data, None, (*t_x, *t_y), target_v);
            out += results.len() as u32;
        }
    }

    out
}

fn main() {
    let (data, trailheads) = read_file(FILENAME).unwrap();

    // first half
    println!("{}", calc_score(&data, &trailheads, false));

    // second half
    println!("{}", calc_score(&data, &trailheads, true));
}
