fn main() {
    let contents = std::fs::read_to_string("./res/input.txt").expect("Could not read file");
    let workspace: Vec<(i32, i32, char, &str)> = contents.lines().collect::<Vec<&str>>().iter().map(|l| split_line(l) ).collect();
    let result = workspace.iter().filter(|&elem| is_valid(elem)).count();
    println!("{}", result);
}



pub fn split_line(line: &str) -> (i32, i32, char, &str){
    let vec: Vec<&str> = line.split(" ").collect();
    let minmax: Vec<i32> = vec[0].split("-").map(|l| l.parse::<i32>().unwrap()).collect();
    let character = vec[1].chars().nth(0).unwrap();
    let password = vec[2];
    (minmax[0], minmax[1], character, password)
}


pub fn is_valid(element: &(i32, i32, char, &str)) -> bool {
    let characters: Vec<char> = element.3.chars().collect();
    if (characters[(element.0 as usize) - 1] == element.2 && characters[(element.1 as usize) - 1] != element.2) || (characters[(element.0 as usize) - 1] != element.2 && characters[(element.1 as usize) - 1] == element.2) {
        return true;
    } 

    false

}