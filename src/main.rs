use std::fs;
use std::u32;

struct Lists {left: Vec<u32>, right: Vec<u32>}

fn main() {
    let Lists {left, right} = get_lists();
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    left_sorted.sort();
    right_sorted.sort();
    let mut distances: Vec<u32> = vec![];
    for i in 0..left.len() {
        let distance = left_sorted[i].abs_diff(right_sorted[i]);
        distances.push(distance);
    }
    let sum: u32 = distances.iter().sum();
    println!("The Distance is: {sum}")
}

fn get_lists() -> Lists  {
    let content = fs::read_to_string("input.txt").
        expect("Could not read file :(");    
    let lines_iter = content.split("\n");
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    lines_iter.for_each(|line| {
        let mut line_iter = line.split("   ");
        let left_id_str  = line_iter.next().expect("Left Id missing");
        let right_id_str = line_iter.next().expect("Right id missing");
        let left_id = left_id_str.parse::<u32>().expect("Unable to parse id");
        let right_id = right_id_str.parse::<u32>().expect("Unable to parse id");
        left.push(left_id);
        right.push(right_id);
    });
    return Lists {
        left: left,
        right: right
    };
}