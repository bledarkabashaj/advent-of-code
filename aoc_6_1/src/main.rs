use std::collections::HashSet;

fn main() {
    let contents = include_str!("../res/input.txt");
    let groups = contents.split("\n\n").collect::<Vec<_>>();
    let groups_parsed = groups.iter().map(|& group|{
        let temp = group.lines().collect::<Vec<_>>();
        let other_temp = temp.join("");
        other_temp
    }).collect::<Vec<_>>();

    let clean_vec = groups_parsed.iter().map(|group| {
        let mut result: HashSet<char> = HashSet::new();
        for elem in group.chars() {
            result.insert(elem);
        }
        result
    }).collect::<Vec<_>>();
    let mut counter: usize = 0;
    for elem in clean_vec {
        counter += elem.len();
    }

    println!("{}", counter);
}
