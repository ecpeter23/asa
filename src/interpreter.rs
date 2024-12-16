use crate::parser::Node;
use std::collections::HashMap;
use crate::error::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
  Identifier(u64),
}

type Frame = HashMap<u64, Value>;

#[derive(Debug)]
pub struct Interpreter {
  stack: Vec<Frame>,
}

impl Interpreter {
  pub fn new() -> Interpreter {
    let mut stack = Vec::new();
    stack.push(HashMap::new());
    Interpreter {
      stack,
    }
  }

  fn hash_identifier(name: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish()
  }

  fn get_variable(&self, id: u64) -> Result<Value, AsaErrorKind> {
    for frame in self.stack.iter().rev() {
      if let Some(val) = frame.get(&id) {
        return Ok(val.clone());
      }
    }
    Err(AsaErrorKind::UndefinedFunction)
  }

  fn set_variable(&mut self, id: u64, val: Value) {
    if let Some(frame) = self.stack.last_mut() {
      frame.insert(id, val);
    }
  }

  fn eval_number(value: &[u8]) -> i32 {
    let s = String::from_utf8_lossy(value);
    s.parse::<i32>().unwrap()
  }

  fn eval_binary_op(&self, op: &[u8], left: Value, right: Value) -> Result<Value, AsaErrorKind> {
    let op_str = std::str::from_utf8(op)
      .map_err(|_| AsaErrorKind::Generic("Invalid UTF-8 in operator".to_string()))?;

    match (left, right, op_str) {
      (l_generic, r_generic, "==") => Ok(Value::Bool(l_generic == r_generic)),
      (l_generic, r_generic, "!=") => Ok(Value::Bool(l_generic != r_generic)),

      (Value::Number(l_num), Value::Number(r_num), _) => {
        let result = match op_str {
          "+" => Value::Number(l_num + r_num),
          "-" => Value::Number(l_num - r_num),
          "*" => Value::Number(l_num * r_num),
          "%" => {
            if r_num == 0 {
              return Err(AsaErrorKind::Generic("Modulo by zero".to_string()));
            }
            Value::Number(l_num % r_num)
          },
          "/" => {
            if r_num == 0 {
              return Err(AsaErrorKind::Generic("Division by zero".to_string()));
            }
            Value::Number(l_num / r_num)
          },
          "^" => {
            let val = i32::pow(l_num, r_num as u32);
            Value::Number(val)
          }
          "<" => Value::Bool(l_num < r_num),
          ">" => Value::Bool(l_num > r_num),
          "<=" => Value::Bool(l_num <= r_num),
          ">=" => Value::Bool(l_num >= r_num),
          _ => return Err(AsaErrorKind::Generic("Unknown operator".to_string())),
        };
        Ok(result)
      }
      (Value::Bool(l_bool), Value::Bool(r_bool), _) => {
        let result = match op_str {
          "&&" => Value::Bool(l_bool && r_bool),
          "||" => Value::Bool(l_bool || r_bool),
          _ => return Err(AsaErrorKind::Generic("Unknown operator for booleans".to_string())),
        };
        Ok(result)
      }
      _ => Err(AsaErrorKind::TypeMismatch(
        "Type error in binary expression: expected matching types".to_string(),
      )),
    }
  }

  fn eval_unary_op(&self, op: &[u8], val: Value) -> Result<Value, AsaErrorKind> {
    let op_str = std::str::from_utf8(op).unwrap();
    match (op_str, val) {
      ("+", Value::Number(n)) => Ok(Value::Number(n)),
      ("-", Value::Number(n)) => Ok(Value::Number(-n)),
      ("!", Value::Number(n)) => Ok(Value::Number(!n)),
      ("!", Value::Bool(b)) => Ok(Value::Bool(!b)),
      _ => Err(AsaErrorKind::Generic("Type error in unary expression".to_string())),
    }
  }

