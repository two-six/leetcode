fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
    println!("{:?}", Solution::generate_combinations(3));
}

struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        todo!();
        match n {
            1 => vec!["()".to_owned()],
            2 => vec!["(())".to_owned(), "()()".to_owned()],
            _ => {
                if n > 2 {
                    Self::generate_combinations(n)
                        .iter()
                        .filter(|x| x != &&[n] && x.iter().fold(0, |acc, y| y + acc) == n)
                        .map(|e| {
                            e.iter()
                                .map(|&num| Self::generate_parenthesis(num))
                                // println!("{:?}", e);
                                .fold(Vec::new(), |mut acc, vec| {
                                    let vec_c = vec.clone();
                                    println!("{:?}", vec_c);
                                    acc.extend(vec);
                                    acc
                                })
                                .iter()
                                .fold(String::new(), |acc, s| acc + s)
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            }
        }
        // Problem jest w tym, ze dla 2,1 to zwraca [(()), ()()] [()], a nastepnie jest to laczone w[(()), ()(), ()], a nastepnie laczone w string [(())()()()]
        // Przerob tak, zeby gdy pojawi sie vector z 2 elementami, to 1 byly przypisywane do kazdego z tych elementow, a nie do wszyskich naraz
        // ()

        // () ()
        // (())

        // ()()()
        // (())()
        // (()())
        // ((()))
        // ()(())

        // 1 1 1 1
        // 2 1 1
        // 1 2 1
        // 1 1 2
        // 3 1 -> (2 1) 1 -> (1 2) 1
        // 1 3 -> 1 (2 1) -> 1 (1 2)
        // ()()()()
        // (())()()
        // ((()))()
        // (((())))
        // ()(())()
        // ()((()))
        // ()()(())
        // (()()())
        // ((())())
        // (()(()))
    }

    fn generate_combinations(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![n]];
        for i in 1..n {
            for j in (1..n - i + 1).rev() {
                for mut c in Self::generate_combinations(j) {
                    c.push(i);
                    res.push(c);
                }
            }
        }
        res
    }
}
