fn extract_value(value: u16, offset: u16, width: u16) -> u16 {
    let mask = (1 << width) - 1;
    (value >> offset) & mask
}

pub fn extract_value_from_left(value: u16, position: u16, width: u16) -> u16 {
    let offset = u16::BITS as u16 - position - width;
    extract_value(value, offset, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_value() {
        let value = 0b1000010110010101_u16;
        let expected_result: u16 = 2137; // 0b100001011001
        let result = extract_value(value, 4, 12);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_extract_from_left() {
        let value = 0b1000010110010101_u16;
        let expected_result: u16 = 2137; // 0b100001011001
        let result = extract_value_from_left(value, 0, 12);
        assert_eq!(result, expected_result);
    }
}
