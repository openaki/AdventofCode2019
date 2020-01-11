use std::collections::VecDeque;

pub enum OpcodeResult {
    Handled(usize),
    AwaitingInput,
    Exit,
    Error,
}

pub struct Program {
    pub program : Vec<i64>,
    pub program_counter : usize,
    pub inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,
    pub relative_base: i32,
}

impl Program {
    pub fn new(program: Vec<i64>) -> Program {
        Program::new_with_io(program, 0, VecDeque::new(), VecDeque::new())
    }

    pub fn new_with_io(program: Vec<i64>, program_counter: usize, inputs: VecDeque<i64>, outputs: VecDeque<i64>) -> Program {
        Program { program, program_counter, inputs, outputs, relative_base: 0}
    }
}

fn handle_opcode(program_state: &mut Program) -> OpcodeResult {
    let program = &mut program_state.program;
    let inputs =  &mut program_state.inputs;
    let outputs = &mut program_state.outputs;
    let pc = program_state.program_counter;
    let opcode = program[pc];
    let inst = opcode % 100;
    let params = opcode / 100;
    let am = (params % 10) as i64;
    let bm = ((params / 10) % 10) as i64;
    let cm = (params / 100) as i64;
    let relative_base = program_state.relative_base;

    // eprintln!("inst = {:#?}", inst);
    // eprintln!("program = {:?}", program);
    // eprintln!("relative_base = {:#?}", relative_base);
    // eprintln!("outputs = {:?}", outputs);

    let mut get_fn = |program: &mut Vec<i64>, index: usize| -> i64 {
        if index >= program.len() {
            program.resize(index + 1, 0);
        }
        program[index]
    };

    let mut rval = |param, index: i32| -> i64 {
        let pi: usize =
        if param == 1 {
            (pc as i32 + index) as usize
        } else if  param == 2 {
            (relative_base as i64 + program[pc + index as usize]) as usize
        }
        else {
            program[(pc as i32 + index) as usize] as usize
        };

        get_fn(program, pi)
    };
    let mut sval = |program: &mut Vec<i64>, param, index| -> usize {
        if param == 1 {
            (pc as i32 + index) as usize
        } else if param == 2 {
            (relative_base as i64 + program[pc + index as usize]) as usize
        } else {
            program[(pc as i32 + index) as usize] as usize
        }
    };

    let mut set_fn = |program : &mut Vec<i64>, index: usize, val: i64| {
        if index >= program.len() {
            program.resize(index as usize+ 1, 0);
        }
        program[index] = val;
    };

    match inst {
        1 => {
            let op1 = rval(am, 1);
            let op2 = rval(bm, 2);
            let result_op = sval(program, cm , 3) as usize;
            set_fn(program, result_op, op1 + op2);
            OpcodeResult::Handled(pc + 4)
        }
        2 => {
            let op1 = rval(am, 1);
            let op2 = rval(bm, 2);
            let result_op = sval(program, cm, 3) as usize;
            set_fn(program, result_op, op1 * op2);
            OpcodeResult::Handled(pc + 4)
        }
        3 => {
            if inputs.is_empty() {
                OpcodeResult::AwaitingInput
            } else {
                let ri = inputs.pop_front().unwrap();
                let location = program[pc + 1];
                let result_op =

                if am == 2 {
                    (relative_base as i64 + location) as usize
                } else {
                    location as usize
                };

                set_fn(program, result_op, ri);
                OpcodeResult::Handled(pc + 2)
            }
        }
        4 => {
            let op1 = rval(am, 1);
            outputs.push_back(op1);
            OpcodeResult::Handled(pc + 2)
        }
        5 => {
            let op1 = rval(am, 1);
            if op1 != 0 {
                let op2 = rval(bm, 2);
                OpcodeResult::Handled(op2 as usize)
            } else {
                OpcodeResult::Handled(pc + 3)
            }
        }
        6 => {
            let op1 = rval(am, 1);
            if op1 == 0 {
                let op2 = rval(bm, 2);
                OpcodeResult::Handled(op2 as usize)
            } else
            {
                OpcodeResult::Handled(pc + 3)
            }
        }
        7 => {
            let op1 = rval(am, 1);
            let op2 = rval(bm, 2);
            let result_op = sval(program, cm, 3) as usize;
            let ans = if op1 < op2 { 1 } else { 0 };
            set_fn(program, result_op, ans);
            OpcodeResult::Handled(pc + 4)
        }
        8 => {
            // eprintln!("opcode = {:#?}", opcode);
            // eprintln!("inst = {:#?}", inst);
            //eprintln!("program = {:?}", program);
            // eprintln!("relative_base = {:#?}", relative_base);
            // eprintln!("outputs = {:?}", outputs);
            // eprintln!("am = {:#?}", am);
            // eprintln!("bm = {:#?}", bm);
            // eprintln!("program[pc+3] = {:#?}", program[pc + 3]);

            let op1 = rval(am, 1);
            let op2 = rval(bm, 2);
            let result_op = sval(program, cm, 3) as usize;
            let ans = if op1 == op2 { 1 } else { 0 };
            set_fn(program, result_op, ans);

            OpcodeResult::Handled(pc + 4)
        }
        9 => {
            let op1 = rval(am, 1);
            //let op1 = program[pc + 1];
            program_state.relative_base += op1 as i32;
            OpcodeResult::Handled(pc + 2)
        }
        99 => OpcodeResult::Exit,
        _ => OpcodeResult::Error,
    }
}

pub fn run_program(program_state: &mut Program) -> OpcodeResult {

    let vec_len = program_state.program.len();

    while program_state.program_counter < vec_len {
        let res = handle_opcode(program_state);
        match res {
            OpcodeResult::Handled(nxt) => {
                program_state.program_counter = nxt;
            }
            OpcodeResult::Exit | OpcodeResult::Error | OpcodeResult::AwaitingInput => return res
        };
        //println!("{:?}", vec);
    }


    OpcodeResult::Exit
}

