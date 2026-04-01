use serde::Serialize;

#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExecutionResult {
    Expression {
        input: String,
        output: ExpressionForms,
    },
    Plot {
        input: String,
        data: Vec<(f64, f64)>,
    },
    HelpTableOfContents {
        input: String,
        builtins: Vec<(String, String, String)>,
    },
    HelpBuiltin {
        input: String,
        title: String,
        patterns: Vec<(String, String)>,
        examples: Vec<(String, String)>,
        related: Vec<String>,
    },
}

#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExecutionError {
    ParseError { input: String, msg: String },
    EvaluationError(String),
    UnknownBuiltIn,
}

#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionForms {
    pub raw: String,
    pub latex: String,
}

#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum KernelResult {
    Ok(ExecutionResult),
    Err(ExecutionError),
}
