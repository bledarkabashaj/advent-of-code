fn main() {
    let contents= std::fs::read_to_string("./res/input.txt").expect("Could not read file!");
    
    let passports = contents.split("\n\n").collect::<Vec<&str>>();

    let validity = passports.iter().all(|str| !str.contains("\r"));
    assert!(validity);

    let folded_passports = passports.iter().map(|&pass| {
        let temp: String = pass.lines().collect::<Vec<&str>>().join(" ");
        return temp;
    }).collect::<Vec<_>>();
    let fields = vec!["byr".to_string(),"iyr".to_string(),"eyr".to_string(),"hgt".to_string(),"hcl".to_string(),"ecl".to_string(),"pid".to_string()];

    let valid_passes = folded_passports.iter().filter(|pass| {
        for elem in fields.iter() {
            if !pass.contains(elem){
                return false;
            }
        }
        validate_fields(pass)
    }).collect::<Vec<_>>();
    println!("{}", valid_passes.len());
}


fn validate_fields(pass: &String) -> bool {
    let fields = pass.as_str().split(" ").collect::<Vec<_>>();

    let map = fields.iter().map(|&pass| {
        let temp = pass.split(':').collect::<Vec<_>>();
        (temp[0], temp[1])
        }
    ).collect::<Vec<_>>();

    for elem in map {
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
    value.chars().count() == 7 && value.chars().skip(1).all(|character| {SCOPE.contains(character)}) 
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

#[test]
fn test_hcl(){
    assert!(validate_hcl("#abc123"));
    assert!(!validate_hcl("abc123#"));
    assert!(!validate_hcl("abc1234"));
    assert!(!validate_hcl("#123abz"));
}

#[test]
fn test_hgt(){
    assert!(validate_hgt("160cm"));
    assert!(validate_hgt("60in"));
    assert!(!validate_hgt("130cm"));
    assert!(!validate_hgt("20in"));
    assert!(!validate_hgt("140"));
    assert!(!validate_hgt("2000"));
}


#[test]
fn test_ecl(){
    assert!(validate_ecl("amb"));
    assert!(validate_ecl("blu"));
    assert!(validate_ecl("brn"));
    assert!(!validate_ecl("ass"));
    assert!(!validate_ecl("egt"));
    assert!(!validate_ecl("lel"));
}

#[test]
fn test_pid(){
    assert!(validate_pid("012345678"));
    assert!(validate_pid("123456789"));
    assert!(!validate_pid("123424fd3"));
    assert!(!validate_pid("23232323"));
    assert!(!validate_pid("1234238085"));
}



static SCOPE: &str = "abcdef0123456789";
static NUM_SCOPE: &str = "0123456789";
static EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

