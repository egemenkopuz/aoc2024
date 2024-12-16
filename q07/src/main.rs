use itertools::Itertools;
use std::fs;

static FILENAME: &str = "q07/res/input.txt";

fn read_file(path: &str) -> Option<Vec<(u64, Vec<u64>)>> {
    let mut out: Vec<(u64, Vec<u64>)> = Vec::new();

    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        if let Some((eq_res, eq_vars)) = line.split_once(":") {
            out.push((
                eq_res.trim().parse::<u64>().unwrap(),
                eq_vars
                    .split_whitespace()
                    .map(|x| x.trim().parse::<u64>().unwrap())
                    .collect(),
            ))
        }
    });

    Some(out)
}

fn generate_perm(val_max: i32, size: usize) -> Vec<Vec<i32>> {
    let values: Vec<i32> = (0..=val_max).collect();
    let repeated_values = vec![values.clone(); size];
    repeated_values
        .into_iter()
        .multi_cartesian_product()
        .collect()
}

fn calc_eq_vars(operators: &Vec<i32>, vars: &Vec<u64>, res: u64) -> bool {
    let max = vars.len();
    let mut curr_res = vars[0];
    for (i, v) in vars.iter().skip(1).enumerate() {
        if i < max {
            match operators[i] {
                0 => curr_res += v,
                1 => curr_res *= v,
                2 => curr_res = format!("{curr_res}{v}").parse::<u64>().unwrap(),
                _ => (),
            }
        }
    }

    if curr_res == res {
        return true;
    }

    false
}

fn calc_sum_results(data: &Vec<(u64, Vec<u64>)>, max_operator_limit: i32) -> u64 {
    let mut out = 0;

    for (eq_res, eq_vars) in data {
        let operators_perms = generate_perm(max_operator_limit, eq_vars.len() - 1);
        for operators in operators_perms {
            if calc_eq_vars(&operators, &eq_vars, eq_res.clone()) {
                out += eq_res;
                break;
            }
        }
    }

    out
}

fn main() {
    let data = read_file(FILENAME).unwrap();

    // first half
    println!("{}", calc_sum_results(&data, 1));

    // second half
    println!("{}", calc_sum_results(&data, 2));
}
