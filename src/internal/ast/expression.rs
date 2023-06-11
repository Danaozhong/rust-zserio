use crate::internal::parser::gen::zserioparser::{
    AND, BANG, DIVIDE, LSHIFT, MINUS, MODULO, MULTIPLY, OR, PLUS, RSHIFT, TILDE, XOR,
};
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

#[derive(PartialEq, Clone)]
pub enum EvaluationState {
    NotEvaluated,
    InProgress,
    Completed,
}

#[derive(Clone)]
pub struct Expression {
    pub expression_type: isize,
    pub text: String,
    pub operand1: Option<Box<Expression>>,
    pub operand2: Option<Box<Expression>>,
    pub operand3: Option<Box<Expression>>,
    pub result_type: ExpressionType,

    // Flag which indicates that the expression is fully resolved
    pub fully_resolved: bool,
    pub evaluation_state: EvaluationState,
}

impl Expression {
    pub fn evaluate(&mut self) {
        // An expression that is already fully resolved does not require evaluation.
        if self.evaluation_state == EvaluationState::Completed {
            return;
        }

        // Start the evaluation
        assert!(
            self.evaluation_state != EvaluationState::InProgress,
            "cyclic expression found"
        );
        self.evaluation_state = EvaluationState::InProgress;

        // evaluate all operands. The final expression can only be fully resolved if all operands
        // are also fully resolved.
        self.fully_resolved = true;
        if let Some(op1) = self.operand1.as_mut() {
            op1.evaluate();
            self.fully_resolved &= op1.fully_resolved;
        }
        if let Some(op2) = self.operand2.as_mut() {
            op2.evaluate();
            self.fully_resolved &= op2.fully_resolved;
        }
        if let Some(op3) = self.operand3.as_mut() {
            op3.evaluate();
            self.fully_resolved &= op3.fully_resolved;
        }

        match self.expression_type {
            LPAREN => self.evaluate_paranthesized_expression(),
            RPAREN => self.evaluate_function_call_expression(),
            PLUS => self.evaluate_arithmetic_expression(),
            MINUS => self.evaluate_arithmetic_expression(),
            MULTIPLY => self.evaluate_arithmetic_expression(),
            DIVIDE => self.evaluate_arithmetic_expression(),
            MODULO => self.evaluate_arithmetic_expression(),
            BANG => self.evaluate_logical_negation(),
            TILDE => self.evaluate_bitwise_negation(),
            AND => self.evaluate_bitwise_expression(),
            OR => self.evaluate_bitwise_expression(),
            XOR => self.evaluate_bitwise_expression(),
            LSHIFT => self.evaluate_bitwise_expression(),
            RSHIFT => self.evaluate_bitwise_expression(),
            _ => panic!("unsupported expression type"),
        }
    }

    fn evaluate_paranthesized_expression(&mut self) {
        match &self.operand1 {
            Some(op1) => {
                // just pass through the expression content
                self.result_type = op1.result_type.clone();
            }
            _ => panic!("paranthesized expression requries one operator"),
        }
    }

    fn evaluate_function_call_expression(&mut self) {
        // TODO need to look up the symbol
    }

    fn evaluate_unary_arithmetic_expression(&mut self) {
        match &self.operand1 {
            Some(op1) => match op1.result_type {
                ExpressionType::Integer(value) => match self.expression_type {
                    PLUS => self.result_type = ExpressionType::Integer(value),
                    MINUS => self.result_type = ExpressionType::Integer(-value),
                    _ => panic!("unexpected unary integer expression"),
                },
                ExpressionType::Float(value) => match self.expression_type {
                    PLUS => self.result_type = ExpressionType::Float(value),
                    MINUS => self.result_type = ExpressionType::Float(-value),
                    _ => panic!("unexpected unary float expression"),
                },
                _ => {
                    panic!("logical negation expression can only be applied to boolean expressions")
                }
            },
            _ => panic!("logical negation expression requries one operator"),
        }
    }

    fn evaluate_arithmetic_expression(&mut self) {
        if self.operand2.is_none() {
            // an arithmetic expression may be +5 or -5, i.e. a sign
            // of a float or integer expression.
            return self.evaluate_unary_arithmetic_expression();
        }
        match (&self.operand1, &self.operand2) {
            (Some(op1), Some(op2)) => match (&op1.result_type, &op2.result_type) {
                (ExpressionType::Integer(value1), ExpressionType::Integer(value2)) => {
                    let result;
                    match self.expression_type {
                        PLUS => result = value1 + value2,
                        MINUS => result = value1 - value2,
                        MULTIPLY => result = value1 * value2,
                        DIVIDE => result = value1 / value2,
                        MODULO => result = value1 % value2,
                        _ => panic!("unexpected integer arithmetic expression"),
                    }
                    self.result_type = ExpressionType::Integer(result);
                }
                (ExpressionType::Float(value1), ExpressionType::Float(value2)) => {
                    let result;
                    match self.expression_type {
                        PLUS => result = value1 + value2,
                        MINUS => result = value1 - value2,
                        MULTIPLY => result = value1 * value2,
                        DIVIDE => result = value1 / value2,
                        _ => panic!("unexpected float arithmetic expression"),
                    }
                    self.result_type = ExpressionType::Float(result);
                }
                (ExpressionType::String(str1), ExpressionType::String(str2)) => {
                    match self.expression_type {
                        PLUS => {
                            self.result_type = ExpressionType::String(format!("{}{}", str1, str2))
                        }
                        _ => panic!("unexpected string arithmetic expression"),
                    }
                }
                _ => {
                    panic!("arithmetic expression can only be applied to integer, float or string operands")
                }
            },
            _ => panic!("arithmetic expression requries two operators"),
        }
    }

    fn evaluate_logical_negation(&mut self) {
        match &self.operand1 {
            Some(op1) => match op1.result_type {
                ExpressionType::Bool(value) => {
                    self.result_type = ExpressionType::Bool(!value);
                }
                _ => {
                    panic!("logical negation expression can only be applied to boolean expressions")
                }
            },
            _ => panic!("logical negation expression requries one operator"),
        }
    }

    fn evaluate_bitwise_negation(&mut self) {
        match &self.operand1 {
            Some(op1) => match op1.result_type {
                ExpressionType::Integer(value) => {
                    self.result_type = ExpressionType::Integer(!value);
                }
                _ => {
                    panic!("bitwise negation expression can only be applied to integer expressions")
                }
            },
            _ => panic!("bitwise negation expression requries one operator"),
        }
    }

    fn evaluate_bitwise_expression(&mut self) {
        match (&self.operand1, &self.operand2) {
            (Some(op1), Some(op2)) => match (&op1.result_type, &op2.result_type) {
                (ExpressionType::Integer(value1), ExpressionType::Integer(value2)) => {
                    let result;
                    match self.expression_type {
                        AND => result = value1 & value2,
                        OR => result = value1 | value2,
                        XOR => result = value1 ^ value2,
                        LSHIFT => result = value1 << value2,
                        RSHIFT => result = value1 >> value2,
                        _ => panic!("unexpected bitwise expression"),
                    }
                    self.result_type = ExpressionType::Integer(result);
                }
                _ => {
                    panic!("bitwise expression can only be applied to integer operands")
                }
            },
            _ => panic!("bitwise expression requries two operands"),
        }
    }
}
