use std::string::String;

#[derive(Clone)]
pub enum ExpressionType {
    Integer(i32),
    Float(f64),
    String(String),
    Bool(bool),
    BitMask,
    Enum,
    Compound,
    Other,
}

#[derive(Clone)]
pub struct Expression {
    pub expression_type: isize,
    pub text: String,
    pub operand1: Option<Box<Expression>>,
    pub operand2: Option<Box<Expression>>,
    pub operand3: Option<Box<Expression>>,
    pub result_type: ExpressionType,
    pub fully_resolved: bool,
}
