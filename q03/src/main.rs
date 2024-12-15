use std::fs;

static FILENAME: &str = "q03/res/input.txt";

fn read_file(path: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        out.push(String::from(line));
    });

    out
}

fn parse_mul_statement(s: &str) -> Option<(i32, i32)> {
    // println!("inside {}", s);

    let (s1, s2) = s.split_once(",")?;
    let v1: i32 = s1.parse().ok()?;
    let v2: i32 = s2.parse().ok()?;

    Some((v1, v2))
}

fn extract_mul_statements(s: &str) -> Vec<&str> {
    let mut stats = Vec::new();

    let mut rem = s;

    while let Some((_, after)) = rem.split_once("mul(") {
        if let Some(c_idx) = after.find(")") {
            if let Some(m_idx) = &after[..=c_idx].find("mul(") {
                rem = &after[m_idx - 1..];
            } else {
                let inside = &after[..=c_idx - 1];
                stats.push(inside);
                rem = &after[c_idx..];
            }
        } else {
            break;
        }
    }

    stats
}

fn extract_mul_statements_v2(s: &str) -> Vec<&str> {
    let mut stats = Vec::new();

    let mut rem = s;
    let mut enabled = true;

    while let Some((before, after)) = rem.split_once("mul(") {
        if let Some(c_idx) = after.find(")") {
            let do_idx = before.find("do()");
            let dont_idx = before.find("don't()");
            let m_idx = &after[..=c_idx].find("mul(");

            if do_idx.is_some() && dont_idx.is_some() {
                if do_idx.unwrap() > dont_idx.unwrap() {
                    enabled = true;
                } else {
                    enabled = false;
                }
            } else if do_idx.is_some() {
                enabled = true;
            } else if dont_idx.is_some() {
                enabled = false;
            }

            if m_idx.is_some() {
                rem = &after[m_idx.unwrap() - 1..];
            } else {
                let inside = &after[..=c_idx - 1];
                if enabled {
                    stats.push(inside);
                }
                rem = &after[c_idx + 1..];
            }
        } else {
            break;
        }
    }

    stats
}

fn calc_muls(sections: &Vec<String>, include_conditionals: bool) -> i32 {
    let mut out = 0;

    let mut merged = String::new();
    sections.iter().for_each(|x| {
        merged += x;
    });

    let statements;
    if include_conditionals {
        statements = extract_mul_statements_v2(&merged);
    } else {
        statements = extract_mul_statements(&merged);
    }

    statements.iter().for_each(|y| {
        if let Some((v1, v2)) = parse_mul_statement(&y) {
            out += v1 * v2;
        }
    });

    out
}

fn main() {
    let sections = read_file(FILENAME);

    // first half
    println!("{}", calc_muls(&sections, false));

    // second half
    println!("{}", calc_muls(&sections, true));
}
