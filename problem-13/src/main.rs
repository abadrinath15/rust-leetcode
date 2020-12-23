struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0i32;
        let mut prior: char = '0';
        for c in s.chars() {
            match c {
                'I' => res = res + 1,
                'V' => match prior {
                    'I' => res = res + 3,
                    _ => res = res + 5,
                },
                'X' => match prior {
                    'I' => res = res + 8,
                    _ => res = res + 10,
                },
                'L' => match prior {
                    'X' => res = res + 30,
                    _ => res = res + 50,
                },
                'C' => match prior {
                    'X' => res = res + 80,
                    _ => res = res + 100,
                },
                'D' => match prior {
                    'C' => res = res + 300,
                    _ => res = res + 500,
                },
                'M' => match prior {
                    'C' => res = res + 800,
                    _ => res = res + 1000,
                },
                _ => res = res + 0,
            }
            prior = c;
        }
        res
    }
}

fn main() {
    let s1 = String::from("III");
    println!("{}", Solution::roman_to_int(s1));
    let s2 = String::from ("IV");
    println!("{}", Solution::roman_to_int(s2));
    let s3 = String::from ("IX");
    println!("{}", Solution::roman_to_int(s3));
    let s4 = String::from ("LVIII");
    println!("{}", Solution::roman_to_int(s4));
    let s5 = String::from ("MCMXCIV");
    println!("{}", Solution::roman_to_int(s5));


}
