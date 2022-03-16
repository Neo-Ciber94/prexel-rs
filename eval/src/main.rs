mod writer;
mod eval_expr;
mod list;
mod repl;
mod style;
mod collections;

use crate::writer::{ColorWriter, set_use_colors};
use crate::eval_expr::EvalExpr;
use crate::list::ListKind;
use clap::{Parser, Subcommand};
use std::str::FromStr;
use crate::repl::ReplConfig;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EvalType {
    Decimal,
    Float,
    Integer,
    Complex,
    Binary,
}

impl FromStr for EvalType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "decimal" => Ok(EvalType::Decimal),
            "float" => Ok(EvalType::Float),
            "integer" => Ok(EvalType::Integer),
            "complex" => Ok(EvalType::Complex),
            "binary" => Ok(EvalType::Binary),
            _ => Err(format!("Unknown eval type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version, author, about, propagate_version = true)]
struct Cli {
    #[clap(long, global = true, help = "Disables color output")]
    no_color: bool,

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

    #[clap(about = "Evaluates math expressions in a REPL (read-eval-print loop)")]
    Repl {
        #[clap(long, short, default_value = "decimal")]
        r#type: EvalType,
        #[clap(long)]
        history: Option<usize>
    },

    #[clap(about = "Prints the constants")]
    Constants {
        #[clap(long, short, default_value = "decimal")]
        r#type: EvalType,
    },

    #[clap(about = "Prints the operators")]
    Operators {
        #[clap(long, short, default_value = "decimal")]
        r#type: EvalType,
    },

    #[clap(about = "Prints the functions")]
    Functions {
        #[clap(long, short, default_value = "decimal")]
        r#type: EvalType,
    },
}

fn main() {
    let cli: Cli = Cli::parse();
    set_use_colors(!cli.no_color);

    match cli.commands {
        Commands::Eval { r#type, expression } => match EvalExpr::new(r#type).eval(&expression) {
            Ok(result) => ColorWriter::new().writeln(result),
            Err(err) => ColorWriter::new().red().writeln_err(err),
        },
        Commands::Repl { r#type, history } => {
            repl::run_repl(ReplConfig {
                history_size: history,
                eval_type: r#type,
            });
        }
        Commands::Constants { r#type } => {
            list::list(r#type, ListKind::Constants);
        }
        Commands::Operators { r#type } => {
            list::list(r#type, ListKind::Operators);
        }
        Commands::Functions { r#type } => {
            list::list(r#type, ListKind::Functions);
        }
    }
}
