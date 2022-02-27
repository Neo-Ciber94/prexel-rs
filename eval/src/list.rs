use crate::EvalType;
use prexel::context::DefaultContext;
use std::fmt::Display;
use prexel::complex::Complex;

const MIN_WIDTH: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListKind {
    Variables,
    Constants,
    Operators,
    Functions,
}

pub fn list( eval_type: EvalType, list: ListKind) {
    match eval_type {
        EvalType::Decimal => {
            list_with_context(list, &DefaultContext::new_decimal());
        },
        EvalType::Float => {
            list_with_context(list, &DefaultContext::<f64>::new_unchecked());
        },
        EvalType::Integer => {
            list_with_context(list, &DefaultContext::<i128>::new_checked());
        }
        EvalType::Complex => {
            list_with_context(list, &DefaultContext::<Complex<f64>>::new_complex());
        }
    }
}

pub fn list_with_context<N>(list: ListKind, context: &DefaultContext<'_, N>) where
    N: Display{
    match list {
        ListKind::Variables => list_variables(context),
        ListKind::Constants => list_constants(context),
        ListKind::Operators => list_operators(context),
        ListKind::Functions => list_functions(context),
    }
}

pub fn list_variables<N>(context: &DefaultContext<'_, N>)
where
    N: Display,
{
    let variables = context.variables();
    let max_name_length = variables
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0)
        + MIN_WIDTH;

    for (name, value) in variables {
        println!("{}{}", pad_right(name, max_name_length), value);
    }
}

pub fn list_constants<N>(context: &DefaultContext<'_, N>)
where
    N: Display,
{
    let constants = context.constants();
    let max_name_length = constants
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0)
        + MIN_WIDTH;

    for (name, value) in constants {
        println!("{}{}", pad_right(name, max_name_length), value);
    }
}

pub fn list_operators<N>(context: &DefaultContext<'_, N>)
where
    N: Display,
{
    struct Operator {
        name: String,
        description: String,
    }

    let operators = context
        .binary_functions()
        .iter()
        .map(|(name, op)| Operator {
            name: name.to_string(),
            description: op.description().unwrap_or_default().to_string(),
        })
        .chain(context.unary_functions().iter().map(|(name, op)| Operator {
            name: name.to_string(),
            description: op.description().unwrap_or_default().to_string(),
        }))
        .collect::<Vec<_>>();

    let max_name_length = operators
        .iter()
        .map(|op| op.name.len())
        .max()
        .unwrap_or(0)
        + MIN_WIDTH;

    for op in operators {
        println!("{}{}", pad_right(&op.name, max_name_length), op.description);
    }
}

pub fn list_functions<N>(context: &DefaultContext<'_, N>)
where
    N: Display,
{
    let functions = context.functions();
    let max_name_length = functions
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0) + MIN_WIDTH;

    for (name, f) in functions {
        let description = f.description().unwrap_or_default();
        println!("{}{}", pad_right(name, max_name_length), description);
    }
}

fn pad_right(s: &str, width: usize) -> String {
    let mut s = s.to_string();
    let padding = if s.len() < width {
        width - s.len()
    } else {
        0
    };

    for _ in 0..padding {
        s.push(' ');
    }

    s
}
