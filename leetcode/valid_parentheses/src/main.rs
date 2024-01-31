

fn main() {
    let _s = String::from("()");
    let n = String::from("([])");

    let valid = is_valid(n);
    print!("Is valid: {:?}", valid);
}

pub fn is_valid(s: String) -> bool {
    //idea: make a substring from bracket to bracket and then make the check down below and then another substring, and so on

    if s.len() %2 != 0 {
        return false;
    }

    let mut vector:Vec<char> = Vec::new();
    let mut count_vector:Vec<i32> = vec![0;3];


    for c in s.chars(){
        match c {
            '(' => {
                vector.push('(');
                count_vector[0] += 1;
            },
            '[' => {
                vector.push('[');
                count_vector[1] += 1;
            },
            '{' => {
                vector.push('{');
                count_vector[2] += 1;
            },
            '}' => {
                if vector.last() != Some(&'{') {
                    return false;
                }else { 
                    vector.pop();
                }
                count_vector[2] -= 1;
            },
            ')' => {
                if vector.last() != Some(&'(') {
                    return false;
                }else {
                    vector.pop();
                }
                count_vector[0] -= 1;
            },
            ']' => {
                if vector.last() != Some(&'[') {
                    return false;
                }else {
                    vector.pop();
                }
                count_vector[1] -= 1;
            },
            _ => print!("Error"),
        }
    }

    if count_vector[0] == 0 && count_vector[1] == 0 && count_vector[2] == 0{
        return true;
    }

    false
}
