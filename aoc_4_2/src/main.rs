fn main() {
    let contents= std::fs::read_to_string("./res/input.txt").expect("Could not read file!");

    let passports = contents.split("\n\n").collect::<Vec<&str>>();

    let folded_passports = passports.iter().map(|&pass| {
        let temp: String = pass.lines().collect::<Vec<&str>>().join(" ");
        return temp;
    });
    let fields = vec!["byr".to_string(),"iyr".to_string(),"eyr".to_string(),"hgt".to_string(),"hcl".to_string(),"ecl".to_string(),"pid".to_string()];

    let valid_passes = folded_passports.filter(|pass| {
        for elem in fields.iter() {
            if !pass.contains(elem){
                return false;
            }
        }
        true
    }).collect::<Vec<String>>();

    let mut count: i32 = 0;

    for elem in valid_passes {
        if validate_fields(&elem) {
            count += 1;
        }
    }

    println!("{}", count);

}

static SCOPE: &str = "abcdef123456789";
static NUM_SCOPE: &str = "0123456789";
static EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];



fn validate_fields(pass: &String) -> bool {
    let fields: Vec<&str> = pass.as_str().split(" ").collect();
    // let map: Vec<(String, String)> = fields.iter().map(|&field| {
        
    //     let sep: Vec<&str> = field.split(":").collect();
    //     return (sep[0].to_string(), sep[1].to_string());
    // }).collect();
        
    let mut map: Vec<(&str, &str)> = Vec::new();

    for elem in fields {
        let sep = elem.split(":").collect::<Vec<&str>>();
        if sep.len() == 2 {
            map.push((sep[0], sep[1]));
        }
    }

    for elem in &map {
        if elem.0[..].len() == 0 {
            continue;
        }
        match &elem.0[..] {
            "byr" =>if !validate_byr(elem.1) {continue} else {return false},
            "hgt" =>if !validate_hgt(elem.1) {continue} else {return false},
            "iyr" =>if !validate_iyr(elem.1) {continue} else {return false},
            "eyr" =>if !validate_eyr(elem.1) {continue} else {return false},
            "hcl" =>if !validate_hcl(elem.1) {continue} else {return false},
            "ecl" =>if !validate_ecl(elem.1) {continue} else {return false},
            "pid" =>if !validate_pid(elem.1) {continue} else {return false},
            _ => continue,
        }
    }

    map.clear();

    true
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
    if hgt_inch.len() > 0 && !(hgt_inch[0].contains("in") || hgt_inch[0].contains("cm")) {
        let res: i32 = hgt_inch[0].parse().unwrap();
        return res >= 59 && res <= 76;
    } else {
        let hgt_cm: Vec<&str> = value.split("cm").collect();
        if hgt_cm.len() > 0 && !(hgt_cm[0].contains("cm") || hgt_cm[0].contains("in")){
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
    true
}

fn validate_ecl(value: &str) -> bool {
    EYE_COLORS.contains(&value)
}

fn validate_pid(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }
    for elem in value.chars() {
        if !NUM_SCOPE.contains(elem) {
            return false;
        }
    }
    true
}
