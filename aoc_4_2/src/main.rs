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
        validate_fields(pass);
        return true;
    }).collect::<Vec<String>>().len();

    println!("{}", count);

}

static SCOPE: &str = "abcdef123456789";
static EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];



fn validate_fields(pass: &String) -> bool {
    validate_byr(pass.as_str()) && validate_iyr(pass.as_str()) && validate_eyr(pass.as_str()) && validate_hgt(pass.as_str()) && validate_hcl(pass.as_str()) && validate_ecl(pass.as_str()) && validate_pid(pass.as_str())
}


fn validate_byr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    year >= 1920 && year <= 2002
}

fn validate_iyr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    year >= 2010 && year <= 2020
}

fn validate_eyr(value: &str) -> bool {
    let year: i32 = value.parse().unwrap();
    year >= 2020 && year <= 2030
}

fn validate_hgt(value: &str) -> bool {
    let hgt_inch: Vec<&str> = value.split("in").collect();
    if hgt_inch.len() > 0 {
        let res: i32 = hgt_inch[0].parse().unwrap();
        return res >= 59 && res <= 76;
    } else {
        let hgt_cm: Vec<&str> = value.split("cm").collect();
        if hgt_cm.len() > 0 {
            let res: i32 = hgt_cm[0].parse().unwrap();
            return res >= 150 && res <= 193;
        }
    }
    false
}

fn validate_hcl(value: &str) -> bool {
    if value.chars().nth(0).unwrap() != '#' {
        return false;
    }
    let mut counter: usize = 0;
    for elem in value.chars() {
        if counter == 0 {
            counter += 1;
            continue;
        } else {
            if !SCOPE.contains(elem) {
                return false;
            }
        }
        counter += 1;
    }
    false
}

fn validate_ecl(value: &str) -> bool {
    true
}

fn validate_pid(value: &str) -> bool {
    true
}