use std::fmt;

#[derive(Debug, Clone)]
pub struct DocReading;

impl fmt::Display for DocReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error reading the document")
    }
}

#[derive(Debug, Clone)]
pub struct PromptAdding;

impl fmt::Display for PromptAdding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error adding the prompt")
    }
}