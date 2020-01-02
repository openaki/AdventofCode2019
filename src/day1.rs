use std::fs;

pub fn solve_impl<T: Fn(i32) -> i32>(f: T) -> i32 {
    let content = fs::read_to_string("./input/input1.txt").unwrap();

    let mut sum_a: i32 = 0;

    for ln in content.lines() {
        let num = ln.parse::<i32>().unwrap();
        let ans_a = f(num);
        sum_a += ans_a;
    }

    sum_a
}

fn calc_fuel(m: i32) -> i32 {
    (m / 3) - 2
}

fn solve_a(input: i32) -> i32 {
    calc_fuel(input)
}

pub fn solve() {
    let sola = solve_impl(solve_a);
    println!("Sol a = {}", sola);
    let solb = solve_impl(solve_b);
    println!("Sol b = {}", solb);
}

fn solve_b(m: i32) -> i32 {
    fn rec_fn(m: i32, ans: i32) -> i32 {
        let new_fuel = calc_fuel(m);
        if new_fuel <= 0 {
            ans
        } else {
            rec_fn(new_fuel, ans + new_fuel)
        }
    };

    rec_fn(m, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1a() {
        assert_eq!(2, solve_a(12));
        assert_eq!(2, solve_a(14));
        assert_eq!(654, solve_a(1969));
        assert_eq!(33583, solve_a(100756));
    }

    #[test]
    fn test_day1b() {
        assert_eq!(2, solve_b(14));
        assert_eq!(966, solve_b(1969));
        assert_eq!(50346, solve_b(100756));
    }
}
