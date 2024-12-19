use std::collections::{HashMap, VecDeque};
use std::{fs, time::Instant};

static FILENAME: &str = "q11/res/input.txt";

fn read_file(path: &str) -> Option<VecDeque<String>> {
    let mut out: VecDeque<String> = VecDeque::new();

    fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .for_each(|line| {
            out.push_back(line.to_string());
        });
    Some(out)
}

fn calc_number_of_stones(data: &VecDeque<String>, max_blink_count: u64) -> usize {
    let mut curr_o = data.clone();
    let mut next_o = VecDeque::new();

    for _ in 0..max_blink_count {
        for idx in 0..curr_o.len() {
            match curr_o[idx].as_ref() {
                "0" => next_o.push_back(String::from("1")),
                c => {
                    if c.len() % 2 == 0 {
                        let split = c.split_at(c.len() / 2);
                        next_o.push_back(split.0.to_string());
                        let rval = split.1.parse::<u64>().unwrap();
                        next_o.push_back(rval.to_string());
                    } else {
                        next_o.push_back((c.parse::<u64>().unwrap() * 2024).to_string());
                    }
                }
            }
        }

        curr_o = next_o.clone();
        next_o.clear();
    }

    curr_o.len()
}

fn update_map(map: &mut HashMap<String, u64>, key: &String, count: u64) {
    if let Some(v) = map.get_mut(key) {
        *v += count;
        return;
    }
    map.insert(key.to_string(), count);
}

fn calc_number_of_stones_v2(start_o: &VecDeque<String>, max_blink_count: u64) -> u64 {
    let mut prev_map: HashMap<String, u64> = HashMap::new();
    for c in start_o.iter() {
        update_map(&mut prev_map, c, 1);
    }

    let mut curr_map: HashMap<String, u64> = HashMap::new();

    for bidx in 0..max_blink_count {
        for (map_k, map_c) in prev_map.iter() {
            match map_k.as_ref() {
                "0" => {
                    update_map(&mut curr_map, &String::from("1"), *map_c);
                }
                s => {
                    if s.len() % 2 == 0 {
                        let split = s.split_at(s.len() / 2);
                        let rval = split.1.parse::<u64>().unwrap().to_string();
                        update_map(&mut curr_map, &rval, *map_c);
                        update_map(&mut curr_map, &split.0.to_string(), *map_c);
                    } else {
                        let n_v = s.parse::<u64>().unwrap() * 2024;
                        update_map(&mut curr_map, &n_v.to_string(), *map_c);
                    }
                }
            }
        }
        std::mem::swap(&mut curr_map, &mut prev_map);
        curr_map.clear();
        // println!(
        //     "Blinking: {bidx} -> {}",
        //     prev_map.values().sum::<u64>() as u64
        // );
    }

    prev_map.values().sum::<u64>() as u64
}

fn main() {
    let data = read_file(FILENAME).unwrap();

    // first half
    let start_first_half = Instant::now();
    let result_first_half = calc_number_of_stones(&data, 25);
    let duration_first_half = start_first_half.elapsed();
    println!("First half result: {}", result_first_half);
    println!("First half duration: {:?}", duration_first_half);

    // second half
    let start_second_half = Instant::now();
    let result_second_half = calc_number_of_stones_v2(&data, 75);
    let duration_second_half = start_second_half.elapsed();
    println!("Second half result: {}", result_second_half);
    println!("Second half duration: {:?}", duration_second_half);
}
