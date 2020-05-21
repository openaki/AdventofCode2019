use aoc2019::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Arg {
    days: Vec<usize>,
}

fn main() {
    let opt = Arg::from_args();

    let mapping: Vec<Box<dyn Fn()>> = vec![
        Box::new(day1::solve),
        Box::new(day2::solve),
        Box::new(day3::solve),
        Box::new(day4::solve),
        Box::new(day5::solve),
        Box::new(day6::solve),
        Box::new(day7::solve),
        Box::new(day8::solve),
        Box::new(day9::solve),
        Box::new(day10::solve),
    ];

    println!("{:?}", opt);

    for i in opt.days {
        mapping[i - 1]();
    }
}
