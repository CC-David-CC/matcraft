//! File actions delegated to the embedding workspace.

use serde_json::{json, Value};

use crate::{Doc, Editor, MatApp};

pub(crate) enum FileRequest {
    New(Option<String>),
    Open,
    Save,
}

impl MatApp {
    /// Keep one script in this window and let the host handle Files operations.
    pub fn enable_workspace_files(&mut self, compact: bool) {
        self.workspace_files = true;
        if compact {
            self.show_left = false;
            self.show_right = false;
        }
        self.raise_doc(Doc::Edit(0));
        if let Some(editor) = self.editors.first_mut() {
            editor.workspace_files = true;
        }
    }

    /// Apply an opened script or a new scratch buffer.
    pub fn workspace_replace(&mut self, name: String, text: Option<String>) -> bool {
        if !self.workspace_files {
            return false;
        }
        let Some(editor) = self.editors.first_mut() else {
            return false;
        };
        *editor = Editor::new();
        editor.workspace_files = true;
        editor.dirty = name.is_empty() && text.is_some();
        editor.path = name;
        if let Some(text) = text {
            editor.text = text;
        }
        self.raise_doc(Doc::Edit(0));
        true
    }

    /// Snapshot the exact script currently held by this window.
    pub fn workspace_snapshot(&self) -> Option<Value> {
        if !self.workspace_files {
            return None;
        }
        let editor = self.editors.first()?;
        Some(json!({ "text": editor.text, "dirty": editor.dirty }))
    }

    /// A successful host write saves only the bytes included in that write.
    pub fn workspace_saved(&mut self, name: String, text: &str) -> bool {
        if !self.workspace_files {
            return false;
        }
        let Some(editor) = self.editors.first_mut() else {
            return false;
        };
        editor.path = name;
        editor.dirty = editor.text != text;
        !editor.dirty
    }

    /// Run the script through the Studio's ordinary buffer evaluator.
    pub fn workspace_run(&mut self) -> Option<Value> {
        if !self.workspace_files {
            return None;
        }
        let editor = self.editors.first()?;
        let name = (!editor.path.is_empty()).then(|| editor.path.clone());
        let text = editor.text.clone();
        Some(self.eval_editor_source(text, name.as_deref()))
    }

    /// Return a pending user action from Studio's buttons or shortcuts.
    pub fn workspace_take_request(&mut self) -> Option<Value> {
        match self.workspace_request.take()? {
            FileRequest::New(text) => Some(json!({ "kind": "new", "text": text })),
            FileRequest::Open => Some(json!({ "kind": "open" })),
            FileRequest::Save => Some(json!({ "kind": "save" })),
        }
    }
}
