# fns (Abandoned)

A Functional, Declarative, Dynamic Programming Language

## What is it?

fns is a simple dynamic, declarative and functional programming language. It has syntax and semantics like JavaScript. You can think of it as a subset of js. It will only include the functional features of js.

## Grammar

```
PROGRAM = STATEMENT*

---

STATEMENT = LET_STATEMENT
                         | CONST_STATEMENT

LET_STATEMENT = "let" IDENTIFIER "=" EXPRESSION
CONST_STATEMENT = "const" IDENTIFIER "=" EXPRESSION

---

EXPRESSION = "(" CORE_EXPRESSION ")"
                          | CORE_EXPRESSION

CORE_EXPRESSION = ASSIGNMENT_EXPRESSION
                                     | BINARY_EXPRESSION
                                     | UNARY_EXPRESSION
                                     | OBJECT_LITERAL_EXPRESSION
                                     | ACCESS_EXPRESSION
                                     | STRING_LITERAL_EXPRESSION
                                     | NUMERIC_LITERAL_EXPRESSION
                                     | BOOLEAN_LITERAL_EXPRESSION
                                     | NONE_LITERAL_EXPRESSION
                                     | IDENTIFIER_EXPRESSION

ASSIGNMENT_EXPRESSION = IDENTIFIER "=" EXPRESSION
BINARY_EXPRESSION = EXPRESSION BINARY_OPERATOR EXPRESSION
UNARY_EXPRESSION = UNARY_OPERATOR EXPRESSION
OBJECT_LITERAL_EXPRESSION = OBJECT
ACCESS_EXPRESSION = IDENTIFIER "." IDENTIFIER
STRING_LITERAL_EXPRESSION = STRING
NUMERIC_LITERAL_EXPRESSION = NUMBER
BOOLEAN_LITERAL_EXPRESSION = BOOLEAN
NONE_LITERAL_EXPRESSION = NONE
IDENTIFIER_EXPRESSION = IDENTIFIER

---

BINARY_OPERATOR = "+" | "-" | "*" | "/" | ">" | "<" | "<=" | ">=" | "==" | "!=" | "&&" | "||"
UNARY_OPERATOR = "+" | "-" | "!"

OBJECT = "{" KEY_VALUE_PAIR,* "}"
KEY_VALUE_PAIR = IDENTIFIER ":" EXPRESSION
STRING = " * "
NUMBER = [0-9]+.*[0-9]*
BOOLEAN = "true" | "false"
NONE = "none"
IDENTIFIER = (_*[A-Z]*[a-z]*)+
```

## To Fix

### These are the bugs that the current implementation has

- Complex grouping of expression (like this: `1 + 2 - 3 * 4 / 5 + ( 6 - 7)`) doesn't work.
