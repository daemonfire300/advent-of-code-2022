use std::{env::args, fs::read_to_string};

fn main() {
    let path = args().nth(1).unwrap();
    println!("path: {}", path);
    let input = read_to_string(path.as_str()).unwrap();
    let mut groups: Vec<u32> = input
        .lines()
        .map(|e| e.parse::<u32>().unwrap_or_default())
        .fold(Vec::new(), |mut acc: Vec<u32>, e| {
            if acc.len() == 0 || e == 0 {
                acc.push(e);
            } else {
                if let Some(last) = acc.last_mut() {
                    *last = *last + e;
                }
            }
            acc
        });
    println!("{:?}", groups);
    groups.sort_unstable();
    groups.reverse();
    println!("{:?}", groups.clone().into_iter().max().unwrap_or_default());
    let top3 = groups.into_iter().take(3).collect::<Vec<u32>>();
    println!("{:?}", top3);
    println!("{:?}", top3.into_iter().sum::<u32>());
}
