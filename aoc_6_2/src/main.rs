use std::collections::HashSet;

fn main() {
    let contents = include_str!("../res/input.txt");
    let groups = contents.split("\n\n").collect::<Vec<_>>();
    let groups_parsed = groups.iter().map(|& group|{
        let temp = group.lines().collect::<Vec<_>>();
        let other_temp = temp.join("");
        other_temp
    }).collect::<Vec<_>>();

    let scope_vec = groups_parsed.iter().map(|group| {
        let mut result: HashSet<char> = HashSet::new();
        for elem in group.chars() {
            result.insert(elem);
        }
        result
    }).collect::<Vec<_>>();
    let mut result_vec: Vec<usize> = Vec::new();
    for idx in 0..scope_vec.len() {
        let mut c_counter: usize = 0;
        for character in scope_vec.get(idx).unwrap() {
            let length = groups.get(idx).unwrap().lines().collect::<Vec<_>>().len();
            let mut count: usize = 0;
            for elem in groups.get(idx).unwrap().chars() {
                if elem == *character {
                    count += 1;
                }
            }
            if length == count {
                c_counter += 1;
            }
        }
        result_vec.push(c_counter);
    }
    let mut res: usize = 0;
    for elem in result_vec {
        res += elem;
    }

    println!("{}", res);
}