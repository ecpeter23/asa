
// You are free to add more error variants if you need them.

use crate::Value;

#[derive(Debug,PartialEq)]
pub enum AsaErrorKind {
  UndefinedFunction,
  VariableNotDefined(String),
  DivisionByZero,
  NumberOverflow,
  NumberUnderflow,
  TypeMismatch(String),
  Generic(String),
  BreakSignal,
  ContinueSignal,
  ReturnSignal(Value),
}