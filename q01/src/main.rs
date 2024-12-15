use std::collections::HashMap;
use std::fs;

static FILENAME: &str = "q01/res/input.txt";

fn read_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    fs::read_to_string(path).unwrap().lines().for_each(|line| {
        let mut iter = line.split_whitespace();
        v1.push(iter.next().unwrap().parse().unwrap());
        v2.push(iter.next().unwrap().parse().unwrap());
    });

    (v1, v2)
}

fn create_count_hashmap(v: &Vec<i32>) -> HashMap<String, i32> {
    let mut hashmap = HashMap::new();

    v.iter().for_each(|x| {
        hashmap
            .entry(x.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    hashmap
}

fn calc_sum_distance(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> i32 {
    v1.sort();
    v2.sort();

    let mut sum: i32 = 0;

    for i in 0..v1.len() {
        sum += (v1[i] - v2[i]).abs();
    }
    sum
}

fn calc_similiarity_score(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut score = 0;
    let counts = create_count_hashmap(&v2);

    v1.iter().for_each(|x| {
        let key = x.to_string();
        match counts.get(&key) {
            Some(i) => score += x * i,
            None => (),
        }
    });

    score
}

fn main() {
    let (v1, v2) = read_file(FILENAME);

    // first half
    println!("{}", calc_sum_distance(&mut v1.clone(), &mut v2.clone()));

    // second half
    println!("{}", calc_similiarity_score(&v1, &v2));
}
