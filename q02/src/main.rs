use std::fs;

static FILENAME: &str = "q02/res/sample.txt";

fn read_file(path: &str) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = Vec::new();

    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        let iter = line.split_whitespace();
        let mut v: Vec<i32> = Vec::new();
        iter.for_each(|x| {
            v.push(x.parse().unwrap());
        });
        out.push(v);
    });

    out
}

fn validate_safety(levels: &Vec<i32>) -> bool {
    if levels.len() <= 1 {
        return true;
    }

    let mut increasing: Option<bool> = None;

    for (i, v) in levels.iter().enumerate() {
        // if last item, no need to check
        if i == levels.len() - 1 {
            break;
        }

        // quick check to determine increasing/decreasing
        let next_v = &levels[i + 1];

        // check if increasing/decreasing situation is valid
        if increasing.is_none() {
            if v == next_v {
                return false;
            }
            increasing = Some(v < next_v);
        } else if let Some(true) = increasing {
            if v >= next_v {
                return false;
            }
        } else if let Some(false) = increasing {
            if v <= next_v {
                return false;
            }
        }
        // validate increase/decrease amount rule
        if (v - next_v).abs() > 3 {
            return false;
        }
    }
    true
}

fn calc_safe_report_count(levels: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    levels.iter().for_each(|x| match validate_safety(&x) {
        true => count += 1,
        false => (),
    });

    count
}

fn calc_safe_report_count_v2(levels: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    levels.iter().for_each(|x| {
        if validate_safety(&x) {
            count += 1;
        } else {
            for i in 0..x.len() {
                let mut sub_x = x.clone();
                sub_x.remove(i);
                if validate_safety(&sub_x) {
                    count += 1;
                    break;
                }
            }
        }
    });

    count
}

fn main() {
    let reports = read_file(FILENAME);

    // first half
    println!("{}", calc_safe_report_count(&reports));

    // second half
    println!("{}", calc_safe_report_count_v2(&reports));
}
