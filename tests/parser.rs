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