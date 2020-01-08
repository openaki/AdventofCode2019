
#[derive(Debug, PartialOrd, PartialEq)]
struct LayerInfo {
    zeros: i32,
    ones: i32,
    twos: i32
}

fn solve_a(pixels: & str, width: usize, height: usize) {

    let layer_len: usize = width * height;

    let bytes = pixels.as_bytes();

    let mut layer_offset = 0;

    let mut layer_infos = Vec::new();

    while layer_offset < bytes.len() {

        let mut zeros = 0;
        let mut ones = 0;
        let mut twos= 0;

        for i in 0..layer_len {
            match bytes[layer_offset + i] {
                b'0' => {zeros += 1;},
                b'1' => {ones += 1;},
                b'2' => {twos += 1;},
                _ => {},
            }
        }

        layer_infos.push(LayerInfo {zeros, ones, twos});

        layer_offset += layer_len;
    }

    layer_infos.sort_by(|a, b| {a.zeros.cmp(&b.zeros)});

    //eprintln!("layer_infos = {:#?}", layer_infos);

    println!("Day 7 sol a {}", layer_infos[0].ones * layer_infos[0].twos);

}


fn solve_b(pixels: &str, width: usize, height: usize) {
    let layer_len: usize = width * height;

    let bytes = pixels.as_bytes();

    let mut layer_offset = 0;

    let mut final_image = Vec::new();
    final_image.resize(layer_len, '2');

    while layer_offset < bytes.len() {
        for i in 0..layer_len {
            if final_image[i] == '2' {
                final_image[i] = bytes[layer_offset + i] as char;
            }
        }
        layer_offset += layer_len;
    }

    println!("Day 7 sol b");
    for i in 0..height {
        for j in 0..width {
            let mut display = ' ';
            if final_image[i * width + j] == '1' {
                display = '+';
            }
            print!("{}", display);
        }
        println!();
    }

}

pub fn solve() {

    let fdata= std::fs::read_to_string("./input/input8.txt");
    let content = fdata.unwrap();

    solve_a(content.clone().trim(), 25, 6);
    solve_b(content.clone().trim(), 25, 6);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_1() {
        solve_a("123456789012", 3, 2);

    }

    #[test]
    fn test_day7_2() {
        solve_b("0222112222120000", 2, 2);
    }
}