extern crate nom;
extern crate asa;

use asa::*;
use std::env;
use std::fs;
use std::process;

fn main() -> Result<(), AsaErrorKind> {
  // Collect command line arguments
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: asa <filename.asa>");
    process::exit(1);
  }

  let filename = &args[1];
  // Read the file into a String
  let source = fs::read_to_string(filename)
    .map_err(|e| AsaErrorKind::Generic(format!("Could not read file {}: {}", filename, e)))?;

  // Lex the source
  let tokens = lex(&source);

  // Parse the tokens
  match program(tokens) {
    Ok((_, tree)) => {
      let mut interpreter = Interpreter::new();
      // First, interpret the entire AST to load all definitions (functions, variables).
      match interpreter.exec(&tree) {
        Ok(_top_level_result) => {
          // Now try to call main
          let main_id = Interpreter::hash_identifier(b"main");
          if let Ok(main_func) = interpreter.get_variable(main_id) {
            // Simulate calling main with no arguments
            match interpreter.call_function(main_func, &[]) {
              Ok(result) => {
                println!("Main returned: {:?}", result);
                Ok(())
              },
              Err(err) => {
                eprintln!("Runtime Error while calling main: {:?}", err);
                process::exit(1);
              }
            }
          } else {
            // If no main function is defined, just proceed
            println!("No main function found; execution completed.");
            Ok(())
          }
        },
        Err(err) => {
          eprintln!("Runtime Error: {:?}", err);
          process::exit(1);
        }
      }
    },
    Err(e) => {
      eprintln!("Parse Error: {:?}", e);
      process::exit(1);
    }
  }
}
