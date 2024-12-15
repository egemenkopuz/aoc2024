use std::collections::{HashMap, HashSet};
use std::fs;

static FILENAME: &str = "q05/res/input.txt";

fn read_file(path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut check_rules = true;

    for line in fs::read_to_string(path).unwrap().lines() {
        if line.is_empty() {
            check_rules = false;
            continue;
        }

        if check_rules {
            let mut rule: (i32, i32) = (0, 0);
            let rule_iter = line.split_once("|");
            rule.0 = rule_iter.unwrap().0.parse().unwrap();
            rule.1 = rule_iter.unwrap().1.parse().unwrap();
            rules.push(rule);
        } else {
            let mut update: Vec<i32> = Vec::new();
            let iter = line.split(",");
            iter.for_each(|x| {
                update.push(x.parse().unwrap());
            });
            updates.push(update);
        }
    }

    (rules, updates)
}

fn gen_rules_map(rules: Vec<(i32, i32)>) -> HashMap<i32, HashSet<i32>> {
    let mut prior: HashMap<i32, HashSet<i32>> = HashMap::new();

    rules.iter().for_each(|(v1, v2)| {
        if prior.contains_key(v1) {
            prior.get_mut(v1).unwrap().insert(*v2);
        } else {
            let mut set: HashSet<i32> = HashSet::new();
            set.insert(*v2);
            prior.insert(*v1, set);
        }
    });

    prior
}

fn check(index: usize, update: &Vec<i32>, prior_map: &HashMap<i32, HashSet<i32>>) -> Option<bool> {
    let v: i32 = update[index];

    if index < update.len() - 1 {
        if let Some(pmap) = prior_map.get(&v) {
            for target_index in (index + 1)..update.len() {
                let target_v = update[target_index];
                // println!("{v} | {target_v}");
                if !pmap.contains(&target_v) {
                    // reason
                    return Some(false);
                }
            }
        } else {
            return Some(false);
        }
    }

    Some(true)
}

fn check_with_switch(
    index: usize,
    update: &mut Vec<i32>,
    prior_map: &HashMap<i32, HashSet<i32>>,
) -> Option<(bool, Vec<i32>)> {
    let v: i32 = update[index];
    if index < update.len() - 1 {
        if let Some(pmap) = prior_map.get(&v) {
            for target_index in (index + 1)..update.len() {
                let target_v = update[target_index];
                let target_pm = prior_map.get(&target_v);
                if !pmap.contains(&target_v) {
                    if target_pm.is_none() || !target_pm.unwrap().contains(&v) {
                        return Some((false, update.clone()));
                    } else {
                        update.swap(index, target_index);
                        return check_with_switch(index, update, prior_map);
                    }
                }
            }
            return Some((true, update.clone()));
        } else {
            update.swap(index, index + 1);
            return check_with_switch(index, update, prior_map);
        }
    }

    Some((true, update.clone()))
}

fn get_safe_updates(
    updates: &Vec<Vec<i32>>,
    prior_map: &HashMap<i32, HashSet<i32>>,
    allow_switch: bool,
) -> (Vec<usize>, Vec<Vec<i32>>) {
    let mut safe_update_indices: Vec<usize> = Vec::new();
    let mut m_updates: Vec<Vec<i32>> = Vec::new();

    for (update_idx, update) in updates.iter().enumerate() {
        let mut m_update = update.clone();
        for i in 0..m_update.len() {
            if allow_switch {
                match check_with_switch(i, &mut m_update, prior_map) {
                    Some((true, m_update)) => {
                        if i == m_update.len() - 1 {
                            safe_update_indices.push(update_idx);
                            m_updates.push(m_update.clone());
                        }
                    }
                    _ => break,
                }
            } else {
                match check(i, update, prior_map) {
                    Some(true) => {
                        if i == update.len() - 1 {
                            safe_update_indices.push(update_idx);
                        }
                    }
                    _ => break,
                }
            }
        }
    }

    (safe_update_indices, m_updates)
}

fn main() {
    let (rules, updates) = read_file(FILENAME);
    let prior_map = gen_rules_map(rules);

    // first half
    let (safe_updates, _) = get_safe_updates(&updates, &prior_map, false);
    let sum_mids: i32 = safe_updates
        .iter()
        .map(|&idx| {
            let update = &updates[idx];
            let mid_index = update.len() / 2;
            update[mid_index]
        })
        .sum();
    println!("{sum_mids}");

    let mut not_safe_updates: Vec<Vec<i32>> = Vec::new();
    for (idx, update) in updates.iter().enumerate() {
        if !safe_updates.contains(&idx) {
            not_safe_updates.push(update.clone());
        }
    }

    // second half
    let (_, switched_safe_updates) = get_safe_updates(&not_safe_updates, &prior_map, true);
    let sum_mids: i32 = switched_safe_updates
        .iter()
        .map(|update| {
            let mid_index = update.len() / 2;
            update[mid_index]
        })
        .sum();
    println!("{sum_mids}");
}