pub fn run_program_first_location(program_state: &mut Program) -> i32 {
    run_program(program_state);
    program_state.program[0] as i32

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2a() {
        assert_eq!(2, run_program_first_location(&mut Program::new(vec!(1, 0, 0, 0, 99))));
        assert_eq!(2, run_program_first_location(&mut Program::new(vec!(2, 3, 0, 3, 99))));
        assert_eq!(30, run_program_first_location(&mut Program::new(vec!(1, 1, 1, 4, 99, 5, 6, 0, 99))));
        assert_eq!(
             3500,
             run_program_first_location(&mut Program::new(vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50)))
        );
    }

    #[test]
    fn test_day5_1() {
        let v = vec!(1002, 4, 3, 4, 33);
        let mut p = Program::new(v);
        run_program(&mut p);
        assert_eq!(99, p.program[4]);
    }

    fn test_io(program: Vec<i64>, input_elem: i64, exp_output: Vec<i64>) {
        let output = VecDeque::new();
        let mut input = VecDeque::new();
        input.push_back(input_elem);

        let mut p = Program::new_with_io(program, 0, input, output);
        run_program(&mut p);
        assert_eq!(exp_output.len(), p.outputs.len());
        for i in 0..exp_output.len() {
            assert_eq!(exp_output[i], p.outputs[i]);
        }
    }

    #[test]
    fn test_day5_2() {
        let v = vec!(3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8);
        test_io(v.clone(), 8, vec!(1));
        test_io(v.clone(), 7, vec!(0));
        test_io(v.clone(), 9, vec!(0));
    }

    #[test]
    fn test_day5_3() {
        let v = vec!(3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8);

        test_io(v.clone(), 5, vec!(1));
        test_io(v.clone(), 9, vec!(0));
        test_io(v.clone(), 8, vec!(0));
    }

    #[test]
    fn test_day5_4() {
        let v = vec!(3, 3, 1108, -1, 8, 3, 4, 3, 99);
        test_io(v.clone(), 8, vec!(1));
        test_io(v.clone(), 5, vec!(0));
        test_io(v.clone(), 9, vec!(0));
    }

    #[test]
    fn test_day5_5() {
        let v = vec!(3, 3, 1107, -1, 8, 3, 4, 3, 99);
        test_io(v.clone(), 7, vec!(1));
        test_io(v.clone(), 9, vec!(0));
        test_io(v.clone(), 8, vec!(0));
    }

    #[test]
    fn test_day5_6() {
        let v = vec!(3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9);
        test_io(v.clone(), 0, vec!(0));
        test_io(v.clone(), -1, vec!(1));
        test_io(v.clone(), 1, vec!(1));
    }


    #[test]
    fn test_day5_7() {
        let v = vec!(3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1);
        test_io(v.clone(), 0, vec!(0));
        test_io(v.clone(), -1, vec!(1));
        test_io(v.clone(), 1, vec!(1));
    }

    #[test]
    fn test_day5_8() {
        let v = vec!(3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                     1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                     999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99);
        test_io(v.clone(), 4, vec!(999));
        test_io(v.clone(), 8, vec!(1000));
        test_io(v.clone(), 9, vec!(1001));
    }


    #[test]
    fn test_day9_1() {
        let v = vec!(109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99);
        let mut output = VecDeque::new();
        let mut input = VecDeque::new();

        let mut p = Program::new_with_io(v.clone(), 0, input, output);
        run_program(&mut p);

        eprintln!("p.outputs = {:#?}", p.outputs);

        assert_eq!(v.len(), p.outputs.len());
        for i in 0..v.len() {
            assert_eq!(v[i], p.outputs[i]);

        }

        // assert_eq!(exp_output.len(), p.outputs.len());
        // for i in 0..exp_output.len() {
        //     assert_eq!(exp_output[i], p.outputs[i]);
        // }
    }

    #[test]
    fn test_day9_2() {
        let v = vec!(1102, 34915192, 34915192, 7, 4, 7, 99, 0);

        let mut output = VecDeque::new();
        let mut input = VecDeque::new();

        let mut p = Program::new_with_io(v, 0, input, output);
        run_program(&mut p);

        eprintln!("output = {:#?}", p.outputs);
        eprintln!("p.relative_base = {:#?}", p.relative_base);

        // assert_eq!(exp_output.len(), p.outputs.len());
        // for i in 0..exp_output.len() {
        //     assert_eq!(exp_output[i], p.outputs[i]);
        // }
    }


    #[test]
    fn test_day9_3() {
        let v = vec!(104, 1125899906842624, 99);

        let mut output = VecDeque::new();
        let mut input = VecDeque::new();

        let mut p = Program::new_with_io(v, 0, input, output);
        run_program(&mut p);

        assert_eq!(1125899906842624, p.outputs[0]);
    }
    
    #[test]
    fn test_temp1() {
        let v = vec!(109, -1, 4, 1, 99);
        let mut output = VecDeque::new();
        let mut input = VecDeque::new();
        let mut p = Program::new_with_io(v, 0, input, output);
        run_program(&mut p);

        eprintln!("p.outputs = {:#?}", p.outputs);
    }


    #[test]
    fn test_temp2() {
        let v = vec!(109, 1, 203, 2, 204, 2, 99);
        let mut output = VecDeque::new();
        let mut input = VecDeque::new();
        input.push_back(9876);
        let mut p = Program::new_with_io(v, 0, input, output);
        run_program(&mut p);

        eprintln!("p.outputs = {:#?}", p.outputs);
    }
}

