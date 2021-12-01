use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}



#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_and_into() {
        let expected = Number { value: 30};
        let num:Number = Number::from(30);
        assert_eq!(expected.value, num.value);

        let value = 30;
        let num:Number = value.into();
        assert_eq!(expected.value, num.value);
    }

    #[test]
    fn test_try_from_and_into() {
        // TryFrom

        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        // TryInto

        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_string_parse() {
        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();

        assert_eq!(5, parsed);
        assert_eq!(10, turbo_parsed);
    }
}
