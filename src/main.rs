use std::env;
use std::fs;

struct Lists {left: u128, right: u16}

fn main() {
    let Lists {left, right} =  get_lists();
    let x = left;
}

fn get_lists() -> Lists  {
    let content = fs::read_to_string("input.txt").
    expect("Could not read file :(");    
    let lines = content.split("\n");
    let lines_separated = lines.map(|line| line.split("   "));
    let left = lines_separated.map(|line| line[0]);
    let right = lines_separated.map(|line| line[1]);
    let lists = Lists {left: left, right: right};
    return lists;
}