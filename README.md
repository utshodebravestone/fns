# fns

A Functional Scripting Language

## Grammar

```
PROGRAM = STATEMENT*

STATEMENT = LET_STATEMENT
LET_STATEMENT = "let" IDENTIFIER "=" EXPRESSION

EXPRESSION = "(" CORE_EXPRESSION ")" | CORE_EXPRESSION
CORE_EXPRESSION = BINARY_EXPRESSION | NUMERIC_EXPRESSION | IDENTIFIER_EXPRESSION
BINARY_EXPRESSION = EXPRESSION BINARY_OPERATOR EXPRESSION
NUMERIC_EXPRESSION = NUMBER
IDENTIFIER_EXPRESSION = IDENTIFIER


BINARY_OPERATOR = "+" | "-" | "*" | "/"
NUMBER = [0-9]+.*[0-9]*
IDENTIFIER = (_*[A-Z]*[a-z]*)+
```

## To Fix

These are the bugs that the current implementation has:

- Complex grouping of expression (like this: `1 + 2 - 3 * 4 / 5 + ( 6 - 7)`) doesn't work.
