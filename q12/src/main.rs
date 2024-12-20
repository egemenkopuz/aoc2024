use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

static FILENAME: &str = "q12/res/input.txt";

fn read_file(path: &str) -> Option<Vec<Vec<char>>> {
    let mut out: Vec<Vec<char>> = Vec::new();

    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        out.push(row);
    });
    Some(out)
}

#[derive(Debug)]
struct Plot {
    coord: (usize, usize),
    v: char,
    up: bool,
    right: bool,
    down: bool,
    left: bool,
    perimeter: u32,
}

impl Plot {
    fn new(x: usize, y: usize, plots: &Vec<Vec<char>>) -> Plot {
        let max_y = plots.len();
        let max_x = plots[0].len();

        let v = plots[y][x];

        // check up
        let up = if y == 0 {
            false
        } else {
            v == plots[(y - 1) as usize][x]
        };

        // check right
        let right = if x == max_x - 1 {
            false
        } else {
            v == plots[y][(x + 1) as usize]
        };

        // check down
        let down = if y == max_y - 1 {
            false
        } else {
            v == plots[(y + 1) as usize][x]
        };

        // check left
        let left = if x == 0 {
            false
        } else {
            v == plots[y][(x - 1) as usize]
        };

        let perimeter = [up, right, down, left]
            .iter()
            .filter(|&&direction| !direction)
            .count() as u32;

        Plot {
            coord: (x, y),
            v,
            up,
            right,
            down,
            left,
            perimeter,
        }
    }
}

struct PlotGroup {
    plots: Vec<Plot>,
}

fn check_neigbors(
    x: usize,
    y: usize,
    plots: &Vec<Vec<char>>,
    curr_visited: &mut Vec<(usize, usize)>,
    all_visited: &mut HashSet<(usize, usize)>,
) {
    let max_y = plots.len();
    let max_x = plots[0].len();

    // check up
    if y > 0 {
        if plots[y - 1][x] == plots[y][x] {
            if !all_visited.contains(&(x, y - 1)) {
                curr_visited.push((x, y - 1));
                all_visited.insert((x, y - 1));
                check_neigbors(x, y - 1, plots, curr_visited, all_visited);
            }
        }
    }

    // check right
    if x < max_x - 1 {
        if plots[y][x + 1] == plots[y][x] {
            if !all_visited.contains(&(x + 1, y)) {
                curr_visited.push((x + 1, y));
                all_visited.insert((x + 1, y));
                check_neigbors(x + 1, y, plots, curr_visited, all_visited);
            }
        }
    }

    // check down
    if y < max_y - 1 {
        if plots[y + 1][x] == plots[y][x] {
            if !all_visited.contains(&(x, y + 1)) {
                curr_visited.push((x, y + 1));
                all_visited.insert((x, y + 1));
                check_neigbors(x, y + 1, plots, curr_visited, all_visited);
            }
        }
    }

    // check left
    if x > 0 {
        if plots[y][x - 1] == plots[y][x] {
            if !all_visited.contains(&(x - 1, y)) {
                curr_visited.push((x - 1, y));
                all_visited.insert((x - 1, y));
                check_neigbors(x - 1, y, plots, curr_visited, all_visited);
            }
        }
    }
}

fn calc_cost(plots: &Vec<Vec<char>>, only_sides: bool) -> u32 {
    let mut out = 0;
    let mut all_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut groups: Vec<PlotGroup> = Vec::new();

    for i in 0..plots.len() {
        for j in 0..plots[i].len() {
            let mut visited: Vec<(usize, usize)> = Vec::new();

            if all_visited.contains(&(j, i)) {
                continue;
            }

            visited.push((j, i));
            all_visited.insert((j, i));

            check_neigbors(j, i, plots, &mut visited, &mut all_visited);

            if visited.len() > 0 {
                let mut plot_group = PlotGroup { plots: Vec::new() };
                for (x, y) in &visited {
                    let plot = Plot::new(*x, *y, plots);
                    plot_group.plots.push(plot);
                }
                groups.push(plot_group);
            }
        }
    }

    for group in &groups {
        if !only_sides {
            let mut tot_perimeter = 0;
            for plot in &group.plots {
                tot_perimeter += plot.perimeter;
            }
            out += group.plots.len() as u32 * tot_perimeter;
        } else {
            let mut corner_count = 0;
            let mut plots_hashmap: HashMap<(usize, usize), &Plot> = HashMap::new();

            for plot in &group.plots {
                plots_hashmap.insert(plot.coord, &plot);
            }

            for (coord, plot) in &plots_hashmap {
                // visualize_coords(plots, *coord);
                if plot.perimeter == 4 {
                    corner_count += 4;
                } else {
                    // Outer
                    if !plot.up && !plot.right {
                        corner_count += 1;
                    }
                    if !plot.right && !plot.down {
                        corner_count += 1;
                    }
                    if !plot.down && !plot.left {
                        corner_count += 1;
                    }
                    if !plot.left && !plot.up {
                        corner_count += 1;
                    }

                    // Inner
                    if plot.up && plot.right {
                        let diag = (coord.0 + 1, coord.1 - 1);
                        if !plots_hashmap.contains_key(&diag) {
                            corner_count += 1;
                        }
                    }
                    if plot.down && plot.right {
                        let diag = (coord.0 + 1, coord.1 + 1);
                        if !plots_hashmap.contains_key(&diag) {
                            corner_count += 1;
                        }
                    }
                    if plot.down && plot.left {
                        let diag = (coord.0 - 1, coord.1 + 1);
                        if !plots_hashmap.contains_key(&diag) {
                            corner_count += 1;
                        }
                    }
                    if plot.up && plot.left {
                        let diag = (coord.0 - 1, coord.1 - 1);
                        if !plots_hashmap.contains_key(&diag) {
                            corner_count += 1;
                        }
                    }
                }
            }

            // println!(
            //     "V: {} =>  Area: {} Count: {} Cost:{}",
            //     group.plots[0].v,
            //     group.plots.len(),
            //     corner_count,
            //     corner_count * group.plots.len() as u32
            // );
            out += corner_count * group.plots.len() as u32;
        }
    }

    out
}

// for debugging
fn visualize_coords(map: &Vec<Vec<char>>, coord: (usize, usize)) {
    let mut map_clone = map.clone();
    let x = coord.0;
    let y = coord.1;

    map_clone[y][x] = 'X';

    let plot = Plot::new(x, y, map);

    if plot.up {
        map_clone[y - 1][x] = 'U';
    }
    if plot.right {
        map_clone[y][x + 1] = 'R';
    }
    if plot.down {
        map_clone[y + 1][x] = 'D';
    }
    if plot.left {
        map_clone[y][x - 1] = 'L';
    }

    println!("Coord: {:?}", coord);
    for row in map_clone {
        println!("{:?}", row);
    }
}

fn main() {
    let data = read_file(FILENAME).unwrap();

    // first half
    let start_first_half = Instant::now();
    let result_first_half = calc_cost(&data, false);
    let duration_first_half = start_first_half.elapsed();
    println!("First half result: {}", result_first_half);
    println!("First half duration: {:?}", duration_first_half);

    // second half
    let start_second_half = Instant::now();
    let result_second_half = calc_cost(&data, true);
    let duration_second_half = start_second_half.elapsed();
    println!("Second half result: {}", result_second_half);
    println!("Second half duration: {:?}", duration_second_half);
}
