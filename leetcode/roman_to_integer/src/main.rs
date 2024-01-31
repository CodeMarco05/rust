
fn main() {
    let numbers = roman_to_int("MMMMLIVIII".to_string());

    println!("Result: {}", numbers);
}

pub fn roman_to_int(s: String) -> i32 {
    let mut result :i32 = 0;

    let mut past_value = 0;

    for c in s.chars() {
        let current_value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0
        };

        if past_value < current_value {
            let num = current_value - past_value;
            result -= past_value;
            result += num;
        }else {
            result += current_value;
        }
        past_value = current_value;

    }

    return result;
}
