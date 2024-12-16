use asalang::*;
use asalang::Node::*;

macro_rules! test {
  ($func:ident, $input:tt, $combinator:tt, $test:expr) => (
    #[test]
    fn $func() -> Result<(),()> {
      let source = $input;
      let tokens = lex(source);
      let parse_result = $combinator(tokens);
      match parse_result {
        Ok((tokens,tree)) => {
          assert_eq!(tokens.is_done(),true);
          assert_eq!(tree,$test)
        },
        _ => {assert!(false)},
      }
      Ok(())
    }
  )
}
// test name, test string, combinator,  expected result
test!(parser_ident, r#"hello"#, identifier, Identifier{value: vec![104, 101, 108, 108, 111]});
test!(parser_number, r#"123"#, number, Number{value: vec![49, 50, 51]});
test!(parser_bool, r#"true"#, boolean, Bool{value: true});
test!(parser_string, r#""hello""#, string, String{value: vec![104, 101, 108, 108, 111]});
test!(parser_function_call, r#"foo()"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
  FunctionArguments{ children: vec![
  ]}
]});
test!(parser_function_call_one_arg, r#"foo(a)"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
  FunctionArguments{ children: vec![
    Expression { children: vec![Identifier { value: vec![97] }]}
  ]}
]});
test!(parser_variable_define_number, r#"let a = 123"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![Number{value: vec![49, 50, 51]}]}
]});
test!(parser_variable_define_bool, r#"let a = true"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![Bool{value: true}]}
]});
test!(parser_math_expr, r#"1+1"#, addition, BinaryExpression {name: vec![43], children: vec![
      Number{value: vec![49]},
      Number{value: vec![49]}
    ]});
