extern crate rand;


fn main() {
    let (width, height) = (20, 20);
    draw_rule(width, height);
}


fn draw_rule(width: u8, height: u8) {
    for _ in 0..height {
        let random_line = get_random_line(width);
        write_line(random_line);
    }
}

fn get_random_line(len: u8) -> Vec<bool> {
    let mut line = vec![];
    for _ in 0..len {
        line.push(rand::random::<bool>());
    }
    line
}

fn write_line(line: Vec<bool>) {
    let masked_line = mask_line(line);
    let line_string: String = masked_line.into_iter().collect();
    println!("{}", line_string);
}

fn mask_line(line: Vec<bool>) -> Vec<char> {
    let mask: [char; 2] = ['🟩', '🟪'];
    line.into_iter().map(|i| {
        if i { mask[1] } else { mask[0] }
    }).collect()
}
