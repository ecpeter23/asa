# CSE262 - Programming Languages - Spring 2024

# Final Exam

⏰ **Due by: 5/12/2024 EOD**

## Ethics Contract

**FIRST**: Please read the following carefully:

- I am the sole author of the content within this exam unless otherwise cited.
- I am not an uncredited author of any content in any other exam.
- I will not dicsuss the exam until after the submission deadline.
- All resources I used (including text books and online references, websites, language models), are cited in this exam.
- I will not plagiarize someone else's work and turn it in as my own. If I use someone else's work in this exam, I will cite that work. Failure to cite work I used is plagiarism.
- I understand that acts of academic dishonesty may be penalized to the full extent allowed by the [Lehigh University Code of Conduct][0], including receiving a failing grade for the course. I recognize that I am responsible for understanding the provisions of the Lehigh University Code of Conduct as they relate to this academic exercise.


If you agree with the above, type your full name next to the pen emoji, along with the date. Your exam **will not be graded** without this assent.

---------------------------------------------
🖋️
---------------------------------------------

⚠️ **IMPORTANT:** When you are done, make your first commit with the commit message: `I, <your full name here>, agree to the ethics contract`.

⚠️ **IMPORTANT: As you are working on your midterm, commit your progress regularly.**

## Exam

In this final exam, you are going to design and implement a new feature for the Asa programming language we've been implementing all semester. You may use your Homework 5 solution as the basis for your exam. If you didn't fully complete Homework 5, you must first complete it using the [posted solutions](http://gitlab.cse.lehigh.edu/cse262-programming-languages/fall-2023/assignments/homework-5/-/tree/solutions). Copy and paste the solutions into your repository, and go through the commented lines of code to understand the implementation of the interpreter.

You can choose between three different features to implement, each with a different maximum possible score, or you may implement them all to achieve the maximum score of 100%:

1. comparison operators (C 75%)
2. if expressions or while loops (B 85%)
3. function define and call (A 93%)
4. the whole deal (100%)

You only have to do one of these. No matter which one you choose, you will have to write the grammar, implement the associated parser combinators, modify the interpreter to execute the new feature, and finally write tests to validate its functionality.

## Feature Semantics

### Conditional and Logic Operators

If you choose this version of the exam, you'll have to implement the following conditional operators:

1) Greater-than (`>`)
2) Less-than (`<`)
3) Greater-than or Equal-to (`>=`)
4) Less-than or Equal-to (`<=`)
5) Equal-to (`==`)
6) Not Equal-to (`!=`)
7) And (`&&`)
8) Or (`||`)

A conditional expressions is a number, boolean, identifier or expression; followed by one of the noted operators below; followed by another number, boolean, identifier or expression. Here are some examples of valid and invalid conditional expressions.

Valid conditional expressions:

```
1 > 2
true == x // if x is boolean
x > y     // if both x and y are boolean
```
Invalid conditional expressions:

```
1 > true    // invalid beause you can't compare number and boolean
5 - false   // invalid because you can't do math on number and boolean
```

Here's an example demonstrating operator precedence, which you must take into account in your implementation:

```
let x = 10;
let y = 5;
let result = x > y == true;
```

The expression `x > y == true` is evaluated as follows:

- `10 > 5` evaluates to `false`
- `false == true` evaluates to `false`

### Full math expressions

In the second part of the midterm you implemented a parser for math expressions that evaluate expressions using PEMDAS operator precedeence. For this cut of the exam, you must implement full math expressions.


- `((3 + 4) * 5 > 2^2) == true`
- `(10 / 2 + (7 - 3) == 2 * 3) == false`
- `(4^2 - 3 * 5 < 20 && 6 > 2) == true`
- `((8 - 2) * 3 != 5 * 2 || 10 > 2^3) == true`
- `((6 + 3) * 2 == 15 && (4 * 2) > 7)) == false`

### if-Expressions

If expressions have the following semantic parts:

- The test expression
- The pass expression
- The fail expression

The syntax of the if expression is the keyword "if" followed by an expression that must evaluate to a boolean value; a list of statements enclosed in curly braces follows; then the keyword "else"; then a second list of statements enclosed in curly braces. An optional "else if" section may follow the "if" section. Here are some examples of valid and invalid if-expressions.

You'll have to implement conditional operators for this part, but not logic operators.

