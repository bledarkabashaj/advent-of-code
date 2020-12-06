fn main() {
    let contents = std::fs::read_to_string("./res/input.txt").expect("Could not read file!");
    let lines: Vec<&str> = contents.lines().collect();
    let mut x_axis: usize = 0;
    let mut y_axis: usize = 0;
    let line_length = lines[0].len();
    let mut count = 0;
    loop {
        x_axis += 3;
        y_axis += 1;
        if lines.get(y_axis).unwrap().chars().nth(x_axis % line_length).unwrap() == '#'{
            count += 1;
        }
        if y_axis == lines.len() - 1 {
            break;
        }
    }

    println!("{}", count);
}
