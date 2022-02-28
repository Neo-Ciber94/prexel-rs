use std::fmt::Display;
use prexel::context::DefaultContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FnInfo {
    name: String,
    aliases: Vec<String>,
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VarInfo {
    name: String,
    value: String,
}

pub fn get_operators<N>(context: &DefaultContext<'_, N>) -> Vec<FnInfo> {
    context
        .binary_functions()
        .iter()
        .map(|(name, f)| FnInfo {
            name: name.to_string(),
            aliases: f
                .aliases()
                .map(|v| v.iter().map(|s| s.to_string()).collect())
                .unwrap_or_default(),
            description: f.description().map(|s| s.to_string()),
        })
        .chain(context.unary_functions().iter().map(|(name, f)| {
            FnInfo {
                name: name.to_string(),
                aliases: f
                    .aliases()
                    .map(|v| v.iter().map(|s| s.to_string()).collect())
                    .unwrap_or_default(),
                description: f.description().map(|s| s.to_string()),
            }
        }))
        .collect()
}

pub fn get_functions<N: Display>(context: &DefaultContext<'_, N>) -> Vec<FnInfo> {
    context
        .functions()
        .iter()
        .map(|(name, f)| FnInfo {
            name: name.to_string(),
            aliases: f
                .aliases()
                .map(|v| v.iter().map(|s| s.to_string()).collect())
                .unwrap_or_default(),
            description: f.description().map(|s| s.to_string()),
        })
        .collect()
}

pub fn get_constants<N: Display>(context: &DefaultContext<'_, N>) -> Vec<VarInfo> {
    context
        .constants()
        .iter()
        .map(|(name, v)| VarInfo {
            name: name.to_string(),
            value: v.to_string(),
        })
        .collect()
}