Vaid if-expressions:

```
// One line form
if x > y {return false;} else {return true;}

// Multi line form
if true {
    return false;
} else {
    return true;
}

// Else-if is supported
if x > y {return 1;} else if x == y {return 2;} else {return 3}

// Result can be assigned to variable
let x = if true {return false;} else {return true;}
```

Invalid if-expressions:

```
// No return for true branch
if true {let x = 1;} else {return true;}

// Missing curly braces
if true {return false;} return true;

// Inconsistent return types
if true {return 1;} else {return true;}
```

Take the following semantic considerations into account when augmenting the interpreter to support if-expressions:

- Evaluation order: Condition is evaluated first, followed by the true/false branch.
- Type consistency: Ensure both expressions have compatible types.
- Short-circuit evaluation: If condition is true, false branch is not evaluated.
- Return value: The if-expression should return a single value that can be assigned to a variable or used in an expression.


### while-Loops

While loops have the following semantic parts:

- Condition: The expression that must evaluate to a boolean value.
- Body: A list of statements that are executed repeatedly as long as the condition is true.

The syntax of the while loop is the keyword "while" followed by an expression that must evaluate to a boolean value, followed by a block of statements enclosed in curly braces. The loop continues to execute the statements in the body as long as the condition evaluates to true.

Here are some examples of valid and invalid while loops:

```
// One line form
while x > y {x = x - 1;}

// Multi line form
while x > 0 {
    x = x - 1;
}

// Infinite loop (break is used to exit)
while true {break;}

// Loop that modifies and checks the condition in each iteration
let result = 0;
while result < 5 {
    result = result + 1;
}
```

Semantic Considerations for While Loops:

- Evaluation order: The condition is evaluated before the body is executed. If the condition is true, the body is executed. After the body executes, the condition is re-evaluated.

- Type consistency: Ensure the condition evaluates to a boolean value. The body of the loop can contain statements with varying types, but no value should be returned unless the loop is specifically designed to exit early via a return.

- Short-circuit evaluation: The condition is evaluated before each iteration. If the condition becomes false during any iteration, the loop terminates immediately, and the body is not executed further.

- Infinite loops: If the condition always evaluates to true and there is no way to break out of the loop (using break or another mechanism), the loop will run indefinitely. This needs to be handled carefully to avoid runaway execution.

### Function Define and Call

A function definition allows you to define a block of reusable code, which can then be invoked (called) with specific arguments to perform a task or compute a result. Functions encapsulate logic and enable modularity in programs.

#### Function Definition

A function is defined using the function keyword, followed by the function name, an optional list of parameters, and a block of statements enclosed in curly braces. The function may return a value, but it is not required to.

```
// Function definition without return value
fn foo(parameter1, parameter2) {
    // function body
    statement1;
    statement2;
    return result;
}

// Function definition with default parameters
fn foo(parameter1 = defaultValue) {
    // function body
    return parameter1;
}
```

Examples:

```
// Simple function without return value
function sayHello() {
    print("Hello, World!");
}

// Function with parameters and return value
function add(a, b) {
    return a + b;
}

// Function with default parameter value
function greet(name = "Guest") {
    return "Hello, " + name;
}

// Function with multiple statements
function calculate(a, b) {
    let sum = a + b;
    let product = a * b;
    return sum + product;
}
```

#### Function Call

A function call involves invoking the function by its name and passing arguments (if required) to it. The return value from the function (if any) can be used immediately or assigned to a variable.

```
// Function call without return value
functionName();

// Function call with arguments
functionName(argument1, argument2);

// Function call with assignment to a variable
let result = functionName(argument1, argument2);
```

Examples:

```
// Calling a function without arguments
sayHello();

// Calling a function with arguments
let sum = add(5, 3);

// Using the return value from a function
let greeting = greet("Alice");

// Function call with multiple arguments
let result = calculate(4, 5);
```

Semantic Considerations:

- Evaluation Order: The function body is executed when the function is called. The arguments are passed in the order they appear in the function call and are evaluated before the function body executes.

- Parameter Matching: When calling a function, the arguments passed should match the parameters in the function definition. If a parameter has a default value, it will be used when no argument is provided.

- Scope: Variables declared within a function are scoped locally to that function. They cannot be accessed outside of it unless explicitly returned or passed as arguments.

