use crate::parser::Node;
use std::collections::HashMap;
use crate::error::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Array(Vec<Value>),
  Number(i32),
  Bool(bool),
  Identifier(u64),
  Function {
    params: Vec<(u64, Option<Node>)>,
    body: Box<Node>,
  },
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

  pub fn hash_identifier(name: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish()
  }

  pub fn get_variable(&self, id: u64) -> Result<Value, AsaErrorKind> {
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

    if op_str == "+" {
      match (left, right) {
        (Value::String(lhs), Value::String(rhs)) => {
          return Ok(Value::String(lhs + &rhs));
        }
        (Value::String(lhs), r_val) => {
          let rhs = match r_val {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Identifier(id) => format!("<id:{}>", id),
            Value::Array(arr) => format!("{:?}", arr),
            Value::Function{..} => "<function>".to_string(),
          };
          return Ok(Value::String(lhs + &rhs));
        }
        (l_val, Value::String(rhs)) => {
          let lhs = match l_val {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Identifier(id) => format!("<id:{}>", id),
            Value::Array(arr) => format!("{:?}", arr),
            Value::Function{..} => "<function>".to_string(),
          };
          return Ok(Value::String(lhs + &rhs));
        }
        (Value::Number(l_num), Value::Number(r_num)) => {
          return Ok(Value::Number(l_num + r_num));
        }
        (Value::Bool(_)|Value::Number(_)|Value::Identifier(_)|Value::Array(_)|Value::Function{..}, Value::Bool(_)|Value::Number(_)|Value::Identifier(_)|Value::Array(_)|Value::Function{..}) => {
          return Err(AsaErrorKind::TypeMismatch("Invalid types for `+` operation".to_string()));
        }
      }
    }

    match (left, right, op_str) {
      (l_generic, r_generic, "==") => Ok(Value::Bool(l_generic == r_generic)),
      (l_generic, r_generic, "!=") => Ok(Value::Bool(l_generic != r_generic)),

      (Value::Number(l_num), Value::Number(r_num), _) => {
        let result = match op_str {
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

  pub fn call_function(&mut self, func_val: Value, arg_nodes: &[Node]) -> Result<Value, AsaErrorKind> {
    match func_val {
      Value::Function { params, body } => {
        if arg_nodes.len() > params.len() {
          return Err(AsaErrorKind::Generic(format!(
            "Function expected {} arguments, got {}",
            params.len(),
            arg_nodes.len()
          )));
        }

        // Create new frame
        self.stack.push(HashMap::new());

        for (i, (param_id, default_node)) in params.iter().enumerate() {
          let val = if i < arg_nodes.len() {
            // Argument provided by caller
            self.exec(&arg_nodes[i])?
          } else {
            // No argument provided, use default if available
            if let Some(def_node) = default_node {
              self.exec(def_node)?
            } else {
              return Err(AsaErrorKind::Generic(
                "Missing argument for parameter without default".to_string()
              ));
            }
          };
          self.set_variable(*param_id, val);
        }

        let result = match self.exec(&body) {
          Ok(val) => val,
          Err(AsaErrorKind::ReturnSignal(ret_val)) => ret_val,
          Err(e) => {
            self.stack.pop();
            return Err(e);
          }
        };

        self.stack.pop();
        Ok(result)
      }
      _ => Err(AsaErrorKind::Generic("Attempted to call a non-function value".to_string())),
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
      Node::FunctionDefine{name, children} => {
        // children[0] = FunctionArguments (now containing ArgumentDefine nodes)
        // children[1] = FunctionStatements
        let func_name_id = Self::hash_identifier(name);

        let mut params = Vec::new();
        if let Node::FunctionArguments { children: param_nodes } = &children[0] {
          for param_node in param_nodes {
            // param_node is Node::ArgumentDefine { children: [...] }
            if let Node::ArgumentDefine { children: arg_children } = param_node {
              // arg_children[0] should be an Identifier
              let (arg_id, default_node) = match arg_children.as_slice() {
                [Node::Identifier { value }] => {
                  let arg_id = Self::hash_identifier(value);
                  (arg_id, None)
                }
                [Node::Identifier { value }, default_expr] => {
                  let arg_id = Self::hash_identifier(value);
                  (arg_id, Some(default_expr.clone()))
                }
                _ => return Err(AsaErrorKind::Generic("Invalid parameter definition".to_string())),
              };
              params.push((arg_id, default_node));
            } else {
              return Err(AsaErrorKind::Generic("Invalid argument node in function definition".to_string()));
            }
          }
        }

        let body_node = &children[1];

        let func_value = Value::Function {
          params,
          body: Box::new(body_node.clone()),
        };

        self.set_variable(func_name_id, func_value);
        Ok(Value::Bool(true))
      },
      Node::FunctionArguments {..} => {
        // Should not be executed on its own
        Err(AsaErrorKind::Generic("FunctionArguments node should not be executed directly".to_string()))
      },
      Node::ArgumentDefine {..} => {
        // Should not be executed on its own
        Err(AsaErrorKind::Generic("ArgumentDefine node should not be executed directly".to_string()))
      },
      Node::FunctionStatements { children } => {
        // Run all children and return last (if no return encountered)
        let mut last = Value::Bool(true);
        for c in children {
          match self.exec(c) {
            Ok(val) => last = val,
            Err(AsaErrorKind::ReturnSignal(val)) => return Err(AsaErrorKind::ReturnSignal(val)),
            Err(e) => return Err(e),
          }
        }
        Ok(last)
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
      }

      Node::ArrayLiteral { children } => {
        let mut arr = Vec::new();
        for c in children {
          let val = self.exec(c)?;
          arr.push(val);
        }
        Ok(Value::Array(arr))
      }

      Node::IndexAccess { children } => {
        // children[0] = object, children[1] = index expression
        let object_val = self.exec(&children[0])?;
        let index_val = self.exec(&children[1])?;
        let idx = match index_val {
          Value::Number(n) => n,
          _ => return Err(AsaErrorKind::TypeMismatch("Index must be a number".to_string())),
        };
        match object_val {
          Value::String(s) => {
            let chars: Vec<char> = s.chars().collect();
            if idx < 0 || (idx as usize) >= chars.len() {
              return Err(AsaErrorKind::Generic("String index out of range".to_string()));
            }
            Ok(Value::String(chars[idx as usize].to_string()))
          }
          Value::Array(arr) => {
            if idx < 0 || (idx as usize) >= arr.len() {
              return Err(AsaErrorKind::Generic("Array index out of range".to_string()));
            }
            Ok(arr[idx as usize].clone())
          }
          _ => Err(AsaErrorKind::TypeMismatch("Cannot index this type".to_string())),
        }
      }

      Node::PropertyAccess { children } => {
        // children[0] = object, children[1] = property identifier
        let object_val = self.exec(&children[0])?;
        let property_node = &children[1];
        let property_name = if let Node::Identifier { value } = property_node {
          String::from_utf8_lossy(value).to_string()
        } else {
          return Err(AsaErrorKind::Generic("Invalid property name".to_string()));
        };

        match object_val {
          Value::String(s) => {
            match property_name.as_str() {
              "length" => Ok(Value::Number(s.chars().count() as i32)),

              _ => Err(AsaErrorKind::Generic("Unknown property on array".to_string()))
            }
          }
          Value::Array(arr) => {
            match property_name.as_str() {
              "length" => Ok(Value::Number(arr.len() as i32)),

              _ => Err(AsaErrorKind::Generic("Unknown property on array".to_string()))
            }
          }
          _ => Err(AsaErrorKind::Generic("Cannot access properties on this type".to_string())),
        }
      }
      Node::MethodCall { name, children } => {
        let object_val = self.exec(&children[0])?;
        let mut arg_values = Vec::new();
        for arg in children.iter().skip(1) {
          arg_values.push(self.exec(&arg)?);
        }

        let method_str = String::from_utf8_lossy(&name).to_string();

        match (object_val, &children[0]) {
          (Value::Array(mut arr), &Node::Identifier { ref value }) => {
            match method_str.as_str() {
              "push" => {
                if arg_values.len() != 1 {
                  return Err(AsaErrorKind::Generic("push expects exactly one argument".to_string()));
                }
                // Mutate the array
                arr.push(arg_values[0].clone());

                // Write back into the variable environment so `a` is updated
                let var_id = Self::hash_identifier(value);
                self.set_variable(var_id, Value::Array(arr.clone()));

                // Return the new length (or any other value you want)
                Ok(Value::Array(arr))
              }
              "pop" => {
                if arg_values.len() != 0 {
                  return Err(AsaErrorKind::Generic("pop expects no arguments".to_string()));
                }
                // Mutate the array
                let popped = arr.pop();

                // Write back into the variable environment so `a` is updated
                let var_id = Self::hash_identifier(value);
                self.set_variable(var_id, Value::Array(arr.clone()));

                // Return the popped value
                match popped {
                  Some(val) => Ok(val),
                  None => Err(AsaErrorKind::Generic("pop on empty array".to_string())),
                }
              }
              "insert" => {
                if arg_values.len() != 2 {
                  return Err(AsaErrorKind::Generic("insert expects exactly two arguments".to_string()));
                }
                let idx = match arg_values[0] {
                  Value::Number(n) => n,
                  _ => return Err(AsaErrorKind::TypeMismatch("Index must be a number".to_string())),
                };
                if idx < 0 || (idx as usize) > arr.len() {
                  return Err(AsaErrorKind::Generic("Array index out of range".to_string()));
                }
                arr.insert(idx as usize, arg_values[1].clone());

                // Write back into the variable environment so `a` is updated
                let var_id = Self::hash_identifier(value);
                self.set_variable(var_id, Value::Array(arr.clone()));

                Ok(Value::Array(arr))
              }
              "prepend" => {
                if arg_values.len() != 1 {
                  return Err(AsaErrorKind::Generic("prepend expects exactly one argument".to_string()));
                }
                arr.insert(0, arg_values[0].clone());

                // Write back into the variable environment so `a` is updated
                let var_id = Self::hash_identifier(value);
                self.set_variable(var_id, Value::Array(arr.clone()));

                Ok(Value::Array(arr))
              }
              _ => Err(AsaErrorKind::Generic(format!("Unknown array method: {}", method_str))),
            }
          },
          (Value::Array(_), _) => {
            // If the object is not a plain identifier (like a property access), handle accordingly
            Err(AsaErrorKind::Generic("Method call on non-identifier object not supported yet".to_string()))
          },
          _ => Err(AsaErrorKind::Generic("Method calls only implemented for arrays currently".to_string())),
        }
      }
      Node::FunctionCall{name, children} => {
        // children[0] = FunctionArguments
        let func_id = Self::hash_identifier(name);
        let func_name_str = String::from_utf8_lossy(name).to_string();

        // Built-in functions
        if func_name_str == "print" {
          // print(x)
          if let Node::FunctionArguments { children: args } = &children[0] {
            if args.len() != 1 {
              return Err(AsaErrorKind::Generic("print expects 1 argument".to_string()));
            }
            let val = self.exec(&args[0])?;
            println!("{:?}", val);
            return Ok(Value::Bool(true));
          } else {
            return Err(AsaErrorKind::Generic("Invalid print args".to_string()));
          }
        }

        if func_name_str == "len" {
          // len(x)
          return if let Node::FunctionArguments { children: args } = &children[0] {
            if args.len() != 1 {
              return Err(AsaErrorKind::Generic("len expects 1 argument".to_string()));
            }
            let val = self.exec(&args[0])?;
            match val {
              Value::String(s) => Ok(Value::Number(s.chars().count() as i32)),
              Value::Array(arr) => Ok(Value::Number(arr.len() as i32)),
              _ => Err(AsaErrorKind::Generic("len() not supported on this type".to_string()))
            }
          } else {
            Err(AsaErrorKind::Generic("Invalid len args".to_string()))
          }
        }

        let func_val = self.get_variable(func_id)?;
        // Extract arguments from children[0]
        let mut arg_nodes = Vec::new();
        if let Node::FunctionArguments { children: args } = &children[0] {
          for arg in args {
            if let Node::Expression { children: exprs } = arg {
              // Each expression node has one child which is the actual expression
              arg_nodes.push(exprs[0].clone());
            } else {
              arg_nodes.push(arg.clone());
            }
          }
        }
        self.call_function(func_val, &arg_nodes)
      },
      Node::Assignment{children} => {
        let val = self.exec(&children[1])?;

        match &children[0] {
          Node::Identifier { value } => {
            // Normal variable assignment
            let var_id = Self::hash_identifier(value);
            self.set_variable(var_id, val.clone());
            Ok(val)
          }

          Node::IndexAccess { children: idx_children } => {
            // idx_children[0] = the object (should be an identifier if we want to mutate)
            // idx_children[1] = index expression
            let idx = match self.exec(&idx_children[1])? {
              Value::Number(n) => n,
              _ => return Err(AsaErrorKind::TypeMismatch("Index must be a number".to_string())),
            };

            // If the base is an identifier, we can mutate the original variable
            if let Node::Identifier { value: array_name } = &idx_children[0] {
              let var_id = Self::hash_identifier(array_name);
              let mut arr = match self.get_variable(var_id)? {
                Value::Array(a) => a,
                _ => return Err(AsaErrorKind::TypeMismatch("Cannot index into non-array".to_string())),
              };
              if idx < 0 || idx as usize >= arr.len() {
                return Err(AsaErrorKind::Generic("Array index out of range".to_string()));
              }
              arr[idx as usize] = val.clone();
              // Store the modified array back into the variable
              self.set_variable(var_id, Value::Array(arr));
              Ok(val)
            } else {
              Err(AsaErrorKind::Generic("Left side of assignment must be a variable or currently unsupported complex expression".parse().unwrap()))
            }
          }

          Node::PropertyAccess {..} => {
            Err(AsaErrorKind::Generic("Property assignment not supported.".parse().unwrap()))
          }

          _ => Err(AsaErrorKind::Generic("Invalid lvalue in assignment.".parse().unwrap()))
        }
      }
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
