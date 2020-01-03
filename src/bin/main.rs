use aoc2019::day1;
use aoc2019::day2;
use aoc2019::day3;
use aoc2019::day4;

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
    ];

    println!("{:?}", opt);

    for i in opt.days {
        mapping[i - 1]();
    }
}
