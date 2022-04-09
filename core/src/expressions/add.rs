use std::fmt::{Display, Formatter, Result};

use super::{Number, Expression, ExpressionInstance};

#[derive(Debug, PartialEq)]
pub struct Add<T: Expression> {
    left: ExpressionInstance<T>,
    right: ExpressionInstance<T>,
}

impl<T: Expression> Add<T> {
    pub fn new(
        left: ExpressionInstance<T>,
        right: ExpressionInstance<T>
    ) -> Self {
        Self { left, right }
    }
}

impl<T: Expression> Display for Add<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "({})", self.stringify())
    }
}

impl<T: Expression> Expression for Add<T> {
    fn reducible(&self) -> bool {
        true
    }

    fn stringify(&self) -> String {
        format!("{} + {}", self.left.to_string(), self.right.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{Number, Add, Expression, ExpressionInstance};
    
    #[test]
    fn test_add() {
        let left = ExpressionInstance::new(Number::new(1));
        let right = ExpressionInstance::new(Number::new(2));
        let expression = Add::new(left, right);

        assert_eq!(
            expression, 
            Add {
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
    fn test_add_to_string() {
        let left = ExpressionInstance::new(Number::new(1));
        let right = ExpressionInstance::new(Number::new(2));
        let expression = Add::new(left, right);

        assert_eq!(expression.stringify(), String::from("1 + 2"));
    }

    #[test]
    fn test_add_reducible() {
        let left = ExpressionInstance::new(Number::new(1));
        let right = ExpressionInstance::new(Number::new(2));
        let expression = Add::new(left, right);

        assert_eq!(expression.reducible(), true);
    }
}