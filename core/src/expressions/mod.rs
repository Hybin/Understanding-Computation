mod number;
mod add;
mod multiply;

use number::Number;
use add::Add;
use multiply::Multiply;

trait Expression {
    /// Check if the expression is reducible
    fn reducible(&self) -> bool;
}