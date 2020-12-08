

fn main() {
    let contents= std::fs::read_to_string("./res/input.txt").expect("Could not read file!");

    let passports = contents.split("\n\n").collect::<Vec<&str>>();

    let folded_passports = passports.iter().map(|&pass| {
        let temp: String = pass.lines().collect::<Vec<&str>>().join(" ");
        return temp;
    });
    let fields = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];

    let count = folded_passports.filter(|pass| {
        for elem in fields.iter() {
            if !pass.contains(elem){
                return false;
            }
        }
        return true;
    }).collect::<Vec<String>>().len();

    println!("{}", count);

}

