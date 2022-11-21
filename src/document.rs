use crate::Row;
use crate::Terminal;

pub struct Document
{
    rows: Vec<String>,
}

impl Document
{
    pub fn default(&mut self) -> Self {
        let mut rows = Vec::new();
        rows.push("Hello World!".to_string());
        Self {
            rows
        }
    } 
}
