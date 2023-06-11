use crate::internal::parser::gen::zserioparser::{AND, BANG, OR, XOR};
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
            BANG => self.evaluate_logical_negation(),
            AND => self.evaluate_bitwise_expression(),
            OR => self.evaluate_bitwise_expression(),
            XOR => self.evaluate_bitwise_expression(),
            _ => panic!("unsupported expression type"),
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

    fn evaluate_bitwise_expression(&mut self) {
        match (&self.operand1, &self.operand2) {
            (Some(op1), Some(op2)) => match (&op1.result_type, &op2.result_type) {
                (ExpressionType::Integer(value1), ExpressionType::Integer(value2)) => {
                    let mut result = 0;
                    match self.expression_type {
                        AND => result = value1 & value2,
                        OR => result = value1 | value2,
                        XOR => result = value1 ^ value2,
                        _ => panic!("unexpected bitwise expression"),
                    }
                    self.result_type = ExpressionType::Integer(result);
                }
                _ => {
                    panic!("logical negation expression can only be applied to boolean expressions")
                }
            },
            _ => panic!("logical negation expression requries one operator"),
        }
    }
}
