# rustpn
This is a Reverse Polish Notation calculator that I built to train my skills with Rust.
It is already usable, although not what I would call "user friendly" just yet.

## How it works
For those of you familiar with RPN calculators, it will be easy enough.

At the CLI, input a series of numbers, alphanumeric strings, operations, keywords, etc.  These will be split by whitespace (including newlines) and then parsed.  Any numbers that the parser finds will be pushed to the top of the stack.  If the parser finds an operator, it will pull the necessary number of items off the top of the stack, perform the operation, and then push the result back onto the stack.

Alphanumeric strings are treated like variables and can also be pushed to the stack.  When pulled off the stack to be operated on, the calculator will check to see if this string has been assigned a value and if so this will be used in the operation.  If not, then the calculator will report an error.

## Assignment
`myvar 2 =` will assign a value of 2 to the variable `myvar`.

## Operations
...

## Keywords
...
