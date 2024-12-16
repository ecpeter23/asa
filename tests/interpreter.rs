extern crate asalang;
extern crate nom;
use std::io::Write;

use asalang::*;
// use nom::IResult;

macro_rules! test_fragment {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),AsaErrorKind> {
      let tokens = lex($test);
      match program(tokens) {
        Ok((tokens, tree)) => {
          assert_eq!(tokens.is_done(), true); // Check that input token stream is fully parsed
          let mut interpreter = Interpreter::new();
          let result = interpreter.exec(&tree);
          std::io::stdout().flush();
          assert_eq!(result, $expected);
          Ok(())
        },
        Err(e) => Err(AsaErrorKind::Generic(format!("{:?}",e))),
      }
    }
  )
}

// Test interpreter fragments (no main function)
test_fragment!(interpreter_numeric, r#"123"#, Ok(Value::Number(123)));
test_fragment!(interpreter_string, r#""hello""#, Ok(Value::String("hello".to_string())));
test_fragment!(interpreter_bool_true, r#"true"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_bool_false, r#"false"#, Ok(Value::Bool(false)));
test_fragment!(interpreter_identifier, r#"x"#, Err(AsaErrorKind::UndefinedFunction));
test_fragment!(interpreter_variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test_fragment!(interpreter_variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test_fragment!(interpreter_variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_variable_string, r#"let string = "Hello";"#, Ok(Value::String("Hello".to_string())));
test_fragment!(interpreter_variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test_fragment!(interpreter_math, r#"1 + 1"#, Ok(Value::Number(2)));
test_fragment!(interpreter_math_no_space, r#"1-1"#, Ok(Value::Number(0)));
test_fragment!(interpreter_math_multiply, r#"2 + 4"#, Ok(Value::Number(6)));
test_fragment!(interpreter_assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test_fragment!(interpreter_define_full_program, r#"let x = 1 + 1; let y = 5 - 2; let z = x + y;"#, Ok(Value::Number(5)));
test_fragment!(interpreter_full_math, r#"let x = ((2 + 3) * 4 - 8) / (2 ^ 2);"#, Ok(Value::Number(3)));

test_fragment!(interpreter_unary_negative, r#"let x = -5;"#, Ok(Value::Number(-5)));
test_fragment!(interpreter_unary_positive, r#"let x = +5;"#, Ok(Value::Number(5)));
test_fragment!(interpreter_complex_expression, r#"((1 + 2) * (3 + 4))"#, Ok(Value::Number(21)));
test_fragment!(interpreter_exponentiation, r#"2 ^ 3"#, Ok(Value::Number(8)));
test_fragment!(interpreter_division_result, r#"0/1"#, Ok(Value::Number(0)));
test_fragment!(interpreter_reuse_variable, r#"let a = 10; let b = a + 5;"#, Ok(Value::Number(15)));
test_fragment!(interpreter_return_variable_value, r#"let a = 2; a"#, Ok(Value::Number(2)));
test_fragment!(interpreter_boolean_comparison, r#"(2 > 1)"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_variable_string_return, r#"let s = "SomeString"; s"#, Ok(Value::String("SomeString".to_string())));
test_fragment!(interpreter_equality_check, r#"let foo = true; let bar = false; let baz = foo == bar; return baz;"#, Ok(Value::Bool(false)));

test_fragment!(interpreter_comparison_check, r#"return !(1 > 0) && true;"#, Ok(Value::Bool(false)));
test_fragment!(conditional_simple_false, r#"1 > 2"#, Ok(Value::Bool(false)));
test_fragment!(conditional_simple_true, r#"2 > 1"#, Ok(Value::Bool(true)));
test_fragment!(conditional_boolean_equality_true, r#"true == true"#, Ok(Value::Bool(true)));
test_fragment!(conditional_boolean_equality_false, r#"true == false"#, Ok(Value::Bool(false)));
test_fragment!(conditional_variable_comparison, r#"let x = true; x == false"#, Ok(Value::Bool(false)));
test_fragment!(conditional_variable_compare_booleans, r#"let x = true; let y = false; x > y"#, Err(AsaErrorKind::Generic("Unknown operator for booleans".to_string())));

test_fragment!(invalid_comparison_number_boolean, r#"1 > true"#, Err(AsaErrorKind::TypeMismatch("Type error in binary expression: expected matching types".to_string())));
test_fragment!(invalid_math_number_boolean, r#"5 - false"#, Err(AsaErrorKind::TypeMismatch("Type error in binary expression: expected matching types".to_string())));

test_fragment!(operator_precedence_example, r#"let x = 10; let y = 5; let result = x > y == true; result"#, Ok(Value::Bool(true)));
test_fragment!(precedence_complex_1, r#"((3 + 4) * 5 > 2^2) == true"#, Ok(Value::Bool(true)));
test_fragment!(precedence_complex_2, r#"(10 / 2 + (7 - 3) == 2 * 3) == false"#, Ok(Value::Bool(true)));
test_fragment!(precedence_complex_3, r#"(4^2 - 3 * 5 < 20 && 6 > 2) == true"#, Ok(Value::Bool(true)));
test_fragment!(precedence_complex_4, r#"((8 - 2) * 3 != 5 * 2 || 10 > 2^3) == true"#, Ok(Value::Bool(true)));
test_fragment!(precedence_complex_5, r#"((6 + 3) * 2 == 15 && ((4 * 2) > 7)) == false"#, Ok(Value::Bool(true)));

test_fragment!(full_math_expression_1, r#"let x = ((2 + 3) * 4 - 8) / (2 ^ 2);"#, Ok(Value::Number(3)));
test_fragment!(full_math_expression_2, r#"((1 + 2) * (3 + 4))"#, Ok(Value::Number(21)));
test_fragment!(full_math_expression_3, r#"2 ^ 3"#, Ok(Value::Number(8)));
test_fragment!(full_math_expression_4, r#"0 / 1"#, Ok(Value::Number(0)));
test_fragment!(full_math_expression_5, r#"let a = 10; let b = a + 5;"#, Ok(Value::Number(15)));

test_fragment!(boolean_comparison_true, r#"(2 > 1)"#, Ok(Value::Bool(true)));
test_fragment!(boolean_comparison_false, r#"(1 > 2)"#, Ok(Value::Bool(false)));
test_fragment!(boolean_equality_check, r#"let foo = true; let bar = false; let baz = foo == bar; return baz;"#, Ok(Value::Bool(false)));
test_fragment!(boolean_not_operator, r#"return !(1 > 0) && true;"#, Ok(Value::Bool(false)));
test_fragment!(boolean_complex_expression, r#"(2 > 1 && 3 > 2) || (5 < 4)"#, Ok(Value::Bool(true)));

// ### If-Expressions
// Simple if/else returning boolean values
test_fragment!(if_simple_true, r#"if true { return false; } else { return true; }"#, Ok(Value::Bool(false)));
test_fragment!(if_simple_false, r#"if false { return false; } else { return true; }"#, Ok(Value::Bool(true)));

// Using if-expressions with variables
test_fragment!(if_variable_assign_true, r#"let x = if true { return false; } else { return true; }; return x;"#, Ok(Value::Bool(false)));
test_fragment!(if_variable_assign_false, r#"let x = if false { return false; } else { return true; }; return x;"#, Ok(Value::Bool(true)));

// Chained else-if
test_fragment!(if_else_if, r#"let x = 5; if x > 5 { return 1; } else if x == 5 { return 2; } else { return 3; }"#, Ok(Value::Number(2)));

// Double if statement (from the provided examples)
test_fragment!(double_if_statement, r#"let x = 0; let y = 0; if (x < 10) { if (y < 5) { y = 5; } x = 10; } return x + y;"#, Ok(Value::Number(15)));

// ### While-Loops
// Simple while loop
test_fragment!(while_loop_decrement, r#"let x = 5; while (x > 0) { x = x - 1; } return x;"#, Ok(Value::Number(0)));

// Nested while loop (from the provided examples)
test_fragment!(double_while_loop, r#"let x = 0; let y = 0; while (x < 10) { while (y < 5) { y = y + 1; } x = x + 1; } return x + y;"#, Ok(Value::Number(15)));

// While loop with break
test_fragment!(while_with_break, r#"let x = 0; while true { x = x + 1; if x > 5 {break;} } return x;"#, Ok(Value::Number(6)));

// While loop condition checking each iteration
test_fragment!(while_condition_update, r#"
        let result = 0;
        while (result < 5) {
            result = result + 1;
        }
        return result;
    "#, Ok(Value::Number(5)));
