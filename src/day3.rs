use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

struct Instruction {
    dir: char,
    steps: i32,
}

fn parse_step_instruction(s: &str) -> Instruction {
    let dir: char = char::from(s.as_bytes()[0]);
    let steps = s.get(1..).unwrap().parse::<i32>().unwrap();

    Instruction { dir, steps }
}

fn generate_list(inst: &str, c: &Coord) -> Vec<Coord> {
    let inst: Instruction = parse_step_instruction(inst);
    let mut lst: Vec<Coord> = Vec::new();

    for i in 0..=inst.steps {
        let new_coord: Coord = match inst.dir {
            'R' => Coord { x: c.x + i, y: c.y },
            'L' => Coord { x: c.x - i, y: c.y },
            'U' => Coord { x: c.x, y: c.y + i },
            'D' => Coord { x: c.x, y: c.y - i },
            _ => panic!("Direction not found {}", inst.dir)
        };
        lst.push(new_coord);
    };

    lst
}

type CoordTable = HashMap<Coord, i32>;

fn find_points(steps: &str, tbl: &mut CoordTable) -> HashSet<Coord> {
    let mut curr: Coord = Coord { x: 0, y: 0 };

    let mut point_set: HashSet<Coord> = HashSet::new();
    let mut step_count: i32 = 0;

    for step in steps.split(',') {
        step_count -= 1;
        let step = step.trim();
        let new_points = generate_list(step, &curr);
        curr = new_points.last().unwrap().clone();

        for p in new_points {
            step_count += 1;
            let p = p.clone();
            if !tbl.contains_key(&p) {
                tbl.insert(p.clone(), step_count);
            }

            point_set.insert(p.clone());
        }
    }
    point_set
}

fn find_intersections(step1: &str, step2: &str, table1: &mut CoordTable, table2: &mut CoordTable) -> Vec<Coord> {
    let set1 = find_points(step1, table1);
    let set2 = find_points(step2, table2);

    let temp: Vec<Coord> = set1.intersection(&set2).map(|i| { i.clone() }).collect();

    /*
    let mut ps : Vec<Coord> = Vec::new();
    for i in set1.intersection(&set2) {
        ps.push(i.clone());
    }
    ps;
    */
    temp
}

fn find_distance(c: &Coord) -> i32 {
    i32::abs(c.x) + i32::abs(c.y)
}

fn sol1(steps1: &str, steps2: &str) -> i32 {
    let mut tb1 = CoordTable::new();
    let mut tb2 = CoordTable::new();
    tb1.insert(Coord { x: 0, y: 0 }, 0);
    tb2.insert(Coord { x: 0, y: 0 }, 0);

    let mut distances: Vec<i32> = find_intersections(steps1, steps2, &mut tb1, &mut tb2)
        .iter()
        .map(find_distance)
        .collect();

    distances.sort();
    distances[1]
}

fn sol2(steps1: &str, steps2: &str) -> i32 {
    let mut tb1 = CoordTable::new();
    let mut tb2 = CoordTable::new();
    tb1.insert(Coord { x: 0, y: 0 }, 0);
    tb2.insert(Coord { x: 0, y: 0 }, 0);

    let mut distances: Vec<i32> = find_intersections(steps1, steps2, &mut tb1, &mut tb2)
        .iter()
        .map(|pt| { tb1.get(pt).unwrap() + tb2.get(pt).unwrap() })
        .collect();

    distances.sort();
    distances[1]
}

fn solve_impl<T: Fn(&str, &str) -> i32>(f: T) -> i32
{
    let content = std::fs::read_to_string("./input/input3.txt").unwrap();
    let content: Vec<&str> = content.trim().lines().collect();
    f(content[0], content[1])
}

fn solve_a() {
    let ans = solve_impl(sol1);
    println!("Solution day3 a = {}", ans);
}

fn solve_b() {
    let ans = solve_impl(sol2);
    println!("Solution day3 b = {}", ans);
}

pub fn solve() {
    solve_a();
    solve_b();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve3a1() {
        assert_eq!(
            6,
            sol1(
                "R8,U5,L5,D3",
                "U7, R6, D4, L4",
            ))
    }

    #[test]
    fn test_solve3b1() {
        assert_eq!(
            30,
            sol2(
                "R8,U5,L5,D3",
                "U7, R6, D4, L4",
            ))
    }

    #[test]
    fn test_solve3a() {
        assert_eq!(
            159,
            sol1("R75, D30, R83, U83, L12, D49, R71, U7, L72",
                 "U62, R66, U55, R34, D71, R55, D58, R83")
        );

        assert_eq!(
            135,
            sol1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                 "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        );
    }

    #[test]
    fn test_solve3b() {
        assert_eq!(
            610,
            sol2("R75, D30, R83, U83, L12, D49, R71, U7, L72",
                 "U62, R66, U55, R34, D71, R55, D58, R83")
        );

        assert_eq!(
            410,
            sol2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                 "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        );
    }
}


