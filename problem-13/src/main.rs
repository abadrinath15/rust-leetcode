struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let mut prior = '0';
        for c in s.chars().rev() {
            match c {
                'I' => match prior{
                    'V' | 'X' => res -= 1,
                    _ => res += 1,
                }
                'V' => res += 5,
                'X' => match prior {
                    'L' | 'C' => res -= 10,
                    _ => res += 10,
                },
                'L' => res += 50,
                'C' => match prior {
                    'D' | 'M' => res -= 100,
                    _ => res += 100,
                },
                'D' => res += 500,
                'M' => res += 1000,
                _ => res += 0,
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
