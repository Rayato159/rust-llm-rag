use std::fmt;

#[derive(Debug, Clone)]
pub struct DocReading;

impl fmt::Display for DocReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error reading the document")
    }
}

#[derive(Debug, Clone)]
pub struct DocAdding;

impl fmt::Display for DocAdding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error adding the doc")
    }
}

#[derive(Debug, Clone)]
pub struct DocEmbedding;

impl fmt::Display for DocEmbedding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error embedding the doc")
    }
}
