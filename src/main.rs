extern crate nom;
extern crate asalang;

use asalang::*;

fn main() -> Result<(), AsaErrorKind> {
  
  let tokens = lex(r#"
fn calculate(a, b) {
    let sum = a + b;
    let product = a * b;
    return sum + product; // e.g., for (5,3): (5+3)+(5*3) = 8+15 = 23
}
return calculate(5, 3);
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
