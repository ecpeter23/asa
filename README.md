# ASA: A Tiny Language Lexer, Parser, and Interpreter

Welcome to the ASA project, a from-scratch lexer, parser, and interpreter for a small, custom programming language. This codebase demonstrates a comprehensive approach to language implementation—covering tokenization, parsing with [Nom](https://github.com/Geal/nom), building an Abstract Syntax Tree (AST), and interpreting it with semantics resembling a dynamically-typed language.

## Overview

This project includes the following major components:

1. **Lexer**: Converts raw source code (UTF-8 text) into a stream of tokens.
2. **Parser**: Uses [Nom](https://docs.rs/nom/) combinators to parse tokens into a rich AST representing the language constructs.
3. **Interpreter**: Evaluates the AST, handling variables, functions, conditionals, loops, arrays, and more. It supports a runtime environment with lexical scoping and built-in functions like `print` and `len`.

The code is meticulously structured to illustrate each stage of language processing. Nom is used for parsing tokens instead of the source directly, showcasing a two-phase design: first lexing into tokens, then parsing the tokens into a syntax tree.

## Key Features

- **Tokenization**:  
  A custom lexer scans the input string and produces `Token` structures with positional information. It detects comments, strings, identifiers, numbers, keywords, operators, and delimiters.

- **Abstract Syntax Tree (AST)**:  
  The parser constructs a strongly-typed `Node` enum, representing the language grammar:
  - **Expressions** (numbers, booleans, strings, identifiers, function calls, arrays, arithmetic, logical operations)
  - **Statements** (variable definitions, assignments, returns, if-expressions, while-loops, break/continue)
  - **Functions** (definitions, calls, arguments, returns)
  - **Complex Structures** (arrays, indexing, property access, method calls)

- **Grammar Highlights**:
  - **Functions**: `fn name(args) { ... }`
  - **Control Flow**: `if (cond) { ... } else if (cond2) { ... } else { ... }`, `while (cond) { ... }`
  - **Variables**: `let x = expression;`
  - **Arrays**: `[1, 2, 3]` with indexing `arr[index]` and methods `push`, `pop`, `insert`, `prepend`.
  - **Built-Ins**: `print(expression)`, `len(array_or_string)`, `main()` function handling.

- **Interpreter**:
  - Manages a call stack (`Frame`) for variables.
  - Evaluates nodes to `Value` variants (`Number`, `String`, `Array`, `Bool`, `Function`, `Identifier`).
  - Implements arithmetic (`+`, `-`, `*`, `/`, `%`, `^`) and logical (`&&`, `||`, `!`) operations, including string concatenation and boolean logic.
  - Supports runtime errors such as division by zero, type mismatches, and undefined functions/variables.
  - Allows user-defined functions and calling them with arguments, including optional default arguments.

- **Testing with a Turing Machine Example**:
  The included test program defines a `turingMachine` function that simulates a simple Turing machine operating on an array-based tape. This showcases:
  - Array manipulation (`push`, `prepend`).
  - Complex logic with `if/else` and `while` loops.
  - Mutation of variables passed into a function.
  - Runtime evaluation and `print`ing of final results.

**Example Input (file1.asa):**
```asa
fn turingMachine(tape, pos) {
    let state = 0;

    while (state != 2) {
        // Expand tape at boundaries
        let n = tape.length;
        if (pos < 0) {
            tape = tape.prepend(0); // Prepend 0 at the start
            pos = 0; // Reset position to 0 after expansion
        } else if (pos >= n) {
            tape = tape.push(0); // Append 0 at the end
        }

        if (state == 0) {
            if (tape[pos] == 0) {
                tape[pos] = 1;
                pos = pos + 1;
                state = 1;
            } else {
                tape[pos] = 0;
                pos = pos + 1;
                state = 0;
            }
        } else if (state == 1) {
            if (tape[pos] == 0) {
                tape[pos] = 1;
                pos = pos + 1;
                state = 2;
            } else {
                tape[pos] = 0;
                pos = pos - 1;
                state = 0;
            }
        }
    }

    return tape;
}

fn main() {
    let tape = [1, 1, 1, 0]; // Initial tape
    let headPos = 0; // Start position
    print("Initial tape: " + tape);
    tape = turingMachine(tape, headPos);
    print("Final tape: " + tape);
    return 0;
}
```
**Example Output:**
```bash
String("Initial tape: [Number(1), Number(1), Number(1), Number(0)]")
String("Final tape: [Number(0), Number(0), Number(0), Number(1), Number(1)]")
Main returned: Number(0)
```

## Code Structure
- **Lexer**:
    - TokenKind enum categorizes tokens (keywords, symbols, operators).
    - lex(input: &str) -> Tokens transforms source code into a vector of tokens.
- **Parser**:
    - Uses nom to define combinators that consume Tokens rather than raw strings.
    - Builds an AST of Node types that represent the language constructs.
    - program(tokens: Tokens) -> IResult<Tokens, Node> is the entry point for parsing a full source file.
- **Interpreter**:
    - The Interpreter struct manages a stack of frames (HashMap<u64, Value>) for variables.
    - exec(&Node) -> Result<Value,AsaErrorKind> recursively evaluates nodes.
- **Data Types & Error Handling**:
    - Value enum represents runtime values.
    - AsaErrorKind enumerates possible runtime errors (e.g., TypeMismatch, UndefinedFunction, ReturnSignal).
 
## Getting Started
To run asa code, just call the asa exectable with your .asa file as an argument:
```bash
./asa path/to/file/<file-name>.asa
```
If you run the included test program (as shown in the code snippet), you will see the described output, you can also browse the tests to see more specific unit tests.

## License
This project is provided under the MIT license.
