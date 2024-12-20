use std::{fs, time::Instant};

static FILENAME: &str = "q13/res/input.txt";

#[derive(Debug, Clone)]
struct ClawDetails {
    a_x: u128,
    a_y: u128,
    b_x: u128,
    b_y: u128,
    p_x: u128,
    p_y: u128,
}

fn read_file(path: &str, part2: bool) -> Option<Vec<ClawDetails>> {
    let mut out: Vec<ClawDetails> = Vec::new();

    let re_xy = regex::Regex::new(r"X(?P<x>[+-]\d+), Y(?P<y>[+-]\d+)").unwrap();
    let re_prize = regex::Regex::new(r"X=(?P<x>\d+), Y=(?P<y>\d+)").unwrap();

    let mut claw_row_idx = 0;
    let mut claw = ClawDetails {
        a_x: 0,
        a_y: 0,
        b_x: 0,
        b_y: 0,
        p_x: 0,
        p_y: 0,
    };
    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        if line == "" {
            claw_row_idx = 0;
            claw = ClawDetails {
                a_x: 0,
                a_y: 0,
                b_x: 0,
                b_y: 0,
                p_x: 0,
                p_y: 0,
            };
        } else {
            if claw_row_idx == 0 {
                if let Some(caps) = re_xy.captures(line) {
                    claw.a_x = caps["x"].parse::<u128>().unwrap();
                    claw.a_y = caps["y"].parse::<u128>().unwrap();
                }
                claw_row_idx += 1;
            } else if claw_row_idx == 1 {
                if let Some(caps) = re_xy.captures(line) {
                    claw.b_x = caps["x"].parse::<u128>().unwrap();
                    claw.b_y = caps["y"].parse::<u128>().unwrap();
                }
                claw_row_idx += 1;
            } else {
                if let Some(caps) = re_prize.captures(line) {
                    if part2 {
                        claw.p_x = caps["x"].parse::<u128>().unwrap() + 10000000000000;
                        claw.p_y = caps["y"].parse::<u128>().unwrap() + 10000000000000;
                    } else {
                        claw.p_x = caps["x"].parse::<u128>().unwrap();
                        claw.p_y = caps["y"].parse::<u128>().unwrap();
                    }
                }
                claw_row_idx += 1;
                out.push(claw.clone());
            }
        }
    });

    Some(out)
}

fn solve(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Option<(f64, f64)> {
    let det = a * e - b * d;

    // no unique solution
    if det == 0.0 {
        return None;
    }

    // Cramer's rule
    let x = (c * e - b * f) / det;
    let y = (a * f - c * d) / det;

    Some((x, y))
}

fn calc(data: &Vec<ClawDetails>) -> i64 {
    let mut out = 0;

    for claw in data {
        if let Some(ans) = solve_system_of_equations(
            claw.a_x as f64,
            claw.b_x as f64,
            claw.p_x as f64,
            claw.a_y as f64,
            claw.b_y as f64,
            claw.p_y as f64,
        ) {
            let (x, y) = ans;
            if x >= 0.0 && y >= 0.0 && x.fract() == 0.0 && y.fract() == 0.0 {
                out += x as i64 * 3 + y as i64 * 1;
            }
        }
    }

    out
}

fn main() {
    let data = read_file(FILENAME, false).unwrap();

    // first half
    let start_first_half = Instant::now();
    let result_first_half = calc(&data);
    let duration_first_half = start_first_half.elapsed();
    println!("First half result: {}", result_first_half);
    println!("First half duration: {:?}", duration_first_half);

    let data = read_file(FILENAME, true).unwrap();
    // second half
    let start_second_half = Instant::now();
    let result_second_half = calc(&data);
    let duration_second_half = start_second_half.elapsed();
    println!("Second half result: {}", result_second_half);
    println!("Second half duration: {:?}", duration_second_half);
}
