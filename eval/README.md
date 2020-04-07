# Command Line - Math Expression Evaluator
A command line program for evaluate math expressions.

## Usage
##### Commands
- `eval [EXPRESSION]`: Evaluates the specified expression
- `eval [--OPTION] [EXPRESSION]`: Evaluates the specified expression
- `eval [--SUBCOMMAND] [ARGS]`: Runs the specified subcommand.
using the given option.

##### Sub Commands:
- `--help | --h`: Get help information for the command.
- `--run | --r`: Runs the evaluator in loop mode.
- `--context | --ctx`: Gets the constants, functions and operators of the 
math context used for evaluations.

##### Options:
- `--decimal | --d`: Flag for evaluate using a 128 bit decimal number.
Used by default.

- `--bigdecimal | --b`: Flag for evaluate using a arbitrary precision
decimal number.

- `--complex | --c`: Flag for evaluate complex numbers expressions.
