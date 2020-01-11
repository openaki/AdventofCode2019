use crate::instruction_set::{run_program, Program};
use std::collections::VecDeque;

fn solve_impl(input_num: i64) -> i64 {
    let content = std::fs::read_to_string("./input/input9.txt").unwrap();
    let content = content.trim();
    let content: Vec<i64> = content
        .split(',')
        .map(|w| w.parse::<i64>().unwrap())
        .collect();
    let mut inputs = VecDeque::new();
    inputs.push_back(input_num);
    let mut p = Program::new_with_io(content, 0, inputs, VecDeque::new());
    run_program(&mut p);

    eprintln!("p.outputs = {:#?}", p.outputs);
    p.outputs.pop_front().unwrap()
}
fn solve_a() {
    let ans = solve_impl(1);

    println!("Solve day 9 a: {}", ans);
}

fn solve_b() {
    let ans = solve_impl(2);

    println!("Solve day 9 a: {}", ans);
}


pub fn solve() {
    solve_a();
    solve_b();
}