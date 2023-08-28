# fns

A Functional Scripting Language

## What is it?

fns is a simple dynamic, declarative and functional programming language. It has syntax and semantics like js but only those that are not weird. It's a subset of js. It WILL only include the functional  features of js.

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

### These are the bugs that the current implementation has

- Complex grouping of expression (like this: `1 + 2 - 3 * 4 / 5 + ( 6 - 7)`) doesn't work.