- Recursive Calls: Functions can call themselves (recursion), but care must be taken to avoid infinite recursion, which could result in stack overflow or program failure. Be sure to set a maximum size for your program stack to catch this.

### The whole deal 

Do cuts 1, 2, and 3. In addition, write an executable program called `asa`. This program will be called as follows

```
> asa file1.asa
```

The first arguement is a filename to Asa source code. The program will read the file into memory, lex, parse, and interpret it. Start any main() function that exists.

If you complete this cut of the exam successfully, you should have a Turing complete-ish programming language. Try writing a turing machine to prove the point, although if you don't you won't be penalized -- you've done enough.

## Part 1 - Grammar / Lexer

In this part, you must:

- Update the grammar to include the syntax for the cut you chose.
- Add any symbols to lexer to accomodate grammar updates.
- Define the meaning of the symbols you use to express your grammar. 

## Part 2 - Parser

In this part, you must:

1. Modify the existing parser to support the new grammar rules you defined in Part 1. This will involve adding new parser node variants.
2. Implement the necessary parser combinators for your chosen feature (conditional operators or if-expressions). 
3. Make parser update consisten with grammar update.

## Part 3 - Interpreter

In this part, you must:

1. Modify the existing interpreter to support the new parser nodes generated by your parser. 
2. Implement the necessary evaluation rules for your chosen feature (conditional operators or math expressions or if-expressions). 
4. Ensure your interpreter returns informative error messages for invalid expressions or runtime errors.

## Part 4 - Tests

In this part you must: 

1. Write at least 10 new tests for your implementation.
2. Update any previous tests to accomodate your new additions.
3. Make a continuous integration (CI) script runs the tests when code is pushed to Gitlab.

## Part 5 - Code Demo and Explanation

This is the oral portion of the exam. You will record an explanation for your interpreter and parser improvements which demonstrates their implementation and functionality. You don't have to show your face but you do have to record your voice (accommodations are available upon request). You should be sure to cover the following points in your discussion:

- Purpose and functionality: Explain what your code does and how it works.

- Grammar: Describe the EBNF grammar you created for your chosen feature and how it defines the syntax of conditional expressions or if-expressions.

- Parser: Explain how your code is organized and structured, discuss any design decisions you made. You should also discuss your coding style and any coding conventions you followed. Demonstrate that the parser matches the grammar by showing what it accepts and rejects.

- Interpreter: Explain how your interpreter has been modified to support the new feature, and discuss the evaluation rules you implemented. Show examples of your interpreter evaluating the chosen feature correctly, and explain how it handles invalid expressions or runtime errors.

If you didn't finish the exam in is entirety, explain how you attempted to solve it and where you got stuck. This will get you at least some points. 

You can use Zoom to do this, [here is a link](https://support.zoom.us/hc/en-us/articles/360059781332-Getting-started-with-recording) to some instructions. You don't have to record your face, only your voice and the screen. Go through the answer and explain how you arrived there. Your goal with this question is to convince me you know what you are talking about, so I want you to do this without reading a script or written answer. Just go through line by line and explain what the program does. When you are done, upload your recording to your Lehigh Drive and add a link below. 

**⚠️IMPORTANT: Make sure you give blanket permission to the link holder to view the file**

🎥 Paste Recording Link(s) Here:

## Submission

Please submit your completed exam, which should include:

1. A detailed description of the feature you chose to implement (conditional operators or if-expressions etc.).
2. The updated EBNF grammar for your chosen feature.
3. The updated parser and interpreter code, including any necessary modifications to support your chosen feature.
4. Test cases demonstrating the correct parsing and evaluation of your chosen feature.
5. A recording link with permission to view granted to the link holder.

- Only files under version control in your forked assignment repository will be graded. Local files left untracked on your computer will not be considered.

- Only code committed *and pushed* prior to the time of grading will be accepted. Locally committed but unpushed code will not be considered.

- Your assignment will be graded according to the [Programming Assignment Grading Rubric](https://drive.google.com/open?id=1V0nBt3Rz6uFMZ9mIaFioLF-48DFX0VdkbgRUDM_eIFk).

Your submission should be organized, well-commented, and easy to understand. Remember to document any assumptions you made during the implementation process, as well as any limitations of your solution. Your final exam will be graded on the correctness, completeness, and clarity of your submission.

## Works Cited

List all sources used during the exam here.
