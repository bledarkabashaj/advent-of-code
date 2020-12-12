fn main() {
    let contents = std::fs::read_to_string("./res/input.txt").expect("Could not read file");
    let count: usize = contents.lines().collect::<Vec<&str>>().iter().map(|&l| {
            return l.split(['-',' ',':'].as_ref()).collect::<Vec<&str>>();
        }).filter(|elem| {
            return (elem.get(4).unwrap().chars().nth(elem.get(0).unwrap().parse::<usize>().unwrap() - 1).unwrap() == elem.get(2).unwrap().chars().nth(0).unwrap()) ^ (elem.get(4).unwrap().chars().nth(elem.get(1).unwrap().parse::<usize>().unwrap() - 1).unwrap() == elem.get(2).unwrap().chars().nth(0).unwrap());
        }).count();
    println!("{}", count);
}