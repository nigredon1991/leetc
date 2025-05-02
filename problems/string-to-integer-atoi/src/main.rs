fn main() {
    let input: Vec<&str> = vec![
        "42",
        "  -42",
        "1337c0d3",
        "0-1",
        "words and 987",
        "-91283472332",
        "+1",
        "+-12",
        "-6147483648",
        "  +  413"
    ];
    let expected: Vec<i32> = vec![42, -42, 1337, 0, 0, -2147483648, 1, 0, -2147483648, 0];
    for i in 0..input.len() {
        let out = my_atoi(String::from(input[i]));
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
pub fn my_atoi(s: String) -> i32 {
    let mut s_n = String::new();
    let n = s.len();
    let mut sign = 0;
    if s.len() == 0 {
        return 0;
    }
    let mut flag_sign = 0;
    let mut first_num = usize::MAX;
    for i in 0..n {
        let x = s.chars().nth(i).unwrap();
        match x {
            ' ' => {
                if flag_sign == 1 {
                    return 0;
                } else {
                    continue;
                }
            }
            '+' | '-' => {
                if flag_sign == 0 {
                    flag_sign = 1
                } else {
                    return 0;
                };
                continue;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                first_num = i;
                break;
            }
            _ => break,
        }
    }
    if first_num == usize::MAX {
        return 0;
    }
    if first_num != 0 && s.chars().nth(first_num - 1).unwrap() == '-' {
        sign = 1;
    }
    let mut not_zero_got = 0;
    for i in first_num..n {
        let x = s.chars().nth(i).unwrap();
        match x {
            '0' => {
                if not_zero_got == 1 {
                    s_n.push(x)
                }
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                s_n.push(x);
                not_zero_got = 1
            }
            _ => break,
        }
    }
    let mut out: i32 = 0;
    let mut cur_mul: i32 = 1;

    // println!("{lala}");

    // get first num
    const RADIX: u32 = 10;
    for i in (0..s_n.len()).rev() {
        let x: i32 = s_n
            .chars()
            .nth(i)
            .unwrap()
            .to_digit(RADIX)
            .unwrap()
            .try_into()
            .unwrap();
        // println!("out: {out}");
        // println!("x: {x}");
        // println!("cur_mul: {cur_mul}");

        if cur_mul == i32::MAX {
            if sign == 1 {
                return i32::MIN;
            }
            return i32::MAX;
        }
        if let Some(l) = cur_mul.checked_mul(x) {
        } else {
            if sign == 1 {
                return i32::MIN;
            }
            return i32::MAX;
        }
        let out_res = out.checked_add(cur_mul * x);
        if let Some(cur) = out_res {
            out = cur;
        } else {
            if sign == 1 {
                return i32::MIN;
            }
            return i32::MAX;
        }
        // println!("out_res: {out_res:?}");

        // out += cur_mul * x;
        // cur_mul *= 10;
        let cur_mul_res = cur_mul.checked_mul(10);
        if let Some(cur) = cur_mul_res {
            cur_mul = cur;
            continue;
        } else {
            cur_mul = i32::MAX;
        }
    }
    if sign == 1 {
        return -out;
    }

    out
}
