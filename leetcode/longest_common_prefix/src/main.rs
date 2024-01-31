fn main() {
    let strs = vec![
        "cir".to_string(),
        "car".to_string(),
      
    ];
    longest_common_prefix(strs);
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        let c = strs[0].chars().nth(0);
        if let Some(ch) = c {
            return ch.to_string();
        }
        return "".to_string();
    }

    let mut result = String::from("");

    let mut sorted_vector = strs.into_iter().collect::<Vec<_>>();
    sorted_vector.sort_by(|a, b| a.len().cmp(&b.len()));

    let shortest_element: &String = &sorted_vector[0];


    'outer: for (i, c_to_compare) in shortest_element.chars().enumerate() {
        let mut matched_elements = 0;
        for s in sorted_vector.iter().skip(1) {
            
            if s.chars().nth(i) == Some(c_to_compare) {
                matched_elements += 1;
            }
        }
        if matched_elements == sorted_vector.len() - 1 {
            result.push(c_to_compare)
        }else {
            break 'outer;
        }
    }

    return result;
}
