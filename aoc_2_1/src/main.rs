fn main() {
    let contents = std::fs::read_to_string("./res/input.txt").expect("Could not read file");
    let workspace: Vec<(i32, i32, char, &str)> = contents.lines().collect::<Vec<&str>>().iter().map(|l| split_line(l) ).collect();
    let result = workspace.iter().filter(|&elem| is_valid(elem)).count();
    print!("{}", result);
}



pub fn split_line(line: &str) -> (i32, i32, char, &str){
    let vec: Vec<&str> = line.split(" ").collect();
    let minmax: Vec<i32> = vec[0].split("-").map(|l| l.parse::<i32>().unwrap()).collect();
    let character = vec[1].chars().nth(0).unwrap();
    let password = vec[2];
    (minmax[0], minmax[1], character, password)
}


pub fn is_valid(element: &(i32, i32, char, &str)) -> bool {
    let reps = element.3.chars().filter(|c| *c == element.2).count();
    if reps >= element.0 as usize && reps <= element.1 as usize {
        return true;
    } else {
        return false;
    }
}