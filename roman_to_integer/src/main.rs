fn main() {
    println!("{}", roman_to_int(String::from("MCMXCIV")));
}

fn roman_to_int(s: String) -> i32 {
    s.chars()
        .map(|c| char_to_i32(&c))
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|chunk| {
            if chunk[0] < chunk[1] {
                -chunk[0]
            } else {
                chunk[0]
            }
        })
        .fold(0, |acc, val| acc + val)
        + char_to_i32(&s.chars().last().unwrap())
}

fn char_to_i32(c: &char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}
