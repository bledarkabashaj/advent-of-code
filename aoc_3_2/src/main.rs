fn main() {
    let contents = std::fs::read_to_string("./res/input.txt").expect("Could not read file!");
    let lines: Vec<&str> = contents.lines().collect();

    let slopes = [(1,1),(3,1),(5,1),(7,1),(1,2)];
    
    let mut count = 1;

    for slope in slopes.iter() {
        count *= count_trees(&lines, *slope);
    }

    

    println!("{}", count);
}


fn count_trees(lines: &Vec<&str> ,slope:(usize, usize)) -> usize {
    let mut x_axis: usize = 0;
    let mut y_axis: usize = 0;
    let line_length = lines[0].len();
    let mut count = 0;
    loop {
        x_axis += slope.0;
        y_axis += slope.1;
        if lines.get(y_axis).unwrap().chars().nth(x_axis % line_length).unwrap() == '#'{
            count += 1;
        }
        if y_axis + slope.1 >= lines.len() {
            break;
        }
    }

    println!("{}", count);
    count
}
