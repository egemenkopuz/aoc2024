use std::collections::HashSet;
use std::fs;

static FILENAME: &str = "q04/res/input.txt";
static DIRECTION_CHANGES: [(i32, i32); 8] = [
    (-1, 0),  // up
    (-1, 1),  // up-right
    (0, 1),   // right
    (1, 1),   // down-right
    (1, 0),   // down
    (1, -1),  // down-left
    (0, -1),  // left
    (-1, -1), // up-left
];

fn read_file(path: &str) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();

    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        let v: Vec<char> = line.chars().collect();
        out.push(v);
    });

    out
}

fn search_step(
    puzzle: &Vec<Vec<char>>,
    target_coord: (i32, i32),
    target_char: char,
) -> Option<bool> {
    let size_x = puzzle.len();
    let size_y = puzzle[0].len();

    if target_coord.0 >= size_x as i32
        || target_coord.1 >= size_y as i32
        || target_coord.0 < 0
        || target_coord.1 < 0
    {
        return None;
    }

    if puzzle[target_coord.0 as usize][target_coord.1 as usize] == target_char {
        return Some(true);
    }

    None
}

fn search(
    puzzle: &Vec<Vec<char>>,
    target_word: &str,
    start_coord: (i32, i32),
    direction: usize,
) -> Option<(bool, Vec<(i32, i32)>)> {
    let word = String::from(target_word);
    let mut found_word_count = 0;
    let mut target_char = word.chars().nth(found_word_count).unwrap();
    let mut target_coord = start_coord;

    let mut coords: Vec<(i32, i32)> = Vec::new();
    coords.push(target_coord);
    while let Some(true) = search_step(puzzle, target_coord, target_char) {
        found_word_count += 1;

        if found_word_count == word.len() {
            return Some((true, coords));
        }

        target_char = match word.chars().nth(found_word_count) {
            Some(c) => c,
            None => break,
        };

        target_coord = (
            target_coord.0 + DIRECTION_CHANGES[direction].0,
            target_coord.1 + DIRECTION_CHANGES[direction].1,
        );

        coords.push(target_coord);
    }

    Some((false, coords))
}

fn calc_search_count(puzzle: &Vec<Vec<char>>, target_word: &str, enforce_x: bool) -> i32 {
    let mut found_coords: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut dir_range = (0..8).step_by(1);
    if enforce_x {
        dir_range = (1..8).step_by(2);
    }

    for (i, row) in puzzle.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == target_word.chars().nth(0).unwrap() {
                for direction in dir_range.clone() {
                    if let Some((true, coords)) =
                        search(puzzle, target_word, (i as i32, j as i32), direction)
                    {
                        found_coords.push(coords);
                    }
                }
            }
        }
    }

    if enforce_x {
        let mut keep_coords_indices: HashSet<usize> = HashSet::new();
        // n2 search to find common A
        for (i1, c1) in found_coords.iter().enumerate() {
            for (i2, c2) in found_coords.iter().enumerate() {
                if c1 == c2 {
                    continue;
                }

                if c1[1].0 == c2[1].0 && c1[1].1 == c2[1].1 {
                    let m_diff = (c1[0].0 - c2[0].0).abs() + (c1[0].1 - c2[0].1).abs();
                    let s_diff = (c1[2].0 - c2[2].0).abs() + (c1[2].1 - c2[2].1).abs();

                    if m_diff == 1 || s_diff == 1 || m_diff == 3 || s_diff == 3 {
                        continue;
                    }

                    keep_coords_indices.insert(i1);
                    keep_coords_indices.insert(i2);
                }
            }
        }

        let mut tmp_found_coords = Vec::new();

        // only keep indices in keep_coords_indices
        for kp in keep_coords_indices {
            tmp_found_coords.push(found_coords[kp].clone());
        }

        found_coords = tmp_found_coords.clone();
    }

    // for (i, row) in puzzle.iter().enumerate() {
    //     for (j, c) in row.iter().enumerate() {
    //         let mut found = false;
    //         for coords in &found_coords {
    //             for coord in coords {
    //                 if coord.0 == i as i32 && coord.1 == j as i32 {
    //                     found = true;
    //                     break;
    //                 }
    //             }
    //         }
    //
    //         if found {
    //             //emtpy square emoji
    //             print!("{} ", c);
    //         } else {
    //             print!(". ");
    //         }
    //     }
    //     println!();
    // }
    //
    // println!();
    //
    // for row in puzzle {
    //     for c in row {
    //         print!("{} ", c);
    //     }
    //     println!(" ");
    // }

    if enforce_x {
        return found_coords.len() as i32 / 2;
    }

    found_coords.len() as i32
}

fn main() {
    let puzzle = read_file(FILENAME);

    println!("size: {}, {}", puzzle.len(), puzzle[0].len());

    // first half
    println!("{}", calc_search_count(&puzzle, "XMAS", false));
    //
    // second half
    println!("{}", calc_search_count(&puzzle, "MAS", true));
}
