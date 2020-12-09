fn main() {
    let contents = include_str!("../res/input.txt");
    let seats = contents.lines().collect::<Vec<_>>();
    let seat_numbers = seats.iter().map(|&seat| {
        binary_move(seat)
    }).collect::<Vec<_>>();
    let mut set: Vec<i32> = seat_numbers.iter().map(|&item|{
        return item.0 * 8 + item.1;
    }).collect::<Vec<_>>();
    let seat = find_seat(&mut set);
    println!("{}", seat);
}


fn binary_move(characters: &str) -> (i32, i32){
    let mut lower_bound = 0;
    let mut upper_bound = 127;
    let mut right_side = 0;
    let mut left_side = 7;
    let mut result: (i32, i32) = (0, 0);
    for elem in characters.chars() {
        match elem {
            'F' => {if (lower_bound + upper_bound) % 2 == 0 {upper_bound = (lower_bound + upper_bound) / 2} else {upper_bound = (lower_bound + upper_bound) / 2}},
            'B' => {if (lower_bound + upper_bound) % 2 == 0 {lower_bound = (lower_bound + upper_bound) / 2} else {lower_bound = (lower_bound + upper_bound) / 2 + 1}},
            'R' => {if (right_side + left_side) % 2 == 0 {right_side = (right_side + left_side) / 2} else {right_side = (right_side + left_side) / 2 + 1}},
            'L' => {if (right_side + left_side) % 2 == 0 {left_side = (right_side + left_side) / 2} else {left_side = (right_side + left_side) / 2}},
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


fn find_seat(vec:&mut Vec<i32>) -> i32 {
    let mut result = 0;
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let filtered_vec = vec.iter().filter(|&elem| {return (*elem > 7) && *elem <= (127 * 8);}).collect::<Vec<_>>();
    for elem in 1..filtered_vec.len() {
        if filtered_vec[elem] - filtered_vec[elem - 1] == 2 {
            result = filtered_vec[elem] - 1;
        }
    }
    result
}