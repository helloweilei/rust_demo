#[allow(dead_code)]
pub fn ip_to_num(ip_str: &str) -> Result<u32, &str> {
    let tokens: Vec<u32> = ip_str
        .split('#')
        .map(|str_segment| str_segment.parse::<u32>().unwrap())
        .collect();
    let valid_range = 0..=255;
    if
        tokens.len() != 4 ||
        !(0..=128).contains(&tokens[0]) ||
        !valid_range.contains(&tokens[1]) ||
        !valid_range.contains(&tokens[2]) ||
        !valid_range.contains(&tokens[3])
    {
        return Err("Invalid input");
    }
    let mut result = 0u32;
    let len = tokens.len();
    for (index, segment) in tokens.iter().enumerate() {
        result |= segment << (8 * (len - index - 1));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::ip_to_num;
    #[test]
    fn get_valid_num_for_ip() {
        assert_eq!(1684340997, ip_to_num("100#101#1#5").unwrap());
    }
}
