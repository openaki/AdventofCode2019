fn is_valid_parta(num_str: &str) -> bool {
    let mut contains_adj = false;
    let mut is_monotonic = true;
    let s = num_str.as_bytes();

    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            contains_adj |= true;
        }
        if s[i] < s[i - 1] {
            is_monotonic = false;
            break;
        }
    }
    contains_adj && is_monotonic
}


fn is_valid_partb(num_str: &str) -> bool {
    let mut contains_adj = false;
    let mut is_monotonic = true;
    let s = num_str.as_bytes();
    let mut last_byte = s[0];
    let mut same_count = 0;

    for i in 1..s.len() {
        if s[i] == last_byte {
            same_count += 1;
        } else {
            if same_count == 1 {
                contains_adj = true;
            }
            same_count = 0;
        }

        if s[i] < last_byte {
            is_monotonic = false;
            break;
        }

        last_byte = s[i];
    }

    if same_count == 1 {
        contains_adj = true;
    }

    contains_adj && is_monotonic
}

fn solve_impl<T: Fn(&str) -> bool >(f: T) -> i32 {
    let mut count = 0;
    for i in 165432..707912 {
        let s = format!("{}", i);
        if f(&s) {
            count += 1;
        }
    }
    count
}

pub fn solve() {
    let sola = solve_impl(is_valid_parta);
    let solb = solve_impl(is_valid_partb);
    println!("Solution day4 part a {}", sola);
    println!("Solution day4 part b {}", solb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_day4a() {
        assert_eq!(true, is_valid_parta("111111"));
        assert_eq!(false, is_valid_parta("223450"));
        assert_eq!(false, is_valid_parta("123789"));
    }


    #[test]
    fn solve_day4b() {
        assert_eq!(true, is_valid_partb("112233"));
        assert_eq!(false, is_valid_partb("123444"));
        assert_eq!(true, is_valid_partb("111122"));
    }
}