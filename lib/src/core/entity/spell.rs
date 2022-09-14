// use std::fmt::{Display, Error, Formatter};
#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub level: f64,
    pub desc: Vec<String>,
    pub classes: Vec<String>,
}
