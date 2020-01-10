use std::fs;
use crate::instruction_set::{run_program, Program};

fn solve_impl() -> i32 {
    let content = fs::read_to_string("./input/input5.txt").unwrap();
    let content = content.trim();
    let content: Vec<i32> = content
        .split(',')
        .map(|w| w.parse::<i32>().unwrap())
        .collect();
    let mut p = Program::new(content);
    run_program(&mut p);
    p.program[0]
}

fn solve_a() {
    println!("Enter 1");
    let sol = solve_impl();
    println!("Solve day 5 part a: {}", sol);
}

fn solve_b() {
    println!("Enter 5");
    let sol = solve_impl();
    println!("Solve day 5 part a: {}", sol);
}

pub fn solve() {
    solve_a();
    solve_b();
}

