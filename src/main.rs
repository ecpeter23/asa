extern crate nom;
extern crate asalang;

use asalang::*;

fn main() -> Result<(), AsaErrorKind> {
  
  let tokens = lex(r#"
fn greet(name = "World") {
    return "Hello, " + name + "!";
}
return greet();
"#);
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
