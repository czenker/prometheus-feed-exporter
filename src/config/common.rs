use cel_interpreter::Program;
use cel_interpreter::{Context, ResolveResult};
use serde::Deserialize;

// wrap program so we are able to write a deserializer
#[derive(Debug)]
pub struct MyCelProgram {
    pub program: Program,
    expression: String,
}

impl<'de> Deserialize<'de> for MyCelProgram {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let expression = String::deserialize(deserializer)?;
            Program::compile(expression.as_str()).map_err(serde::de::Error::custom).map(|p| MyCelProgram{program: p, expression: expression})
    }
}
impl PartialEq for MyCelProgram {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
    }
}
impl MyCelProgram {
    pub fn execute(&self, context: &Context) -> ResolveResult {
        self.program.execute(context)
    }
}