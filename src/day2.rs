use std::fs;
use crate::instruction_set::run_program;
use std::collections::VecDeque;

fn solve_impl<T: Fn(&mut Vec<i32>)>(f: T) {
    let content = fs::read_to_string("./input/input2.txt").unwrap();
    let content = content.trim();
    let mut content: Vec<i32> = content
        .split(',')
        .map(|w| w.parse::<i32>().unwrap())
        .collect();

    f(&mut content);
}

fn solve_a() {
    solve_impl(|vec: &mut Vec<i32>| {
        vec[1] = 12;
        vec[2] = 2;
        run_program(vec, 0, &mut VecDeque::new(), &mut Vec::new());
        println!("Solution for Day2 part a: {}", vec[0])
    })
}

fn solve_b_impl(addr: &mut Vec<i32>) {
    for i in 0..100 {
        for j in 0..100 {
            let mut new_addr = addr.clone();
            new_addr[1] = i;
            new_addr[2] = j;
            run_program(&mut new_addr, 0, &mut VecDeque::new(), &mut Vec::new());

            if new_addr[0] == 19690720 {
                println!("Solution 2 part b: {}", 100 * i + j);
                break;
            }
        }
    }
}

fn solve_b() {
    solve_impl(|vec: &mut Vec<i32>| solve_b_impl(vec))
}

pub fn solve() {
    solve_a();
    solve_b();
}

