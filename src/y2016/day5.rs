static ID: &str = "reyedfim";

fn md5s() -> impl Iterator<Item = String> {
    std::iter::successors(Some(0), |i| Some(i + 1))
        .map(|i| md5::compute(format!("{}{}", ID, i)))
        .map(|hash| format!("{:x}", hash))
        .filter(|hex| hex.starts_with("00000"))
}

pub fn p1() -> String {
    let mut result = "".to_owned();
    for hex in md5s() {
        result.push_str(&hex.chars().nth(5).unwrap().to_string());
        if result.len() >= 8 {
            return result;
        }
    }
    panic!()
}

pub fn p2() -> String {
    let mut result: String = "-".repeat(8).to_owned();
    let mut count = 0;
    for hex in md5s() {
        let index = hex.chars().nth(5).unwrap().to_digit(16).unwrap() as usize;
        if index > 7 || result.chars().nth(index).unwrap() != '-' {
            continue;
        }
        result.replace_range(index..index + 1, &hex.chars().nth(6).unwrap().to_string());
        count += 1;
        if count >= 8 {
            return result;
        }
    }
    panic!()
}
