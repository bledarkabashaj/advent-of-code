fn main() {
    let contents = std::fs::read_to_string("./res/input.txt").expect("Could not read file");
    let count: usize = contents.lines().collect::<Vec<&str>>().iter().map(|&l| {split_line(l)} ).filter(|elem| is_valid(elem)).count();
    println!("{}", count);
}



pub fn split_line(line: &str) -> (i32, i32, char, &str){
    let minmax: Vec<&str> = line.split(['-',' ',':'].as_ref()).collect();
    (minmax[0].parse::<i32>().unwrap(), minmax[1].parse::<i32>().unwrap(), minmax[2].chars().nth(0).unwrap(), minmax[4])
}


pub fn is_valid(element: &(i32, i32, char, &str)) -> bool {
    let reps = element.3.chars().filter(|c| *c == element.2).count();
    if reps >= element.0 as usize && reps <= element.1 as usize {
        return true;
    } else {
        return false;
    }
}
