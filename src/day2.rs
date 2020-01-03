use std::fs;

enum OpcodeResult {
    Handled(usize),
    Exit,
    Error,
}

fn handle_opcode(vec: &mut Vec<i32>, pc: usize) -> OpcodeResult {
    match vec[pc] {
        1 => {
            let (op1, op2): (usize, usize) = (vec[pc + 1] as usize, vec[pc + 2] as usize);
            let result_op = vec[pc + 3] as usize;
            vec[result_op] = vec[op1] + vec[op2];
            OpcodeResult::Handled(pc + 4)
        }
        2 => {
            let (op1, op2): (usize, usize) = (vec[pc + 1] as usize, vec[pc + 2] as usize);
            let result_op = vec[pc + 3] as usize;
            vec[result_op] = vec[op1] * vec[op2];
            OpcodeResult::Handled(pc + 4)
        }
        99 => OpcodeResult::Exit,
        _ => OpcodeResult::Error,
    }
}

fn run_program(vec: &mut Vec<i32>) -> i32 {
    let mut next_inst = 0;

    let vec_len = vec.len();

    while next_inst < vec_len {
        match handle_opcode(vec, next_inst) {
            OpcodeResult::Handled(nxt) => {
                next_inst = nxt;
            }
            OpcodeResult::Exit => break,
            OpcodeResult::Error => println!("Bad state reached"),
        };
        //println!("{:?}", vec);
    }

    vec[0]
}

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
        run_program(vec);
        println!("Solution for Day2 part a: {}", vec[0])
    })
}

fn solve_b_impl(addr: &mut Vec<i32>) {
    for i in 0..100 {
        for j in 0..100 {
            let mut new_addr = addr.clone();
            new_addr[1] = i;
            new_addr[2] = j;
            run_program(&mut new_addr);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2a() {
        assert_eq!(2, run_program(&mut vec!(1, 0, 0, 0, 99)));
        assert_eq!(2, run_program(&mut vec!(2, 3, 0, 3, 99)));
        assert_eq!(30, run_program(&mut vec!(1, 1, 1, 4, 99, 5, 6, 0, 99)));
        assert_eq!(
            3500,
            run_program(&mut vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50))
        );
    }
}
