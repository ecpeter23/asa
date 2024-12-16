extern crate asalang;
extern crate nom;
use std::io::Write;

use asalang::*;
use nom::IResult;

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
