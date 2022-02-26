mod eval_expr;
mod colored;
mod repl;

use std::str::FromStr;
use clap::{Parser, Subcommand};
use termcolor::Color;
use crate::eval_expr::EvalExpr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EvalType {
    Decimal,
    Float,
    Integer,
    Complex,
    //Binary,
}

impl FromStr for EvalType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "decimal" => Ok(EvalType::Decimal),
            "float" => Ok(EvalType::Float),
            "integer" => Ok(EvalType::Integer),
            "complex" => Ok(EvalType::Complex),
            //"binary" => Ok(EvalType::Binary),
            _ => Err(format!("Unknown eval type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version, author, about, propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about)]
    Eval {
        #[clap(long, short, default_value = "decimal")]
        r#type: EvalType,
        expression: String,
    },

    #[clap(about="Evaluates math expressions in a REPL (read-eval-print loop)")]
    Repl {
        #[clap(long, short, default_value = "decimal")]
        r#type: EvalType,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.commands {
        Commands::Eval { r#type, expression } => {
            match EvalExpr::new(r#type).eval(&expression) {
                Ok(result) => println!("{}", result),
                Err(err) => eprintln_colored!(Color::Red, "{}", err),
            }
        },
        Commands::Repl { r#type } => {
            repl::run_repl(r#type);
        },
    }
}