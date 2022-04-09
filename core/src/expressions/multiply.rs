use std::fmt::{Display, Formatter, Result};

use super::{Number, Expression, ExpressionInstance};

#[derive(Debug, PartialEq)]
pub struct Multiply<T: Expression> {
    left: ExpressionInstance<T>,
    right: ExpressionInstance<T>,
}

impl<T: Expression> Multiply<T> {
    pub fn new(
        left: ExpressionInstance<T>,
        right: ExpressionInstance<T>
    ) -> Self {
        Self { left, right }
    }
}

impl<T: Expression> Display for Multiply<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "({})", self.stringify())
    }
}

impl<T: Expression> Expression for Multiply<T> {
    fn reducible(&self) -> bool {
        true
    }

    fn stringify(&self) -> String {
        format!("{} * {}", self.left.to_string(), self.right.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{Number, Multiply, Expression, ExpressionInstance};

    #[test]
    fn test_multiply() {
        let left = ExpressionInstance::new(Number::new(1));
        let right = ExpressionInstance::new(Number::new(2));
        let expression = Multiply::new(left, right);

        assert_eq!(
            expression,
            Multiply {
                left: ExpressionInstance {
                    instance: Number {
                        value: 1
                    }
                },
                right: ExpressionInstance {
                    instance: Number {
                        value: 2
                    }
                }
            }
        );
    }

    #[test]
    fn test_multiply_to_string() {
        let left = ExpressionInstance::new(Number::new(1));
        let right = ExpressionInstance::new(Number::new(2));
        let expression = Multiply::new(left, right);

        assert_eq!(expression.stringify(), String::from("1 * 2"));
    }

    #[test]
    fn test_multiply_display() {
        let left = ExpressionInstance::new(Number::new(1));
        let right = ExpressionInstance::new(Number::new(2));
        let expression = Multiply::new(left, right);

        println!("{}", expression);
    }

    #[test]
    fn test_multiply_reducible() {
        let left = ExpressionInstance::new(Number::new(1));
        let right = ExpressionInstance::new(Number::new(2));
        let expression = Multiply::new(left, right);

        assert_eq!(expression.reducible(), true);
    }
}