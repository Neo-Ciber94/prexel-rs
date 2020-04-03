use crate::cmd::commands::info::NumberType;
use crate::cmd::error::{Error, Result};
use crate::cmd::{Command, CommandArgs};
use bigdecimal::BigDecimal;
use math_engine::context::DefaultContext;
use math_engine::utils::extensions::IteratorExt;
use std::convert::TryFrom;

pub struct ContextCommand;
impl Command<String, Result> for ContextCommand {
    fn name(&self) -> &str {
        "--context"
    }

    fn alias(&self) -> Option<&str> {
        Some("--ctx")
    }

    fn help_info(&self) -> &str {
        "\
Prints the constants, functions and operators of a context

USAGE:
    eval --context | ctx
    eval --context | ctx [--OPTION]
    eval --context | ctx [--OPTION] [-PRINT FORMAT]

OPTIONS:
    --decimal, --d          Evaluates using a 128 bits decimal number. Used by default
    --bigdecimal, --b       Evaluates using an arbitrary decimal number
    --complex, --c          Evaluates using a complex number

PRINT FORMATS:
    -t, -table              Flag for print as a table. Used by default.
    -r, -row                Flag for print as rows.
    -c, -col                Flag for print as columns.

EXAMPLES:
    eval --context
    eval --ctx
    eval --context --c
    eval --ctx --bigdecimal
    eval --context -table
    eval --ctx -c"
    }

    fn execute(&self, args: CommandArgs<'_, String>) -> Result {
        if args.len() > 2 {
            return Err(Error::new(format!(
                "Invalid expression, expected: eval {} | {} [--OPTION]",
                self.name(),
                self.alias().unwrap()
            )));
        }

        let mut i = 0;
        let number_type = if args.len() == 0 {
            NumberType::default()
        } else {
            NumberType::try_from(args[0].as_str())
                .map(|n| { i += 1; n })
                .unwrap_or_default()
        };

        let print_format = if i >= args.len(){
            PrintFormat::default()
        } else{
            PrintFormat::try_from(args[i].as_str())
                .unwrap_or_default()
        };

        match number_type {
            NumberType::Decimal => {
                print_context(&DefaultContext::new_decimal(), print_format)
            },
            NumberType::BigDecimal => {
                let ctx: DefaultContext<BigDecimal> = DefaultContext::new_unchecked();
                print_context(&ctx, print_format);
            }
            NumberType::Complex => {
                print_context(&DefaultContext::new_complex(), print_format)
            },
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PrintFormat{
    Table, Row, Column
}

impl Default for PrintFormat{
    #[inline]
    fn default() -> Self {
        PrintFormat::Table
    }
}

impl TryFrom<&str> for PrintFormat{
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value{
            "-t" | "-table" => Ok(PrintFormat::Table),
            "-r" | "-row" => Ok(PrintFormat::Row),
            "-c" | "-col" => Ok(PrintFormat::Column),
            _ => Err(())
        }
    }
}

fn print_context<N>(context: &DefaultContext<N>, format: PrintFormat){
    match format{
        PrintFormat::Table => print_context_as_table(&context),
        PrintFormat::Row => print_context_as_row(&context),
        PrintFormat::Column => print_context_as_column(&context),
    }
}

fn print_context_as_table<N>(context: &DefaultContext<N>) {
    const PAD: usize = 18;

    let mut buffer = String::new();

    buffer.push_str(&format!(
        "{1:0$} | {2:0$} | {3:0$} | {4:0$}\n",
        PAD, "Functions", "Binary Operators", "Unary Operators", "Constants"
    ));

    buffer.push_str(&format!(
        "{1:-<0$}---{1:-<0$}---{1:-<0$}---{1:-<0$}\n",
        PAD, ""
    ));

    let mut iter1 = context
        .functions()
        .iter()
        .sorted_by_key(|c| c.0.as_str().chars().nth(0).unwrap());

    let mut iter2 = context.binary_functions().iter();
    let mut iter3 = context.unary_functions().iter();
    let mut iter4 = context.constants().iter();

    loop {
        let item1 = iter1.next().map(|n| n.0);
        let item2 = iter2.next().map(|n| n.0);
        let item3 = iter3.next().map(|n| n.0);
        let item4 = iter4.next().map(|n| n.0);

        if item1.is_none() && item2.is_none() && item3.is_none() && item4.is_none() {
            break;
        }

        let s1 = item1.map_or("", |s| s.as_str());
        let s2 = item2.map_or("", |s| s.as_str());
        let s3 = item3.map_or("", |s| s.as_str());
        let s4 = item4.map_or("", |s| s.as_str());

        buffer.push_str(&format!(
            "{1:0$} | {2:0$} | {3:0$} | {4:0$}\n",
            PAD, s1, s2, s3, s4
        ));
    }

    println!("{}", buffer);
}

fn print_context_as_row<N>(context: &DefaultContext<N>){
    println!("Functions: ");
    context.functions().iter()
        .sorted_by_key(|c| c.0.as_str().chars().nth(0).unwrap())
        .for_each(|s| print!("{} ", s.0));
    println!("\n");

    println!("Binary Operators: ");
    context.binary_functions().iter().for_each(|s| print!("{} ", s.0));
    println!("\n");

    println!("Unary Operators: ");
    context.unary_functions().iter().for_each(|s| print!("{} ", s.0));
    println!("\n");

    println!("Constants: ");
    context.constants().iter().for_each(|s| print!("{} ", s.0));
    println!("\n");
}

fn print_context_as_column<N>(context: &DefaultContext<N>){
    println!("Functions: ");
    context.functions().iter()
        .sorted_by_key(|c| c.0.as_str().chars().nth(0).unwrap())
        .for_each(|s| println!("{}", s.0));
    println!();

    println!("Binary Operators: ");
    context.binary_functions().iter().for_each(|s| println!("{}", s.0));
    println!();

    println!("Unary Operators: ");
    context.unary_functions().iter().for_each(|s| println!("{}", s.0));
    println!();

    println!("Constants: ");
    context.constants().iter().for_each(|s| println!("{}", s.0));
}
