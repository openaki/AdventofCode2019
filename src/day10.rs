pub fn solve() {
    let content = std::fs::read_to_string("./input/input10.txt").unwrap();
    solve_a(content.trim());
}

#[derive(Debug)]
enum Node {
    Space,
    Comet,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Node>>,
}

fn count_visible_comet(graph: &Graph, ci: usize, cj: usize) -> i32 {
    let num_rows = graph.nodes.len();
    let num_cols = graph.nodes[0].len();

    let mut angles_seen_up = std::collections::HashSet::new();
    let mut angles_seen_down = std::collections::HashSet::new();
    match graph.nodes[ci][cj] {
        Node::Space => 0,
        Node::Comet => {
            for i in 0..num_rows {
                for j in 0..num_cols {
                    match graph.nodes[i][j] {
                        Node::Space => {}
                        Node::Comet => {
                            let y_dist: f64 = ci as f64 - i as f64;
                            let x_dist: f64 = cj as f64 - j as f64;
                            let angle = (x_dist / y_dist).atan() * 100000000. + 0.5;
                            let angle = angle as i64;

                            if i < ci {
                                angles_seen_up.insert(angle);
                            } else {
                                angles_seen_down.insert(angle);
                            }
                        }
                    }
                }
            }

            angles_seen_up.len() as i32 + angles_seen_down.len() as i32
        }
    }
}

fn find_visibles(graph: &Graph) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let num_rows = graph.nodes.len();
    let num_cols = graph.nodes[0].len();

    let mut col_vec = Vec::new();
    col_vec.resize(num_cols, 0);
    ans.resize(num_rows, col_vec.clone());

    for i in 0..num_rows {
        for j in 0..num_cols {
            ans[i][j] = count_visible_comet(&graph, i, j);
        }
    }

    ans
}

fn find_max(v: Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            let val = v[i][j];
            max = std::cmp::max(val, max);
        }
    }
    max
}

fn solve_a(s: &str) -> i32 {
    let mut graph = Graph { nodes: Vec::new() };
    for l in s.lines() {
        let mut row = Vec::new();

        for b in l.bytes() {
            if b == b'.' {
                row.push(Node::Space);
            } else {
                row.push(Node::Comet);
            }
        }
        graph.nodes.push(row);
    }

    //println!("{:#?}", graph);

    let visibility = find_visibles(&graph);

    // for i in 0..visibility.len() {
    //     println!("{:?}", visibility[i]);
    // }

    let ans = find_max(visibility) - 1;

    println!("Solution day10, part a: {}", ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_1() {
        let test_str = ".#..#
.....
#####
....#
...##";
        let ans = solve_a(test_str);
        assert_eq!(ans, 8);
    }

    #[test]
    fn day10_2() {
        let test_str = "......#.#.
#..#.#....
..#######.
 .#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let ans = solve_a(test_str);
        assert_eq!(ans, 33);
    }

    #[test]
    fn day10_3() {
        let test_str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let ans = solve_a(test_str);
        assert_eq!(ans, 35);
    }
    #[test]
    fn day10_4() {
        let test_str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";
        let ans = solve_a(test_str);
        assert_eq!(ans, 41);
    }

    #[test]
    fn day10_5() {
        let test_str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
        let ans = solve_a(test_str);
        assert_eq!(ans, 210);
    }
}
