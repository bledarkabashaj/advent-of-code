fn main() {
    let contents = std::fs::read_to_string("./res/input.txt").expect("Could not read file");
    let count: usize = contents.lines().collect::<Vec<&str>>().iter().map(|&l| {
            return l.split(['-',' ',':'].as_ref()).collect::<Vec<&str>>();
        }).filter(|elem| {
            return (elem.get(0).unwrap().parse::<usize>().unwrap()..=elem.get(1).unwrap().parse::<usize>().unwrap()).contains(&elem.get(4).unwrap().chars().filter(|&character| {
                return character == elem.get(2).unwrap().chars().nth(0).unwrap();
            }).count());})
        .count();
    println!("{}", count);
}