test!(parser_variable_define_math_expr, r#"let a = 1 + 1"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![
    BinaryExpression {name: vec![43], children: vec![
      Number{value: vec![49]},
      Number{value: vec![49]}
    ]}
  ]}
]});
test!(parser_variable_function_call, r#"let a = foo()"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![
    FunctionCall{name: vec![102, 111, 111], children: vec![
      FunctionArguments{ children: vec![
      ]}
    ]}
  ]}
]});
test!(parser_function_define, r#"fn a(){return 1;}"#, function_define, FunctionDefine{
  name: vec![97],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![
      FunctionReturn{ children: vec![ 
        Expression { children: vec![Number{value: vec![49]}]}
      ]}
    ]}
  ]
});
test!(parser_function_define_multi_statements, r#"fn add(a,b){let x=a+b;return x;}"#, function_define, FunctionDefine{
  name: vec![97, 100, 100],
  children: vec![
    FunctionArguments{ children: vec![
      Expression { children: vec![Identifier { value: vec![97] }] },
      Expression { children: vec![Identifier { value: vec![98] }] },
    ] },
    FunctionStatements{ children: vec![
      VariableDefine{children: vec![
        Identifier { value: vec![120] },
        Expression { children: vec![
          BinaryExpression {name: vec![43], children: vec![
            Identifier{value: vec![97]},
            Identifier{value: vec![98]}
          ]}
        ]}
      ]},
      FunctionReturn{ children: vec![ 
        Expression { children: vec![Identifier{value: vec![120] }]}
      ]}
    ]}
  ]
});
// test!(test_ident, r#"hello"#, identifier, Identifier{value: vec![104, 101, 108, 108, 111]});
// test!(test_number, r#"123"#, number, Number{value: vec![49, 50, 51]});
// test!(test_bool, r#"true"#, boolean, Bool{value: true});
// test!(test_string, r#""hello""#, string, String{value: vec![104, 101, 108, 108, 111]});
// test!(test_function_call, r#"foo()"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
//   FunctionArguments{ children: vec![
//   ]}
// ]});
// test!(test_function_call_one_arg, r#"foo(a)"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
//   FunctionArguments{ children: vec![
//     Expression { children: vec![Identifier { value: vec![97] }]}
//   ]}
// ]});
// test!(test_variable_define_number, r#"let a = 123;"#, variable_define, VariableDefine{children: vec![
//   Identifier { value: vec![97] },
//   Expression { children: vec![Number{value: vec![49, 50, 51] }]}
// ]});
// test!(test_variable_define_bool, r#"let a = true;"#, variable_define, VariableDefine{children: vec![
//   Identifier { value: vec![97] },
//   Expression { children: vec![Bool{value: true}]}
// ]});
// test!(test_math_expr, r#"1+1;"#, addition, BinaryExpression {name: vec![43], children: vec![
//   Number{value: vec![49]},
//   Number{value: vec![49]}
// ]});
// test!(test_variable_define_math_expr, r#"let a = 1 + 1;"#, variable_define, VariableDefine{children: vec![
//   Identifier { value: vec![97] },
//   Expression { children: vec![
//     BinaryExpression {name: vec![43], children: vec![
//       Number{value: vec![49]},
//       Number{value: vec![49]}
//     ]}
//   ]}
// ]});
// test!(test_variable_function_call, r#"let a = foo();"#, variable_define, VariableDefine{children: vec![
//   Identifier { value: vec![97] },
//   Expression { children: vec![
//     FunctionCall{name: vec![102, 111, 111], children: vec![
//       FunctionArguments{ children: vec![
//       ]}
//     ]}
//   ]}
// ]});
// test!(test_function_define, r#"fn a(){return 1;}"#, function_define, FunctionDefine{
//   name: vec![97],
//   children: vec![
//     FunctionArguments{ children: vec![] },
//     FunctionStatements{ children: vec![
//       FunctionReturn{ children: vec![
//         Expression { children: vec![Number{value: vec![49] }]}
//       ]}
//     ]}
//   ]
// });
// test!(test_function_define_multi_statements, r#"fn add(a,b){let x=a+b;return x;}"#, function_define, FunctionDefine{
//   name: vec![97, 100, 100],
//   children: vec![
//     FunctionArguments{ children: vec![
//       Expression { children: vec![Identifier { value: vec![97] }] },
//       Expression { children: vec![Identifier { value: vec![98] }] },
//     ] },
//     FunctionStatements{ children: vec![
//       VariableDefine{children: vec![
//         Identifier { value: vec![120] },
//         Expression { children: vec![
//           BinaryExpression {name: vec![43], children: vec![
//             Identifier{value: vec![97]},
//             Identifier{value: vec![98]}
//           ]}
//         ]}
//       ]},
//       FunctionReturn{ children: vec![
//         Expression { children: vec![Identifier{value: vec![120] }]}
//       ]}
//     ]}
//   ]
// });
//
// test!(test_function_call_multiple_args, r#"foo(a, b, c)"#, function_call, FunctionCall {
//   name: vec![102, 111, 111], // "foo"
//   children: vec![
//     FunctionArguments { children: vec![
//       Expression { children: vec![Identifier { value: vec![97] } ] }, // "a"
//       Expression { children: vec![Identifier { value: vec![98] } ] }, // "b"
//       Expression { children: vec![Identifier { value: vec![99] } ] }, // "c"
//     ]}
//   ]
// });
//
// test!(test_nested_function_call, r#"foo(bar())"#, function_call, FunctionCall {
//   name: vec![102, 111, 111], // "foo"
//   children: vec![
//     FunctionArguments { children: vec![
//       Expression { children: vec![
//         FunctionCall {
//           name: vec![98, 97, 114], // "bar"
//           children: vec![
//             FunctionArguments { children: vec![] }
//           ]
//         }
//       ] }
//     ]}
//   ]
// });
//
// test!(test_string_with_spaces, r#""hello world""#, string, String {
//   value: vec![104, 101, 108, 108, 111, 119, 111, 114, 108, 100] // "hello world"
// });
//
// test!(test_math_expression_multiple_ops, r#"1 - 2 + 3"#, addition, BinaryExpression {
//   name: vec![43], // "add" (assuming "add" represents the math expression)
//   children: vec![
//     BinaryExpression { name: vec![45], children: vec![
//       Number { value: vec![49] }, // "1"
//       Number { value: vec![50] }, // "2"
//     ]},
//     Number { value: vec![51] }, // "3"
//   ]
// });
//
// test!(test_function_define_multiple_args, r#"fn add(a, b, c){return a + b + c;}"#, function_define, FunctionDefine {
//   name: vec![97, 100, 100], // "add"
//   children: vec![
//     FunctionArguments { children: vec![
//       Expression { children: vec![Identifier { value: vec![97] } ] }, // "a"
//       Expression { children: vec![Identifier { value: vec![98] } ] }, // "b"
//       Expression { children: vec![Identifier { value: vec![99] } ] }, // "c"
//     ] },
//     FunctionStatements { children: vec![
//       FunctionReturn { children: vec![
//         Expression { children: vec![
//           BinaryExpression {
//             name: vec![43], // "add"
//             children: vec![
//               BinaryExpression {
//                 name: vec![43], // "add"
//                 children: vec![
//                   Identifier { value: vec![97] }, // "a"
//                   Identifier { value: vec![98] }, // "b"
//                 ]
//               },
//               Identifier { value: vec![99] }, // "c"
//             ]
//           }
//         ] }
//       ] }
//     ]}
//   ]
// });
//
// test!(test_variable_define_string, r#"let greeting = "hello";"#, variable_define, VariableDefine {
//   children: vec![
//     Identifier { value: vec![103, 114, 101, 101, 116, 105, 110, 103] }, // "greeting"
//     Expression { children: vec![
//       String { value: vec![104, 101, 108, 108, 111] } // "hello"
//     ]}
//   ]
// });
//
// test!(test_function_return_function_call, r#"return foo();"#, function_return, FunctionReturn {
//   children: vec![
//     FunctionCall {
//         name: vec![102, 111, 111], // "foo"
//         children: vec![
//           FunctionArguments { children: vec![] }
//         ]
//     }
//   ]
// });
//
// test!(test_program_multiple_statements, r#"let a = 1; let b = 2; return a + b;"#, program, Program {
//   children: vec![
//     VariableDefine { children: vec![
//         Identifier { value: vec![97] }, // "a"
//         Expression { children: vec![Number { value: vec![49] } ] } // "1"
//       ]},
//     VariableDefine { children: vec![
//         Identifier { value: vec![98] }, // "b"
//         Expression { children: vec![Number { value: vec![50] } ] } // "2"
//       ]},
//     FunctionReturn { children: vec![
//         Expression { children: vec![
//           BinaryExpression {
//             name: vec![43], // "add"
//             children: vec![
//               Identifier { value: vec![97] }, // "a"
//               Identifier { value: vec![98] }, // "b"
//             ]
//           }
//         ] }
//       ] }
//   ]
// });
//
// test!(test_function_define_with_whitespace, r#"fn   add ( a , b ) { return a + b ; }"#, function_define, FunctionDefine {
//   name: vec![97, 100, 100], // "add"
//   children: vec![
//     FunctionArguments { children: vec![
//       Expression { children: vec![Identifier { value: vec![97] } ] }, // "a"
//       Expression { children: vec![Identifier { value: vec![98] } ] }, // "b"
//     ] },
//     FunctionStatements { children: vec![
//       FunctionReturn { children: vec![
//         Expression { children: vec![
//           BinaryExpression {
//             name: vec![43], // "add"
//             children: vec![
//               Identifier { value: vec![97] }, // "a"
//               Identifier { value: vec![98] }, // "b"
//             ]
//           }
//         ] }
//       ] }
//     ]}
//   ]
// });
//
// test!(test_program_with_variables_and_functions, r#"let a = 10; fn add(b, c) { return b + c; } let result = add(a, 20);"#, program, Program {
//   children: vec![
//     VariableDefine { children: vec![
//       Identifier { value: vec![97] }, // "a"
//       Expression { children: vec![Number { value: vec![49, 48] } ] } // "10"
//     ]},
//     FunctionDefine {
//       name: vec![97, 100, 100], // "add"
//       children: vec![
//         FunctionArguments { children: vec![
//           Expression { children: vec![Identifier { value: vec![98] } ] }, // "b"
//           Expression { children: vec![Identifier { value: vec![99] } ] }, // "c"
//         ] },
//         FunctionStatements { children: vec![
//           FunctionReturn { children: vec![
//             Expression { children: vec![
//               BinaryExpression {
//                 name: vec![43], // "add"
//                 children: vec![
//                   Identifier { value: vec![98] }, // "b"
//                   Identifier { value: vec![99] }, // "c"
//                 ]
//               }
//             ] }
//           ] }
//         ]}
//       ]
//     },
//     VariableDefine { children: vec![
//       Identifier { value: vec![114, 101, 115, 117, 108, 116] }, // "result"
//       Expression { children: vec![
//         FunctionCall {
//           name: vec![97, 100, 100], // "add"
//           children: vec![
//             FunctionArguments { children: vec![
//               Expression { children: vec![Identifier { value: vec![97] } ] }, // "a"
//               Expression { children: vec![Number { value: vec![50, 48] } ] }, // "20"
//             ]}
//           ]
//         }
//       ]}
//     ]}
//   ]
// });
//
// test!(
//     test_multiplication_precedence,
//     r#"2 + 3 * 4"#,
//     addition,
//     BinaryExpression {
//         name: vec![43], // "+"
//         children: vec![
//             Number { value: vec![50] }, // "2"
//             BinaryExpression {
//                 name: vec![42], // "*"
//                 children: vec![
//                     Number { value: vec![51] }, // "3"
//                     Number { value: vec![52] }, // "4"
//                 ],
//             },
//         ],
//     }
// );
//
// // Test division precedence over subtraction
// test!(
//     test_division_precedence,
//     r#"10 - 6 / 2"#,
//     addition, // Assuming you have a `subtraction` combinator similar to `addition`
//     BinaryExpression {
//         name: vec![45], // "-"
//         children: vec![
//             Number { value: vec![49, 48] }, // "10"
//             BinaryExpression {
//                 name: vec![47], // "/"
//                 children: vec![
//                     Number { value: vec![54] }, // "6"
//                     Number { value: vec![50] }, // "2"
//                 ],
//             },
//         ],
//     }
// );
//
// //////////////////////////
// // Nested Parentheses //
// //////////////////////////
//
// // Test nested parentheses affecting precedence
// test!(
//     test_nested_parentheses,
//     r#"3 * (2 + (4 - 1))"#,
//     multiplication,
//     BinaryExpression {
//         name: vec![42], // "*"
//         children: vec![
//             Number { value: vec![51] }, // "3"
//             BinaryExpression {
//                 name: vec![43], // "+"
//                 children: vec![
//                     Number { value: vec![50] }, // "2"
//                     BinaryExpression {
//                         name: vec![45], // "-"
//                         children: vec![
//                             Number { value: vec![52] }, // "4"
//                             Number { value: vec![49] }, // "1"
//                         ],
//                     },
//                 ],
//             },
//         ],
//     }
// );
//
// // Test multiple levels of nested parentheses
// test!(
//     test_multiple_nested_parentheses,
//     r#"((1 + 2) * (3 + 4)) / 5"#,
//     multiplication,
//     BinaryExpression {
//         name: vec![47], // "/"
//         children: vec![
//             BinaryExpression {
//                 name: vec![42], // "*"
//                 children: vec![
//                     BinaryExpression {
//                         name: vec![43], // "+"
//                         children: vec![
//                             Number { value: vec![49] }, // "1"
//                             Number { value: vec![50] }, // "2"
//                         ],
//                     },
//                     BinaryExpression {
//                         name: vec![43], // "+"
//                         children: vec![
//                             Number { value: vec![51] }, // "3"
//                             Number { value: vec![52] }, // "4"
//                         ],
//                     },
//                 ],
//             },
//             Number { value: vec![53] }, // "5"
//         ],
//     }
// );
//
// //////////////////////
// // Unary Operators //
// //////////////////////
//
// // Test unary minus
// test!(
//     test_unary_minus,
//     r#"-5 + 3"#,
//     addition,
//     BinaryExpression {
//         name: vec![43], // "+"
//         children: vec![
//             UnaryExpression {
//                 name: vec![45], // "-"
//                 children: vec![
//                     Number { value: vec![53] }, // "5"
//                 ],
//             },
//             Number { value: vec![51] }, // "3"
//         ],
//     }
// );
//
// // Test multiple unary operators
// test!(
//     test_multiple_unary_operators,
//     r#"-2 * -3"#,
//     multiplication,
//     BinaryExpression {
//         name: vec![42], // "*"
//         children: vec![
//             UnaryExpression {
//                 name: vec![45], // "-"
//                 children: vec![
//                     Number { value: vec![50] }, // "2"
//                 ],
//             },
//             UnaryExpression {
//                 name: vec![45], // "-"
//                 children: vec![
//                     Number { value: vec![51] }, // "3"
//                 ],
//             },
//         ],
//     }
// );
//
// ///////////////////////////
// // Exponentiation Tests //
// ///////////////////////////
//
// // Test exponentiation precedence
// test!(
//     test_exponentiation_precedence,
//     r#"2 ^ 3 * 4"#,
//     multiplication,
//     BinaryExpression {
//         name: vec![42], // "*"
//         children: vec![
//             BinaryExpression {
//                 name: vec![94], // "^"
//                 children: vec![
//                     Number { value: vec![50] }, // "2"
//                     Number { value: vec![51] }, // "3"
//                 ],
//             },
//             Number { value: vec![52] }, // "4"
//         ],
//     }
// );
//
// // Test multiple exponentiations (right-associative)
// test!(
//     test_multiple_exponentiations,
//     r#"2 ^ 3 ^ 2"#,
//     exponentiation,
//     BinaryExpression {
//         name: vec![94], // "^"
//         children: vec![
//             BinaryExpression {
//                 name: vec![94], // "^"
//                 children: vec![
//                     Number { value: vec![50] }, // "3"
//                     Number { value: vec![51] }, // "2"
//                 ],
//             },
//             Number { value: vec![50] }, // "2"
//         ],
//     }
// );
//
// //////////////////////////////
// // Complex Mathematical Expr //
// //////////////////////////////
//
// // Test a complex expression combining multiple operations
// test!(
//     test_complex_expression,
//     r#"3 + 5 * (2 - 8) ^ 2 / 4"#,
//     addition,
//     BinaryExpression {
//         name: vec![43], // "+"
//         children: vec![
//             Number { value: vec![51] }, // "3"
//             BinaryExpression {
//                 name: vec![47], // "/"
//                 children: vec![
//                     BinaryExpression {
//                         name: vec![42], // "*"
//                         children: vec![
//                             Number { value: vec![53] }, // "5"
//                             BinaryExpression {
//                                 name: vec![94], // "^"
//                                 children: vec![
//                                     BinaryExpression {
//                                         name: vec![45], // "-"
//                                         children: vec![
//                                             Number { value: vec![50] }, // "2"
//                                             Number { value: vec![56] }, // "8"
//                                         ],
//                                     },
//                                     Number { value: vec![50] }, // "2"
//                                 ],
//                             },
//                         ],
//                     },
//                     Number { value: vec![52] }, // "4"
//                 ],
//             },
//         ],
//     }
// );
//
// //////////////////////
// // Function Calls //
// //////////////////////
//
// // Test function call with expression as argument
// test!(
//     test_function_call_with_expression,
//     r#"foo(3 + 4 * 2)"#,
//     function_call,
//     FunctionCall {
//         name: vec![102, 111, 111], // "foo"
//         children: vec![
//             FunctionArguments {
//                 children: vec![
//                     Expression {
//                         children: vec![
//                             BinaryExpression {
//                                 name: vec![43], // "+"
//                                 children: vec![
//                                     Number { value: vec![51] }, // "3"
//                                     BinaryExpression {
//                                         name: vec![42], // "*"
//                                         children: vec![
//                                             Number { value: vec![52] }, // "4"
//                                             Number { value: vec![50] }, // "2"
//                                         ],
//                                     },
//                                 ],
//                             },
//                         ]
//                     },
//                 ],
//             },
//         ],
//     }
// );
//
// test!{
//   test_greater_than,
//   r#"3 > 4"#,
//   comparison,
//   BinaryExpression {
//     name: vec![62],
//     children: vec![
//       Number { value: vec![51] },
//       Number { value: vec![52] }
//     ]
//   }
// }
//
// test!{
//   test_equals,
//   r#"3 == 4"#,
//   comparison,
//   BinaryExpression {
//     name: vec![61, 61],
//     children: vec![
//       Number { value: vec![51] },
//       Number { value: vec![52] }
//     ]
//   }
// }
//
// test!{
//   test_less_equals,
//   r#"3 <= 4"#,
//   comparison,
//   BinaryExpression {
//     name: vec![60, 61],
//     children: vec![
//       Number { value: vec![51] },
//       Number { value: vec![52] }
//     ]
//   }
// }
//
// // Test an if statement without an else branch
// test!(
//     test_if_simple,
//     r#"if (x > 0) { y = 1; }"#,
//     if_expression,
//     IfExpression {
//         children: vec![
//             // Condition: Expression(BinaryExpression(x > 0))
//             Expression {
//                 children: vec![
//                     BinaryExpression {
//                         name: vec![62], // '>' operator
//                         children: vec![
//                             Identifier { value: vec![120] }, // 'x'
//                             Number { value: vec![48] },      // '0'
//                         ],
//                     },
//                 ],
//             },
//             // Then branch: y = 1;
//             Block {
//                 children: vec![
//                     Assignment {
//                         children: vec![
//                             Identifier { value: vec![121] }, // 'y'
//                             Expression {
//                                 children: vec![
//                                     Number { value: vec![49] }, // '1'
//                                 ],
//                             },
//                         ],
//                     },
//                 ],
//             },
//         ],
//     }
// );
//
// // Test an if statement with an else branch
// test!(
//     test_if_with_else,
//     r#"if (x > 0) { y = 1; } else { y = -1; }"#,
//     if_expression,
//     IfExpression {
//         children: vec![
//             // Condition: Expression(BinaryExpression(x > 0))
//             Expression {
//                 children: vec![
//                     BinaryExpression {
//                         name: vec![62], // '>' operator
//                         children: vec![
//                             Identifier { value: vec![120] }, // 'x'
//                             Number { value: vec![48] },      // '0'
//                         ],
//                     },
//                 ],
//             },
//             // Then branch: y = 1;
//             Block {
//                 children: vec![
//                     Assignment {
//                         children: vec![
//                             Identifier { value: vec![121] }, // 'y'
//                             Expression {
//                                 children: vec![
//                                     Number { value: vec![49] }, // '1'
//                                 ],
//                             },
//                         ],
//                     },
//                 ],
//             },
//             // Else branch: y = -1;
//             Block {
//                 children: vec![
//                     Assignment {
//                         children: vec![
//                             Identifier { value: vec![121] }, // 'y'
//                             Expression {
//                                 children: vec![
//                                     UnaryExpression {
//                                         name: vec![45], // '-' operator
//                                         children: vec![
//                                             Number { value: vec![49] }, // '1'
//                                         ],
//                                     },
//                                 ],
//                             },
//                         ],
//                     },
//                 ],
//             },
//         ],
//     }
// );
//
// // Test a nested if statement
// test!(
//     test_if_nested,
//     r#"if (x > 0) { if (y < 5) { z = 10; } }"#,
//     if_expression,
//     IfExpression {
//         children: vec![
//             // Outer Condition: Expression(BinaryExpression(x > 0))
//             Expression {
//                 children: vec![
//                     BinaryExpression {
//                         name: vec![62], // '>' operator
//                         children: vec![
//                             Identifier { value: vec![120] }, // 'x'
//                             Number { value: vec![48] },      // '0'
//                         ],
//                     },
//                 ],
//             },
//             // Outer Then branch: if (y < 5) { z = 10; }
//             Block {
//                 children: vec![
//                     IfExpression {
//                         children: vec![
//                             // Inner Condition: Expression(BinaryExpression(y < 5))
//                             Expression {
//                                 children: vec![
//                                     BinaryExpression {
//                                         name: vec![60], // '<' operator
//                                         children: vec![
//                                             Identifier { value: vec![121] }, // 'y'
//                                             Number { value: vec![53] },      // '5'
//                                         ],
//                                     },
//                                 ],
//                             },
//                             // Inner Then branch: z = 10;
//                             Block {
//                                 children: vec![
//                                     Assignment {
//                                         children: vec![
//                                             Identifier { value: vec![122] }, // 'z'
//                                             Expression {
//                                                 children: vec![
//                                                     Number { value: vec![49, 48] }, // '10'
//                                                 ],
//                                             },
//                                         ],
//                                     },
//                                 ],
//                             },
//                         ],
//                     },
//                 ],
//             },
//         ],
//     }
// );
//
// // Test a simple while loop
// test!(
//     test_while_simple,
//     r#"while (x < 10) { x = x + 1; }"#,
//     while_loop,
//     WhileLoop {
//         children: vec![
//             // Condition: Expression(BinaryExpression(x < 10))
//             Expression {
//                 children: vec![
//                     BinaryExpression {
//                         name: vec![60], // '<' operator
//                         children: vec![
//                             Identifier { value: vec![120] }, // 'x'
//                             Number { value: vec![49, 48] },  // '10'
//                         ],
//                     },
//                 ],
//             },
//             // Body: x = x + 1;
//             Block {
//                 children: vec![
//                     Assignment {
//                         children: vec![
//                             Identifier { value: vec![120] }, // 'x'
//                             Expression {
//                                 children: vec![
//                                     BinaryExpression {
//                                         name: vec![43], // '+' operator
//                                         children: vec![
//                                             Identifier { value: vec![120] }, // 'x'
//                                             Number { value: vec![49] },      // '1'
//                                         ],
//                                     },
//                                 ],
//                             },
//                         ],
//                     },
//                 ],
//             },
//         ],
//     }
// );
//
// // Test a while loop with multiple body statements
// test!(
//     test_while_multiple_body,
//     r#"while (count < 5) { count = count + 1; sum = sum + count; }"#,
//     while_loop,
//     WhileLoop {
//         children: vec![
//             // Condition: Expression(BinaryExpression(count < 5))
//             Expression {
//                 children: vec![
//                     BinaryExpression {
//                         name: vec![60], // '<' operator
//                         children: vec![
//                             Identifier { value: vec![99, 111, 117, 110, 116] }, // 'count'
//                             Number { value: vec![53] },                        // '5'
//                         ],
//                     },
//                 ],
//             },
//             // Body:
//             Block {
//                 children: vec![
//                     Assignment {
//                         children: vec![
//                             Identifier { value: vec![99, 111, 117, 110, 116] }, // 'count'
//                             Expression {
//                                 children: vec![
//                                     BinaryExpression {
//                                         name: vec![43], // '+' operator
//                                         children: vec![
//                                             Identifier { value: vec![99, 111, 117, 110, 116] }, // 'count'
//                                             Number { value: vec![49] },                         // '1'
//                                         ],
//                                     },
//                                 ],
//                             },
//                         ],
//                     },
//                     Assignment {
//                         children: vec![
//                             Identifier { value: vec![115, 117, 109] }, // 'sum'
//                             Expression {
//                                 children: vec![
//                                     BinaryExpression {
//                                         name: vec![43], // '+' operator
//                                         children: vec![
//                                             Identifier { value: vec![115, 117, 109] },       // 'sum'
//                                             Identifier { value: vec![99, 111, 117, 110, 116] }, // 'count'
//                                         ],
//                                     },
//                                 ],
//                             },
//                         ],
//                     },
//                 ],
//             },
//         ],
//     }
// );
//
// // Test a nested while loop
// test!(
//     test_while_nested,
//     r#"while (x < 10) { while (y < 5) { y = y + 1; } }"#,
//     while_loop,
//     WhileLoop {
//         children: vec![
//             // Outer Condition: Expression(BinaryExpression(x < 10))
//             Expression {
//                 children: vec![
//                     BinaryExpression {
//                         name: vec![60], // '<' operator
//                         children: vec![
//                             Identifier { value: vec![120] },        // 'x'
//                             Number { value: vec![49, 48] },         // '10'
//                         ],
//                     },
//                 ],
//             },
//             // Outer Body: while (y < 5) { y = y + 1; }
//             Block {
//                 children: vec![
//                     WhileLoop {
//                         children: vec![
//                             // Inner Condition: Expression(BinaryExpression(y < 5))
//                             Expression {
//                                 children: vec![
//                                     BinaryExpression {
//                                         name: vec![60], // '<' operator
//                                         children: vec![
//                                             Identifier { value: vec![121] }, // 'y'
//                                             Number { value: vec![53] },      // '5'
//                                         ],
//                                     },
//                                 ],
//                             },
//                             // Inner Body: y = y + 1;
//                             Block {
//                                 children: vec![
//                                     Assignment {
//                                         children: vec![
//                                             Identifier { value: vec![121] }, // 'y'
//                                             Expression {
//                                                 children: vec![
//                                                     BinaryExpression {
//                                                         name: vec![43], // '+' operator
//                                                         children: vec![
//                                                             Identifier { value: vec![121] }, // 'y'
//                                                             Number { value: vec![49] },      // '1'
//                                                         ],
//                                                     },
//                                                 ],
//                                             },
//                                         ],
//                                     },
//                                 ],
//                             },
//                         ],
//                     },
//                 ],
//             },
//         ],
//     }
// );
// test!(
//     test_big_program,
//     r#"
//         let x = 10;
//         let y = 20;
//
//         fn add(a, b) {
//             return a + b;
//         }
//
//         fn main() {
//             let result = add(x, y);
//             if (result > 25) {
//                 while (x < y) {
//                     x = x + 1;
//                 }
//             } else {
//                 x = x - 1;
//             }
//         }
//     "#,
//     program,
//     Program {
//         children: vec![
//             // let x = 10;
//             VariableDefine {
//                 children: vec![
//                     Identifier { value: vec![120] }, // "x"
//                     Expression { children: vec![
//                         Number { value: vec![49, 48] } // "10"
//                     ]}
//                 ]
//             },
//             // let y = 20;
//             VariableDefine {
//                 children: vec![
//                     Identifier { value: vec![121] }, // "y"
//                     Expression { children: vec![
//                         Number { value: vec![50, 48] } // "20"
//                     ]}
//                 ]
//             },
//             // fn add(a, b) { return a + b; }
//             FunctionDefine {
//                 name: vec![97, 100, 100], // "add"
//                 children: vec![
//                     FunctionArguments { children: vec![
//                         Expression { children: vec![Identifier { value: vec![97] }] }, // "a"
//                         Expression { children: vec![Identifier { value: vec![98] }] }, // "b"
//                     ] },
//                     FunctionStatements { children: vec![
//                         FunctionReturn { children: vec![
//                             Expression { children: vec![
//                                 BinaryExpression {
//                                     name: vec![43], // "+" operator
//                                     children: vec![
//                                         Identifier { value: vec![97] }, // "a"
//                                         Identifier { value: vec![98] }, // "b"
//                                     ]
//                                 }
//                             ] }
//                         ] }
//                     ]}
//                 ]
//             },
//             // fn main() { ... }
//             FunctionDefine {
//                 name: vec![109, 97, 105, 110], // "main"
//                 children: vec![
//                     FunctionArguments { children: vec![] }, // No arguments
//                     FunctionStatements { children: vec![
//                         // let result = add(x, y);
//                         VariableDefine {
//                             children: vec![
//                                 Identifier { value: vec![114, 101, 115, 117, 108, 116] }, // "result"
//                                 Expression { children: vec![
//                                     FunctionCall {
//                                         name: vec![97, 100, 100], // "add"
//                                         children: vec![
//                                             FunctionArguments { children: vec![
//                                                 Expression { children: vec![Identifier { value: vec![120] }] }, // "x"
//                                                 Expression { children: vec![Identifier { value: vec![121] }] }, // "y"
//                                             ]}
//                                         ]
//                                     }
//                                 ]}
//                             ]
//                         },
//                         // if (result > 25) { while (x < y) { x = x + 1; } } else { x = x - 1; }
//                         IfExpression {
//                             children: vec![
//                                 // Condition: result > 25
//                                 Expression {
//                                     children: vec![
//                                         BinaryExpression {
//                                             name: vec![62], // ">" operator
//                                             children: vec![
//                                                 Identifier { value: vec![114, 101, 115, 117, 108, 116] }, // "result"
//                                                 Number { value: vec![50, 53] }, // "25"
//                                             ],
//                                         },
//                                     ],
//                                 },
//                                 // Then branch: { while (x < y) { x = x + 1; } }
//                                 Block {
//                                     children: vec![
//                                         WhileLoop {
//                                             children: vec![
//                                                 // Condition: x < y
//                                                 Expression {
//                                                     children: vec![
//                                                         BinaryExpression {
//                                                             name: vec![60], // "<" operator
//                                                             children: vec![
//                                                                 Identifier { value: vec![120] }, // "x"
//                                                                 Identifier { value: vec![121] }, // "y"
//                                                             ],
//                                                         },
//                                                     ],
//                                                 },
//                                                 // Body: { x = x + 1; }
//                                                 Block {
//                                                     children: vec![
//                                                         Assignment {
//                                                             children: vec![
//                                                                 Identifier { value: vec![120] }, // "x"
//                                                                 Expression { children: vec![
//                                                                     BinaryExpression {
//                                                                         name: vec![43], // "+" operator
//                                                                         children: vec![
//                                                                             Identifier { value: vec![120] }, // "x"
//                                                                             Number { value: vec![49] }, // "1"
//                                                                         ],
//                                                                     },
//                                                                 ]}
//                                                             ]
//                                                         },
//                                                     ],
//                                                 },
//                                             ],
//                                         },
//                                     ],
//                                 },
//                                 // Else branch: { x = x - 1; }
//                                 Block {
//                                     children: vec![
//                                         Assignment {
//                                             children: vec![
//                                                 Identifier { value: vec![120] }, // "x"
//                                                 Expression { children: vec![
//                                                     BinaryExpression {
//                                                         name: vec![45], // "-" operator
//                                                         children: vec![
//                                                             Identifier { value: vec![120] }, // "x"
//                                                             Number { value: vec![49] }, // "1"
//                                                         ],
//                                                     },
//                                                 ]}
//                                             ]
//                                         },
//                                     ],
//                                 },
//                             ],
//                         },
//                     ]}
//                 ]
//             },
//         ]
//     }
// );