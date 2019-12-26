use structopt::StructOpt;
use aoc2019::day1;
use aoc2019::day2;

#[derive(Debug, StructOpt)]
struct Arg {

    days: Vec<usize>,

}

fn main() {

    let opt = Arg::from_args();

    let mapping : Vec<Box <dyn Fn()>>= vec!(
        Box::new(day1::solve), 
        Box::new(day2::solve),
        );

    println!("{:?}", opt);

    for i in opt.days {
        mapping[i - 1]();
    }
}