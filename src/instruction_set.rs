use std::collections::VecDeque;

pub enum OpcodeResult {
    Handled(usize),
    AwaitingInput(usize), // The program counter
    Exit,
    Error,
}

fn handle_opcode(program: &mut Vec<i32>, pc: usize, inputs: &mut VecDeque<i32>, outputs: &mut Vec<i32>) -> OpcodeResult {
    let opcode = program[pc];
    let inst = opcode % 100;
    let params = opcode / 100;
    let am = (params % 10) as usize;
    let bm = (params / 10) as usize;

    let rval = |param, index: usize| -> i32 {
        if param == 1 {
            program[pc + index]
        } else {
            program[program[pc + index] as usize]
        }
    };

    match inst {
        1 => {
            let op1 = rval(am, 1);
            let op2 = rval(bm, 2);
            let result_op = program[pc + 3] as usize;
            program[result_op] = op1 + op2;
            OpcodeResult::Handled(pc + 4)
        }
        2 => {
            let op1 = rval(am, 1);
            let op2 = rval(bm, 2);
            let result_op = program[pc + 3] as usize;
            program[result_op] = op1 * op2;
            OpcodeResult::Handled(pc + 4)
        }
        3 => {
            //let mut read_string = String::new();
            //reader.read_line(&mut read_string).unwrap();
            //let ri = read_string.trim().parse::<i32>().unwrap();
            if inputs.is_empty() {
                OpcodeResult::AwaitingInput(pc)
            } else {
                let ri = inputs.pop_front().unwrap();

                let result_op = program[pc + 1] as usize;
                program[result_op] = ri;
                OpcodeResult::Handled(pc + 2)
            }
        }
        4 => {
            let op1 = rval(am, 1);
            outputs.push(op1);
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
            let result_op = program[pc + 3] as usize;
            program[result_op] = if op1 < op2 { 1 } else { 0 };
            OpcodeResult::Handled(pc + 4)
        }
        8 => {
            let op1 = rval(am, 1);
            let op2 = rval(bm, 2);
            let result_op = program[pc + 3] as usize;
            program[result_op] = if op1 == op2 { 1 } else { 0 };
            OpcodeResult::Handled(pc + 4)
        }
        99 => OpcodeResult::Exit,
        _ => OpcodeResult::Error,
    }
}

pub fn run_program(program: &mut Vec<i32>, pc: usize, inputs: &mut VecDeque<i32>, outputs: &mut Vec<i32>) -> OpcodeResult {
    let mut next_inst = pc;

    let vec_len = program.len();

    while next_inst < vec_len {
        let res = handle_opcode(program, next_inst, inputs, outputs);
        match res {
            OpcodeResult::Handled(nxt) => {
                next_inst = nxt;
            }
            OpcodeResult::Exit | OpcodeResult::Error | OpcodeResult::AwaitingInput(_) => return res
        };
        //println!("{:?}", vec);
    }

    OpcodeResult::Exit
}

pub fn run_program_first_location(program: &mut Vec<i32>, pc: usize, inputs: &mut VecDeque<i32>, outputs: &mut Vec<i32>) -> i32 {
    run_program(program, pc, inputs, outputs);
    program[0]

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2a() {
        assert_eq!(2, run_program_first_location(&mut vec!(1, 0, 0, 0, 99), 0, &mut VecDeque::new(), &mut Vec::new()));
        assert_eq!(2, run_program_first_location(&mut vec!(2, 3, 0, 3, 99), 0, &mut VecDeque::new(), &mut Vec::new()));
        assert_eq!(30, run_program_first_location(&mut vec!(1, 1, 1, 4, 99, 5, 6, 0, 99), 0, &mut VecDeque::new(), &mut Vec::new()));
        assert_eq!(
            3500,
            run_program_first_location(&mut vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50), 0, &mut VecDeque::new(), &mut Vec::new())
        );
    }

    #[test]
    fn test_day5_1() {
        let mut v = vec!(1002, 4, 3, 4, 33);
        run_program(&mut v, 0, &mut VecDeque::new(), &mut Vec::new());
        assert_eq!(99, v[4]);
    }

    fn test_io(mut program: Vec<i32>, input_elem: i32, exp_output: Vec<i32>) {
        let mut output = Vec::new();
        let mut input = VecDeque::new();
        input.push_back(input_elem);

        run_program(&mut program, 0, &mut input, &mut output);
        assert_eq!(exp_output.len(), output.len());
        for i in 0..exp_output.len() {
            assert_eq!(exp_output[i], output[i]);
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


}

