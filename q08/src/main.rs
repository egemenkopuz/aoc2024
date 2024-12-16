use std::collections::{HashMap, HashSet};
use std::fs;

static FILENAME: &str = "q08/res/input.txt";

fn read_file(path: &str) -> Option<Vec<Vec<char>>> {
    let mut out: Vec<Vec<char>> = Vec::new();

    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        let mut row: Vec<char> = Vec::new();
        line.chars().for_each(|c| row.push(c));
        out.push(row);
    });
    Some(out)
}

fn gen_unique_antinodes(data: &Vec<Vec<char>>, repeat: bool) -> HashSet<(i32, i32)> {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    let x_bound = data.len() as i32;
    let y_bound = data[0].len() as i32;

    for (i, row) in data.iter().enumerate() {
        for (j, freq) in row.iter().enumerate() {
            if *freq == '.' {
                continue;
            }
            if antennas.contains_key(freq) {
                antennas.get_mut(freq).unwrap().push((i as i32, j as i32));
            } else {
                antennas.insert(*freq, vec![(i as i32, j as i32)]);
            }
        }
    }

    for (_, locs) in antennas.iter() {
        for loc_1 in locs.iter() {
            for loc_2 in locs.iter() {
                if loc_1 == loc_2 {
                    continue;
                }

                let x_diff = loc_2.0 - loc_1.0;
                let y_diff = loc_2.1 - loc_1.1;
                let mut an = (loc_1.0 - x_diff, loc_1.1 - y_diff);

                if !(an.0 < 0 || an.0 >= x_bound || an.1 < 0 || an.1 >= y_bound) {
                    antinodes.insert(an.clone());
                }
                if !repeat {
                    continue;
                }
                loop {
                    an = (an.0 + x_diff, an.1 + y_diff);
                    if an.0 < 0 || an.0 >= x_bound || an.1 < 0 || an.1 >= y_bound {
                        break;
                    }
                    antinodes.insert(an.clone());
                }
            }
        }
    }

    antinodes
}

fn main() {
    // first half
    let mut data = read_file(FILENAME).unwrap();
    let antinodes = gen_unique_antinodes(&data, false);
    println!("{}", antinodes.len());

    // for an in antinodes.iter() {
    //     data[an.0 as usize][an.1 as usize] = '#';
    // }
    //
    // for row in data.iter() {
    //     println!("{:?}", row);
    // }

    // second half
    data = read_file(FILENAME).unwrap();
    let antinodes = gen_unique_antinodes(&data, true);
    println!("{}", antinodes.len());

    // for an in antinodes.iter() {
    //     data[an.0 as usize][an.1 as usize] = '#';
    // }
    //
    // for row in data.iter() {
    //     println!("{:?}", row);
    // }
}
