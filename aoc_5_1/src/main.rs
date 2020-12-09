fn main() {
    let contents = include_str!("../res/input.txt");
    let seats = contents.lines().collect::<Vec<_>>();
    let seat_numbers = seats.iter().map(|&seat| {
        binary_move(seat)
    }).collect::<Vec<_>>();

    let max = find_max(seat_numbers.iter().map(|&item|{
        return item.0 * 8 + item.1;
    }).collect::<Vec<_>>());
    println!("{}", max);
}


fn binary_move(characters: &str) -> (i32, i32){
    let mut lower_bound = 0;
    let mut upper_bound = 127;
    let mut right_side = 0;
    let mut left_side = 7;
    let mut result: (i32, i32) = (0, 0);
    for elem in characters.chars() {
        match elem {
            'F' => {upper_bound = (lower_bound + upper_bound) / 2},
            'B' => {lower_bound = (lower_bound + upper_bound) / 2 + ((lower_bound + upper_bound) % 2)},
            'R' => {right_side = (right_side + left_side) / 2 + ((right_side + left_side) % 2)},
            'L' => {left_side = (right_side + left_side) / 2},
            _ => {},
        }
    }
    if characters.chars().nth(6).unwrap() == 'F' {
        result.0 = upper_bound;
    } else {
        result.0 = lower_bound;
    }

    if characters.chars().last().unwrap() == 'R' {
        result.1 = right_side;
    } else { 
        result.1 = left_side;
    }

    result
}


fn find_max(vec: Vec<i32>) -> i32 {
    let mut result = 0;

    for elem in vec {
        if elem >= result {
            result = elem;
        }
    }

    result
}


#[test]
fn assert_bin_move(){
    assert!(binary_move("FBFBBFFRLR") == (44, 5));
    assert!(binary_move("BFFFBBFRRR") == (70, 7));
    assert!(binary_move("FFFBBBFRRR") == (14, 7));
    assert!(binary_move("BBFFBBFRLL") == (102, 4));
}