use serde::Serialize;

/// A single row in the resource table, holding column values as strings.
#[derive(Debug, Clone, Serialize)]
pub struct TableRow {
    pub name: String,
    pub namespace: String,
    pub columns: Vec<String>,
    pub age: String,
}

/// The full table of resources for a given view.
#[derive(Debug, Default, Clone)]
pub struct ResourceTable {
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
}