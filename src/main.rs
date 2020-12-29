mod solution_files;
use solution_files::Solution;

fn test_problem_1() {
    println!("Problem 1");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    print!("nums: {:?}, target: {}", nums, target);
    println!(", solution: {:?}", Solution::two_sum(nums, target));
    let nums = vec![3, 2, 4];
    let target = 6;
    print!("nums: {:?}, target: {}", nums, target);
    println!(", solution: {:?}", Solution::two_sum(nums, target));
    let nums = vec![3, 3];
    let target = 6;
    print!("nums: {:?}, target: {}", nums, target);
    println!(", solution: {:?}", Solution::two_sum(nums, target));
}

fn test_problem_7() {
    println!("Problem 7");
    let x = 123;
    print!("Input: {}", x);
    println!(", Output: {}", Solution::reverse(x));
    let x = -123;
    print!("Input: {}", x);
    println!(", Output: {}", Solution::reverse(x));
    let x = 120;
    print!("Input: {}", x);
    println!(", Output: {}", Solution::reverse(x));
    let x = 0;
    print!("Input: {}", x);
    println!(", Output: {}", Solution::reverse(x));
}

fn test_problem_9() {
    println!("Problem 9");
    let x = 121;
    print!("Input: {}", x);
    println!(", Output: {}", Solution::is_palindrome(x));
    let x = -121;
    print!("Input: {}", x);
    println!(", Output: {}", Solution::is_palindrome(x));
    let x = 10;
    print!("Input: {}", x);
    println!(", Output: {}", Solution::is_palindrome(x));
    let x = -101;
    print!("Input: {}", x);
    println!(", Output: {}", Solution::is_palindrome(x));
}

fn test_problem_13() {
    println!("Problem 13");
    let x = String::from("III");
    print!("Input: {}, Output: ", x);
    println!("{}", Solution::roman_to_int(x));
    let x = String::from("IV");
    print!("Input: {}, Output: ", x);
    println!("{}", Solution::roman_to_int(x));
    let x = String::from("IX");
    print!("Input: {}, Output: ", x);
    println!("{}", Solution::roman_to_int(x));
    let x = String::from("LVIII");
    print!("Input: {}, Output: ", x);
    println!("{}", Solution::roman_to_int(x));
    let x = String::from("MCMXCIV");
    print!("Input: {}, Output: ", x);
    println!("{}", Solution::roman_to_int(x));
}

fn test_problem_14() {
    let test_1 = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let test_2 = vec![String::from("dog"), String::from("racecar"),String::from("car")];
    println!("{}", Solution::longest_common_prefix(test_1));
    println!("{}", Solution::longest_common_prefix(test_2));
}

fn test_problem_20() {
    let ex_1 = "()".to_string();
    assert_eq!(Solution::is_valid(ex_1), true);
    let ex_2 = "()[]{}".to_string();
    assert_eq!(Solution::is_valid(ex_2), true);
    let ex_3 = "(]".to_string();
    assert_eq!(Solution::is_valid(ex_3), false);
    let ex_4 = "([)]".to_string();
    assert_eq!(Solution::is_valid(ex_4), false);
    let ex_5 = "{[]}".to_string();
    assert_eq!(Solution::is_valid(ex_5), true);

}

fn main() {
    // test_problem_1();
    // test_problem_7();
    // test_problem_9();
    // test_problem_13();
    test_problem_20();
}
