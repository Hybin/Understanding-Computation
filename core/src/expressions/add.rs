use std::fmt::{Display, Formatter, Result};

use super::{Number, Expression};

#[derive(Debug, PartialEq)]
pub struct Add {
    left: Number,
    right: Number,
}

impl Add {
    pub fn new(left: Number, right: Number) -> Self {
        Self { left, right }
    }

    pub fn to_string(&self) -> String {
        format!("{} + {}", self.left.to_string(), self.right.to_string())
    }
}

impl Display for Add {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "({})", self.to_string())
    }
}

impl Expression for Add {
    fn reducible(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{Number, Add, Expression};
    
    #[test]
    fn test_add() {
        let left = Number::new(1);
        let right = Number::new(2);
        let expression = Add::new(left, right);

        assert_eq!(
            expression, 
            Add {
                left: Number {
                    value: 1
                },
                right: Number {
                    value: 2
                }
            }
        );
    }

    #[test]
    fn test_add_to_string() {
        let left = Number::new(1);
        let right = Number::new(2);
        let expression = Add::new(left, right);

        assert_eq!(expression.to_string(), String::from("1 + 2"));
    }

    #[test]
    fn test_add_reducible() {
        let left = Number::new(1);
        let right = Number::new(2);
        let expression = Add::new(left, right);

        assert_eq!(expression.reducible(), true);
    }
}