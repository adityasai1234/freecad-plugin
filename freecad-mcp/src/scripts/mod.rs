use std::path::Path;

/// Builds a FreeCAD Python script with the standard document-open header and JSON result footer.
pub struct ScriptBuilder {
    lines: Vec<String>,
}

impl ScriptBuilder {
    /// Create a new builder; injects the document-open header using `session_doc`.
    pub fn new(_session_doc: &Path) -> Self {
        todo!()
    }

    /// Append a line of Python to the script body.
    pub fn push(&mut self, _line: impl Into<String>) {
        todo!()
    }

    /// Finish the script: append `print(json.dumps({result_expr}))` and the except block.
    pub fn finish(self, _result_expr: &str) -> String {
        todo!()
    }
}
