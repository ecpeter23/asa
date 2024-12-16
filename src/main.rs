extern crate nom;
extern crate asalang;

use asalang::*;

fn main() -> Result<(), AsaErrorKind> {
  
  let tokens = lex("123");
  match program(tokens) {
    Ok((tokens, tree)) => {
      println!("{:?}", tokens);
      println!("Tree: {:#?}", tree);
      let mut interpreter = Interpreter::new();
      let result = interpreter.exec(&tree);
      println!("Interpreter Result: {:?}", result);
    },
    Err(e) => println!("Error: {:?}", e),
  }

    
  Ok(())
}
