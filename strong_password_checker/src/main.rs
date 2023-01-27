fn main() {
    let p1 = String::from("a");
    let p2 = String::from("aA1");
    let p3 = String::from("aaaaabbbb1234567890ABA");
    let p4 = String::from("aaa111");
    let p5 = String::from("ABABABABABABABABABAB1");
    let p6 = String::from("bbaaaaaaaaaaaaaaacccccc");
    let p7 = String::from("FFFFFFFFFFFFFFF11111111111111111111AAA");
    let p8 = String::from("aA123");
    let p9 = String::from("aaaabbbbccccddeeddeeddeedd");
    println!(
        "{} - {}",
        &p1,
        Solution::strong_password_checker(p1.clone())
    );
    println!(
        "{} - {}",
        &p2,
        Solution::strong_password_checker(p2.clone())
    );
    println!(
        "{} - {}",
        &p3,
        Solution::strong_password_checker(p3.clone())
    );
    println!(
        "{} - {}",
        &p4,
        Solution::strong_password_checker(p4.clone())
    );
    println!(
        "{} - {}",
        &p5,
        Solution::strong_password_checker(p5.clone())
    );
    println!(
        "{} - {}",
        &p6,
        Solution::strong_password_checker(p6.clone())
    );
    println!(
        "{} - {}",
        &p7,
        Solution::strong_password_checker(p7.clone())
    );
    println!(
        "{} - {}",
        &p8,
        Solution::strong_password_checker(p8.clone())
    );
    println!(
        "{} - {}",
        &p9,
        Solution::strong_password_checker(p9.clone())
    );
}

struct Solution {}

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        if password.len() <= 3 {
            return 6 - password.len() as i32;
        }

        let mut repeating = Self::repeating_letter(&password)
            .iter()
            .filter(|&&e| e >= 3)
            .map(|e| *e)
            .collect::<Vec<usize>>();

        let contains = vec![
            password.chars().any(|c| c.is_ascii_lowercase()),
            password.chars().any(|c| c.is_ascii_uppercase()),
            password.chars().any(|c| c.is_digit(10)),
        ]
        .iter()
        .filter(|&&e| e)
        .count();
        if password.len() <= 20 {
            let repeating_needed = repeating.iter().map(|e| e / 3).sum();
            let need_to_insert = if password.len() < 6 {
                6 - password.len()
            } else {
                0
            };
            if 3 - contains > repeating_needed {
                if need_to_insert > 3 - contains {
                    return need_to_insert as i32;
                } else {
                    return 3 - contains as i32;
                }
            } else {
                if repeating_needed < need_to_insert {
                    return need_to_insert as i32;
                } else {
                    return repeating_needed as i32;
                }
            }
        } else {
            if repeating.len() == 0 {
                return (password.len() - 17 - contains) as i32;
            } else {
                let mut remove_needed = password.len() - 20;
                for _ in 0..remove_needed {
                    if repeating.iter().any(|e| e % 3 == 0) {
                        for e in repeating.iter_mut() {
                            if *e % 3 == 0 {
                                *e -= 1;
                                remove_needed -= 1;
                                break;
                            }
                        }
                    }
                }
                for _ in 0..remove_needed {
                    let tmp_value = repeating
                        .clone()
                        .into_iter()
                        .filter(|&e| e >= 3)
                        .min_by(|&a, &b| (a % 3).cmp(&(b % 3)));
                    if let Some(n) = tmp_value {
                        for e in repeating.iter_mut() {
                            if *e == n {
                                *e -= 1;
                                remove_needed -= 1;
                                break;
                            }
                        }
                    }
                }
                let repeating_needed = repeating.iter().map(|e| e / 3).sum::<usize>();
                (password.len() - 20
                    + if 3 - contains > repeating_needed {
                        3 - contains
                    } else {
                        repeating_needed
                    }) as i32
            }
        }
    }
    fn repeating_letter(password: &str) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut current_letter: char = password.chars().next().unwrap();
        let mut prev_start = 0;
        password.chars().enumerate().for_each(|(i, c)| {
            if c != current_letter {
                result.push(i - prev_start);
                prev_start = i;
                current_letter = c;
            }
        });
        result.push(password.len() - prev_start);
        result
    }
}
