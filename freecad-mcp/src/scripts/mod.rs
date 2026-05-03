use std::path::Path;

/// Builds a FreeCAD Python script with the standard document-open header and JSON result footer.
///
/// Every generated script follows the contract in CLAUDE.md:
/// - Imports `FreeCAD`, `Part`, `json`, `os`, `sys`
/// - Opens or creates the session document at `DOC_PATH`
/// - Wraps all tool logic in `try/except Exception`
/// - Prints a single JSON line as the very last stdout output
pub struct ScriptBuilder {
    lines: Vec<String>,
}

impl ScriptBuilder {
    /// Create a new builder and inject the standard document-open header.
    pub fn new(session_doc: &Path) -> Self {
        let path_str = session_doc.display().to_string().replace('\\', "\\\\");
        let header = format!(
            r#"import FreeCAD, Part, json, os, sys
DOC_PATH = r"{path_str}"
try:
    if os.path.exists(DOC_PATH):
        doc = FreeCAD.openDocument(DOC_PATH)
    else:
        doc = FreeCAD.newDocument("session")"#
        );
        let lines: Vec<String> = header.lines().map(|l| l.to_string()).collect();
        Self { lines }
    }

    /// Append a line of Python to the script body (indented inside the try block).
    pub fn push(&mut self, line: impl Into<String>) {
        self.lines.push(format!("    {}", line.into()));
    }

    /// Finish the script: append the result print and the except block, then return the full source.
    pub fn finish(mut self, result_expr: &str) -> String {
        self.lines.push(format!("    print(json.dumps({result_expr}))"));
        self.lines.push("except Exception as e:".to_string());
        self.lines.push(
            r#"    print(json.dumps({"success": False, "error": str(e)}))"#.to_string(),
        );
        self.lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn generated_script_contains_doc_path() {
        let doc = PathBuf::from("/tmp/test/session.FCStd");
        let builder = ScriptBuilder::new(&doc);
        let script = builder.finish(r#"{"success": True}"#);
        assert!(script.contains("/tmp/test/session.FCStd"));
        assert!(script.contains("FreeCAD.openDocument"));
        assert!(script.contains("json.dumps"));
        assert!(script.contains("except Exception"));
    }

    #[test]
    fn push_indents_lines_inside_try() {
        let doc = PathBuf::from("/tmp/session.FCStd");
        let mut builder = ScriptBuilder::new(&doc);
        builder.push("obj = doc.addObject(\"Part::Box\", \"Box\")");
        let script = builder.finish(r#"{"success": True, "object_id": obj.Name}"#);
        assert!(script.contains("    obj = doc.addObject"));
    }
}
