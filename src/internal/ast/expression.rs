use std::string::String;


pub enum ExpressionType {
    IntegerExpression(i32),
    FloatExpression(f64),
    StringExpression(String),
    BoolExpression(bool),
    BitMaskExpression,
    EnumExpression,
    CompoundExpression,
    OtherExpression,
}


pub struct Expression {
    pub text: String,
    pub operand1: Option<Box<Expression>>,
    pub operand2: Option<Box<Expression>>,
    pub operand3: Option<Box<Expression>>,
    pub result_type: ExpressionType,
    pub fully_resolved: bool,
}
