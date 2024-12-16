```
Notation:
,     Concat
|     Alternative
{}     1 or more
[]    optional
()    Grouping
??    Special Form

program                 = {function_definition|expression|statement|string|bool|number|comment} ;
function_definition     = "fn" , identifier , "(" , [arguments] , ")" , "{" , {statement} , "}" ;
arguments               = expression , { "," , expression } ;
statement               = (variable_define | function_return) ";", [comment] ;
variable_define         = "let" , identifier , "=" , expression ;
function_return         = "return" , (function_call | expression | value) ;
function_call           = identifier , "(" , [arguments] , ")" ;
expression              = boolean | math_expression | function_call | number | string | identifier ;
math_expression         = value , { ("+" | "-") , value } ;
value                   = number | identifier | boolean | string ;
number                  = {digit} ;
boolean                 = "true" | "false" ;
string                  = "\"" , {alnum | " "} , "\"" ;
identifier              = alpha , <alnum> ;
alpha                   = ?alphabetic or equivalent character?;
alnum                   = ?alphanumeric character?;
digit                   = 0..9;
whitespace              = space | tab | newline | carriage_return; 
comment                 = "//", ?any character?

Note: The grammar as written doesn't handle whitespace, although the examples include it. You should handle it accordingly.
```
