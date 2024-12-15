use std::collections::HashSet;
use std::fs;

static FILENAME: &str = "q06/res/input.txt";
static DIR_DIFF: [(i32, i32); 4] = [
    (-1, 0), // up
    (0, 1),  // right
    (1, 0),  // down
    (0, -1), // left
];
static DIR_ROTATE_MAP: [usize; 4] = [1, 2, 3, 0];

fn read_file(path: &str) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();

    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        let v: Vec<char> = line.chars().collect();
        out.push(v);
    });

    out
}

fn get_start_coord(map: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == '^' {
                return Some((x as i32, y as i32));
            }
        }
    }

    None
}

fn patrol(map: &Vec<Vec<char>>, start_coord: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut out = HashSet::new();
    out.insert(start_coord);

    let x_max = map.len() as i32;
    let y_max = map[0].len() as i32;

    let mut curr_coord = start_coord;
    let mut direction = 0; // up

    loop {
        // ------- probing -------
        let mut dir_diff = DIR_DIFF[direction];
        let next_coord = (curr_coord.0 + dir_diff.0, curr_coord.1 + dir_diff.1);

        // check if it is out of map
        if next_coord.0 >= x_max || next_coord.0 < 0 || next_coord.1 >= y_max || next_coord.1 < 0 {
            break;
        }

        // get next coord's char
        match map[next_coord.0 as usize][next_coord.1 as usize] {
            '#' => {
                direction = DIR_ROTATE_MAP[direction];
            }
            _ => (),
        }

        // ------- updating -------

        dir_diff = DIR_DIFF[direction];
        curr_coord = (curr_coord.0 + dir_diff.0, curr_coord.1 + dir_diff.1);
        out.insert(curr_coord);
    }

    out
}

fn patrol_with_loop_checking(
    map: &Vec<Vec<char>>,
    start_coord: (i32, i32),
) -> (HashSet<(i32, i32)>, bool) {
    let mut out = HashSet::new();
    out.insert(start_coord);

    let x_max = map.len() as i32;
    let y_max = map[0].len() as i32;

    let mut curr_coord = start_coord;
    let mut direction = 0; // up

    let mut hit = HashSet::new();

    loop {
        let mut dir_diff = DIR_DIFF[direction];
        let next_coord = (curr_coord.0 + dir_diff.0, curr_coord.1 + dir_diff.1);

        // check if it is out of map
        if next_coord.0 >= x_max || next_coord.0 < 0 || next_coord.1 >= y_max || next_coord.1 < 0 {
            break;
        }

        let hit_data = (next_coord, direction);

        match map[next_coord.0 as usize][next_coord.1 as usize] {
            '#' => {
                if hit.contains(&hit_data) {
                    return (out, true);
                } else {
                    hit.insert(hit_data.clone());
                }
                direction = DIR_ROTATE_MAP[direction];
            }
            '0' => {
                if hit.contains(&hit_data) {
                    return (out, true);
                } else {
                    hit.insert(hit_data.clone());
                }
                direction = DIR_ROTATE_MAP[direction];
            }
            _ => {
                dir_diff = DIR_DIFF[direction];
                curr_coord = (curr_coord.0 + dir_diff.0, curr_coord.1 + dir_diff.1);
                out.insert(curr_coord);
            }
        }
    }

    (out, false)
}

fn main() {
    let map = read_file(FILENAME);

    let start_coord = get_start_coord(&map).unwrap();

    // first half
    let patrolled = patrol(&map.clone(), start_coord);
    println!("{}", patrolled.len());

    // second half
    let mut total_loops = 0;

    let mut obs_locations: HashSet<(i32, i32)> = HashSet::new();
    patrolled.iter().for_each(|x| {
        obs_locations.insert(x.clone());
    });
    obs_locations.remove(&start_coord);
    for (obs_x, obs_y) in obs_locations {
        let mut obs_map = map.clone();
        obs_map[obs_x as usize][obs_y as usize] = '0';
        match patrol_with_loop_checking(&obs_map, start_coord) {
            (_, true) => {
                total_loops += 1;
            }
            _ => {}
        }
    }
    println!("{}", total_loops);
}
