use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Number(i64),
    Variable(String),
    // Add more variants as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub expressions: Vec<Expr>,
}
