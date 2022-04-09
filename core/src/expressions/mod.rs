mod number;
mod add;
mod multiply;

use number::Number;
use add::Add;
use multiply::Multiply;

pub trait Expression {
    /// Check if the expression is reducible
    fn reducible(&self) -> bool;

    /// Convert the expression to string
    fn stringify(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub struct ExpressionInstance<T: Expression> {
    instance: T
}

impl<T: Expression> ExpressionInstance<T> {
    fn new(instance: T) -> Self {
        Self { instance }
    }

    fn to_string(&self) -> String {
        self.instance.stringify()
    }
}