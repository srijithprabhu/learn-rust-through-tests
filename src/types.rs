// `NanoSecond` is a new name for `u64`.
type NanoSecond = u64;
type Inch = u64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_casting() {
        let expected:u8 = 255;
        let value:i8 = expected as i8;
        let actual = value as u8;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_types() {
        let value: u64 = 5;
        let ns:NanoSecond = value;
        let inches:Inch = value;
        assert_eq!(ns, inches);
    }
}
