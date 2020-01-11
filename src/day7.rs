use crate::instruction_set::{run_program, Program};

pub fn run_through_amplification(program: Vec<i64>, phase_setting: Vec<i64>, num_amplifiers: usize) -> i64 {
    let mut current_output = 0;

    let mut first_time = true;
    let mut programs = Vec::new();
    for _i in 0..num_amplifiers {
        programs.push(Program::new(program.clone()));
    }
    loop {
        for i in 0..num_amplifiers {
            let prog = &mut programs[i];

            if first_time {
                prog.inputs.push_back(phase_setting[i]);
            }
            prog.inputs.push_back(current_output);

            run_program(prog);

            if !prog.outputs.is_empty() {
                current_output = prog.outputs[0];
                prog.outputs.pop_front();
            } else {
                return current_output
            }

        }
        first_time = false;
    }
}

pub fn get_max_output(program: Vec<i64>, phase_min: i64, phase_max: i64) -> i64 {
    let mut max_output = 0;

    for p1 in phase_min..phase_max {
        for p2 in phase_min..phase_max {
            if p2 == p1 { continue; }
            for p3 in phase_min..phase_max {
                if p3 == p2 || p3 == p1 { continue; }

                for p4 in phase_min..phase_max {
                    if p4 == p3 || p4 == p2 || p4 == p1 { continue; }

                    for p5 in phase_min..phase_max {
                        if p5 == p4 || p5 == p3 || p5 == p2 || p5 == p1 { continue; }

                        let phases = vec!(p1, p2, p3, p4, p5);
                        let output = run_through_amplification(program.clone(), phases, 5);
                        if output > max_output {
                            max_output = output;
                        }
                    }
                }
            }
        }
    }
    max_output
}


fn solve_impl(phase_min: i64, phase_max:i64) -> i64 {
    let content = std::fs::read_to_string("./input/input7.txt").unwrap();
    let content = content.trim();
    let content: Vec<i64> = content
        .split(',')
        .map(|w| w.parse::<i64>().unwrap())
        .collect();

    get_max_output(content, phase_min, phase_max)
}


fn solve_a() {
    let sol = solve_impl(0, 5);
    println!("Solve day 5 part a: {}", sol);
}

fn solve_b() {
    let sol = solve_impl(5, 10);
    println!("Solve day 5 part b: {}", sol);
}

pub fn solve() {
    solve_a();
    solve_b();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_1() {
        let program = vec!(3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0);

        let phase_setting = vec!(4, 3, 2, 1, 0);

        let out = run_through_amplification(program.clone(), phase_setting, 5);
        eprintln!("out = {:#?}", out);

        assert_eq!(43210, get_max_output(program.clone(), 0, 5))
    }

    #[test]
    fn tes_day7_2() {
        let program = vec!(3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23,
                           101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0);

        assert_eq!(54321, get_max_output(program, 0, 5))
    }


    #[test]
    fn tes_day7_4() {
        let program = vec!(3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
                           1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0);

        assert_eq!(65210, get_max_output(program, 0, 5))
    }

    #[test]
    fn tes_day7_1_b() {
        let program = vec!(
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
            27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5);


        let phase_setting = vec!(9, 8, 7, 6, 5);

        let out = run_through_amplification(program.clone(), phase_setting, 5);
        eprintln!("out = {:#?}", out);
        assert_eq!(139629729, get_max_output(program, 5, 10))
    }

    #[test]
    fn tes_day7_2_b() {
        let program = vec!(
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10);

        assert_eq!(18216, get_max_output(program, 5, 10))
    }
}