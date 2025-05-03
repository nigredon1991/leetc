fn main() {
    let input: Vec<i32> = vec![1, 101, 12, -101];
    let expected: Vec<bool> = vec![true, true, false, false];
    for i in 0..input.len() {
        let out = is_palindrome(input[i]);
        let exp = expected[i];

        if out == expected[i] {
            println!("Ok")
        } else {
            let inp = input[i];
            println!("Input: {inp}");
            println!("Got: {out}");
            println!("Expected: {exp}");
            println!("");
        }
    }
}
pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    let mut r = x.to_string().len()-1;
    let mut l = 0;
    while r > l {
        if s.chars().nth(r).unwrap() != s.chars().nth(l).unwrap() {
            return false;
        }
        r -= 1;
        l += 1;
    }
    true
}