  pub fn exec(&mut self, node: &Node) -> Result<Value,AsaErrorKind> {
    match node {
      Node::Program{children} => {
        let mut last = Value::Bool(true); // default if empty
        for n in children {
          let val = match self.exec(n) {
            Ok(val) => val,
            Err(AsaErrorKind::ReturnSignal(val)) => {
              // Stop executing further and return this value immediately
              return Ok(val);
            },
            Err(e) => return Err(e),
          };
          last = val;
        }
        Ok(last)
      }

      Node::Expression{children} => {
        self.exec(&children[0])
      }
      Node::Number{value} => {
        Ok(Value::Number(Self::eval_number(value)))
      }
      Node::String{value} => {
        Ok(Value::String(String::from_utf8_lossy(value).to_string()))
      }
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      }
      Node::Identifier{value} => {
        let id = Self::hash_identifier(value);
        self.get_variable(id)
      },
      Node::VariableDefine{children} => {
        // children[0] = identifier
        // children[1] = expression
        if let Node::Identifier{value} = &children[0] {
          let var_id = Self::hash_identifier(value);
          let val = self.exec(&children[1])?;
          self.set_variable(var_id, val.clone());
          Ok(val)
        } else {
          Err(AsaErrorKind::Generic("Invalid variable define".to_string()))
        }
      },
      Node::Statement{children} => {
        let mut last = Value::Bool(true);
        for c in children {
          last = self.exec(c)?;
        }
        Ok(last)
      }
      Node::BinaryExpression{name, children} => {
        let left_val = self.exec(&children[0])?;
        let right_val = self.exec(&children[1])?;
        self.eval_binary_op(name, left_val, right_val)
      },
      Node::UnaryExpression{name, children} => {
        let val = self.exec(&children[0])?;
        self.eval_unary_op(name, val)
      },
      Node::FunctionDefine{..} => {
        // Not tested
        Err(AsaErrorKind::Generic("Function define not implemented".to_string()))
      },
      Node::FunctionArguments{..} => {
        // Just return Null or something if encountered directly
        Ok(Value::Bool(true))
      },
      Node::FunctionStatements{..} => {
        // Run all children and return last
        Err(AsaErrorKind::Generic("Function statements not implemented".to_string()))
      },
      Node::IfExpression { children } => {
        // children layout:
        // [if_condition, if_block, else_if_condition, else_if_block, ..., else_block(optional)]
        //
        // Conditions and blocks come in pairs. For each condition-block pair:
        // - even index: condition
        // - odd index: block
        //
        // If there's an extra child at the end (odd number of children overall),
        // that last child is the else block.

        let mut index = 0;
        while index < children.len() - 1 {
          let condition_value = self.exec(&children[index])?;
          match condition_value {
            Value::Bool(true) => {
              // Condition matched: execute this block
              return self.exec(&children[index + 1]);
            },
            Value::Bool(false) => {
              // Condition not met, move to next pair
            },
            _ => {
              return Err(AsaErrorKind::TypeMismatch(
                "If/Else-If condition must be boolean".to_string()
              ));
            }
          }
          index += 2;
        }

        // If we get here, none of the conditions were true.
        // Check if there's an else block.
        if children.len() % 2 == 1 {
          // There's an extra child at the end, which is the else block
          self.exec(&children[children.len() - 1])
        } else {
          // No else block - return a default value.
          Ok(Value::Bool(true))
        }
      }
      Node::WhileLoop{children} => {
        // children[0] = condition
        // children[1] = body block

        loop {
          let condition_value = self.exec(&children[0])?;
          match condition_value {
            Value::Bool(true) => {
              // Condition is true, execute the body block
              match self.exec(&children[1]) {
                Ok(_) => {
                  // Body executed successfully with no break or continue, loop again
                },
                Err(AsaErrorKind::BreakSignal) => {
                  // Break out of the loop
                  break Ok(Value::Bool(true));
                },
                Err(AsaErrorKind::ContinueSignal) => {
                  // Skip to next iteration (re-check condition)
                  continue;
                },
                Err(e) => {
                  // Other error, propagate upwards
                  return Err(e);
                }
              }
            },
            Value::Bool(false) => {
              // Condition is false, stop looping and return default value
              break Ok(Value::Bool(true));
            },
            _ => {
              // If the condition isn't boolean, return a type error
              return Err(AsaErrorKind::TypeMismatch(
                "While condition must be boolean".to_string()
              ));
            }
          }
        }
      },
      Node::FunctionCall{..} => {
        // Not tested
        Err(AsaErrorKind::Generic("Function call not implemented".to_string()))
      },
      Node::Assignment{children} => {
        // Not tested, but easy to implement:
        // children[0] = identifier
        // children[1] = expression
        if let Node::Identifier{value} = &children[0] {
          let var_id = Self::hash_identifier(value);
          let val = self.exec(&children[1])?;
          // if variable not defined previously, error?
          // Let's assume it must exist, else error:
          if !self.stack.last().unwrap().contains_key(&var_id) {
            return Err(AsaErrorKind::UndefinedFunction);
          }
          self.set_variable(var_id, val.clone());
          Ok(val)
        } else {
          Err(AsaErrorKind::Generic("Invalid assignment".to_string()))
        }
      },
      Node::FunctionReturn{children} => {
        // Not tested
        let val = self.exec(&children[0])?;
        Err(AsaErrorKind::ReturnSignal(val))
      },
      Node::Null => {
        Ok(Value::Bool(true))
      },
      Node::Block{children} => {
        let mut last = Value::Bool(true);
        for c in children {
          last = self.exec(c)?;
        }
        Ok(last)
      },
      Node::Break => {
        // Not tested
        Err(AsaErrorKind::BreakSignal)
      },
      Node::Continue => {
        // Not tested
        Err(AsaErrorKind::ContinueSignal)
      },
    }
  }
}
