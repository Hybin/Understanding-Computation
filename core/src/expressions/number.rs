use std::fmt::{Display, Formatter, Result};

use super::Expression;

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: i32,
}

impl Number {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Display for Number {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "({})", self.to_string())
    }
}

impl Expression for Number {
    fn reducible(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{Number, Expression};

    #[test]
    fn test_number() {
        let number = Number::new(32);

        assert_eq!(
            number,
            Number {
                value: 32
            }
        );
    }

    #[test]
    fn test_number_to_string() {
        let number = Number::new(32);

        assert_eq!(number.to_string(), String::from("32"));
    }

    #[test]
    fn test_number_reducible() {
        let number = Number::new(32);

        assert_eq!(number.reducible(), false);
    }
}