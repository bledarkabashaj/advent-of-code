use std::fs;


fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("File could not be opened");
    let structure: Vec<&str> = input.lines().filter(|l| l.len() > 0).collect();
    let mut index: usize = 0;
    let mut result: (i32, i32, i32) = (0,0, 0);
    for elem in &structure {
        let mut jindex: usize = 0;
        for elem2 in &structure {
            let mut zindex: usize = 0;
            for elem3 in &structure {
                if index == jindex || index == zindex || jindex == zindex {
                    zindex += 1;
                    continue;
                }
                if elem.parse::<i32>().unwrap() + elem2.parse::<i32>().unwrap() + elem3.parse::<i32>().unwrap() == 2020 { 
                    result = (elem.parse::<i32>().unwrap(), elem2.parse::<i32>().unwrap(), elem3.parse::<i32>().unwrap());
                }
                zindex += 1;
            }
            jindex += 1;
        }
        index += 1;
    }
    

    println!("{}", result.0 * result.1 * result.2);
}
