extern crate rand;


fn main() {
    let (width, height) = (50, 20);
    let rule = [false, true, true, false, true, true, true, false];
    draw_rule(width, height, rule);
}


fn draw_rule(width: u8, height: u8, rule: [bool; 8]) {
    let mut line = get_random_line(width);
    for _ in 0..height {
        write_line(line.clone());
        line = get_next_line(line, rule);
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

fn get_next_line(previous_line: Vec<bool>, rule: [bool; 8]) -> Vec<bool> {
    previous_line.clone().into_iter().enumerate().map(|(i, _)| {
        rule[get_position(previous_line.clone(), i)]
    }).collect()
}

fn get_position(line: Vec<bool>, i: usize) -> usize {
    let last = line.len() - 1;

    let (x, y, z) =
        if      i == 0    { (line[last],     line[0],    line[1])     }
        else if i == last { (line[last - 1], line[last], line[0])     }
        else              { (line[i - 1],    line[i],    line[i + 1]) };

    if       x &&  y &&  z { 0 } // 111
    else if  x &&  y && !z { 1 } // 110
    else if  x && !y &&  z { 2 } // 101
    else if  x && !y && !z { 3 } // 100
    else if !x &&  y && !z { 4 } // 011
    else if !x &&  y && !z { 5 } // 010
    else if !x && !y && !z { 6 } // 001
    else                   { 7 } // 000
}
