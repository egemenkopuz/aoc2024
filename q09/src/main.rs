use std::{borrow::BorrowMut, fs};

static FILENAME: &str = "q09/res/input.txt";

fn gen_disk_map(data: &str) -> Vec<i64> {
    let mut out = Vec::new();

    let mut mem_seg_idx: i64 = 0;
    for (i, char_v) in data.chars().enumerate() {
        if char_v == ' ' || char_v == '\n' {
            continue;
        }
        let block_count = char_v.to_digit(10).unwrap();
        if i % 2 == 0 {
            let mut v = vec![mem_seg_idx; block_count as usize];
            out.append(v.borrow_mut());
            mem_seg_idx += 1;
        } else {
            out.append(&mut vec![-1; block_count as usize]);
        }
    }

    out
}

fn convert_compact_version(disk_map: &Vec<i64>) -> Vec<i64> {
    let mut out = Vec::new();

    let mut upper_limit = disk_map.len();
    for i in 0..disk_map.len() {
        if i >= upper_limit {
            return out;
        }
        // not empty
        if disk_map[i] != -1 {
            // println!("not empty, adding {:?}", disk_map[i]);
            out.push(disk_map[i]);
            continue;
        }

        // empty, replace with last
        for j in (i..upper_limit).rev() {
            // println!("checking j: {:?}", j);
            if disk_map[j] != -1 {
                // println!("empty, adding {:?}", disk_map[j]);
                out.push(disk_map[j]);
                upper_limit = j;
                break;
            }
            upper_limit = j;
        }
    }

    out
}

fn convert_compact_version_v2(disk_map: &mut Vec<i64>) -> &Vec<i64> {
    let mut ridx = &disk_map.len() - 1;

    while 0 < ridx {
        let curr_seg_idx = disk_map[ridx];
        if curr_seg_idx == -1 {
            ridx -= 1;
            continue;
        }

        let rem_seg = (0..ridx)
            .rev()
            .take_while(|&j| disk_map[j] == curr_seg_idx)
            .count()
            + 1;

        let mut lidx = 0;
        while lidx + rem_seg <= ridx {
            let tar_seg_idx = disk_map[lidx];

            if tar_seg_idx != -1 {
                lidx += 1;
                continue;
            }

            let rem_eseg = (lidx..=ridx - rem_seg)
                .take_while(|&j| disk_map[j] == -1)
                .count();

            if rem_eseg >= rem_seg {
                // replace empty seg
                for _ in 0..rem_seg {
                    disk_map[lidx] = curr_seg_idx;
                    lidx += 1;
                }
                // remove target seg
                let mut tmp_ridx = ridx;
                for _ in 0..rem_seg {
                    disk_map[tmp_ridx] = -1;
                    tmp_ridx -= 1;
                }
                break;
            }

            lidx += 1;
        }

        if ridx < rem_seg {
            break;
        }

        ridx -= rem_seg;
    }
    disk_map
}

fn main() {
    let data = fs::read_to_string(FILENAME).unwrap();

    let mut disk_map = gen_disk_map(&data);

    // first half
    let compact = convert_compact_version(&disk_map.clone());
    println!(
        "{:?}",
        compact
            .iter()
            .enumerate()
            .map(|(i, &x)| { i as i64 * x })
            .sum::<i64>()
    );

    // second half
    let compact = convert_compact_version_v2(&mut disk_map);
    println!(
        "{:?}",
        compact
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                if x != -1 {
                    i as i64 * x
                } else {
                    0
                }
            })
            .sum::<i64>()
    );
}
