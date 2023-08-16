use crate::internal::ast::zbitmask::ZBitmaskType;

use crate::internal::compiler::symbol_scope::{ModelScope, ScopeLocation, Symbol, SymbolReference};
use crate::internal::parser::gen::zserioparser::{
    AND, BANG, DIVIDE, DOT, EQ, GE, GT, ID, INDEX, LBRACKET, LE, LENGTHOF, LOGICAL_AND, LOGICAL_OR,
    LPAREN, LSHIFT, LT, MINUS, MODULO, MULTIPLY, NE, NUMBITS, OR, PLUS, QUESTIONMARK, RPAREN,
    RSHIFT, TILDE, VALUEOF, XOR,
};

use crate::internal::ast::{field::Field, zenum::ZEnum, zstruct::ZStruct};
use std::cell::RefCell;
use std::rc::Rc;
use std::string::String;

use super::type_reference::TypeReference;
use super::zfunction::ZFunction;

#[derive(Clone)]
pub enum CompoundExpressionType {
    Field(Rc<RefCell<Field>>),
    ZStruct(Rc<RefCell<ZStruct>>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum ExpressionFlag {
    None,
    IsDotExpressionRightOperand,
}

#[derive(Clone, Debug)]
pub enum ExpressionType {
    Integer(i32),
    Float(f64),
    String(String),
    Bool(bool),
    BitMask(Rc<RefCell<ZBitmaskType>>),
    Enum(Rc<RefCell<ZEnum>>),
    Compound,
    // TODO consider if it is possible to remove Function and use Compound().
    Function(Rc<RefCell<ZFunction>>),
    Other,
}

#[derive(PartialEq, Clone, Debug)]
pub enum EvaluationState {
    NotEvaluated,
    InProgress,
    Completed,
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expression_type: isize,
    pub text: String,
    pub flag: ExpressionFlag,
    pub operand1: Option<Box<Expression>>,
    pub operand2: Option<Box<Expression>>,
    pub operand3: Option<Box<Expression>>,
    pub result_type: ExpressionType,
    pub symbol: Option<Symbol>,

    // Flag which indicates that the expression is fully resolved
    pub fully_resolved: bool,
    pub evaluation_state: EvaluationState,
}

impl Expression {
    pub fn evaluate(&mut self, scope: &mut ModelScope) {
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
            op1.evaluate(scope);
            self.fully_resolved &= op1.fully_resolved;
        }
        if let Some(op2) = self.operand2.as_mut() {
            op2.evaluate(scope);
            self.fully_resolved &= op2.fully_resolved;
        }
        if let Some(op3) = self.operand3.as_mut() {
            op3.evaluate(scope);
            self.fully_resolved &= op3.fully_resolved;
        }

        match self.expression_type {
            LPAREN => self.evaluate_paranthesized_expression(),
            RPAREN => self.evaluate_function_call_expression(scope),
            LBRACKET => self.evaluate_array_element(scope),
            DOT => self.evaluate_dot_expression(scope),
            LENGTHOF => self.evaluate_lengthof_operator(),
            VALUEOF => self.evaluate_valueof_operator(),
            NUMBITS => self.evaluate_numbits_operator(),
            EQ | GE | GT | LE | LT | NE => self.evaluate_comparison_expression(),
            PLUS | MINUS | MULTIPLY | DIVIDE | MODULO => self.evaluate_arithmetic_expression(),
            BANG => self.evaluate_logical_negation(),
            TILDE => self.evaluate_bitwise_negation(),
            AND | OR | XOR | LSHIFT | RSHIFT => self.evaluate_bitwise_expression(),
            LOGICAL_OR | LOGICAL_AND => self.evaluate_logical_expression(),
            QUESTIONMARK => self.evaluate_ternary_expression(),
            INDEX => self.evaluate_index_expression(),
            ID => self.evaluate_identifier_expression(scope),
            0xFFFFF => (), // Ignore
            _ => panic!("unsupported expression type"),
        }
        self.evaluation_state = EvaluationState::Completed;
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

    fn evaluate_function_call_expression(&mut self, scope: &mut ModelScope) {
        match &self.operand1.as_ref().unwrap().result_type {
            ExpressionType::Function(func) => {
                self.result_type = type_reference_to_expression_type(
                    func.as_ref().borrow().return_type.as_ref(),
                    scope,
                );
                self.symbol = Option::from(Symbol::Function(func.clone()));
            }
            _ => panic!(
                "expression is not a function, but {:?}",
                &self.operand1.as_ref().unwrap()
            ),
        }
    }

    fn evaluate_array_element(&mut self, _scope: &mut ModelScope) {
        match &self.operand1 {
            Some(op1) => {
                match &op1.result_type {
                    ExpressionType::Compound => {
                        // TODO
                    }
                    _ => panic!("todo"),
                }
            }
            _ => panic!("array element requires an operand to be set"),
        }
    }

    fn evaluate_dot_expression(&mut self, scope: &mut ModelScope) {
        match (&self.operand1, &self.operand2) {
            (Some(op1), Some(_op2)) => match &op1.result_type {
                ExpressionType::Enum(_z_enum) => {
                    self.evaluate_enum_dot_expression();
                }
                ExpressionType::BitMask(_) => {
                    self.evaluate_bitmask_dot_expression();
                }
                ExpressionType::Compound => {
                    self.evaluate_compound_dot_expression(scope);
                }
                _ => {
                    panic!("unexpected dot expression type {:?}", &op1.result_type);
                }
            },
            _ => panic!("arithmetic expression requries two operators"),
        }
    }

    fn evaluate_enum_dot_expression(&mut self) {
        match (&self.operand1, &self.operand2) {
            (Some(op1), Some(op2)) => match &op1.result_type {
                ExpressionType::Enum(z_enum) => {
                    for z_enum_field in &z_enum.as_ref().borrow().items {
                        if z_enum_field.name == op2.text {
                            self.result_type = ExpressionType::Integer(0);
                            return;
                        }
                    }
                    panic!(
                        "enum value {} not found in enum {}",
                        &z_enum.as_ref().borrow().name,
                        &op2.text
                    );
                }
                _ => panic!(),
            },
            _ => panic!("enum dot expression requries two operators"),
        }
    }

    fn evaluate_bitmask_dot_expression(&mut self) {
        match (&self.operand1, &self.operand2) {
            (Some(op1), Some(op2)) => match &op1.result_type {
                ExpressionType::BitMask(z_bitmask) => {
                    for bitmask_value in &z_bitmask.borrow().values {
                        if bitmask_value.name == op2.text {
                            self.result_type = ExpressionType::Integer(0);
                            self.fully_resolved = false;
                            return;
                        }
                    }
                    panic!(
                        "bitmask value {} not found in bitmask {}",
                        &z_bitmask.as_ref().borrow().name,
                        &op2.text
                    );
                }
                _ => panic!(),
            },
            _ => panic!("a bitmask dot expression requires two operators"),
        }
    }

    fn evaluate_compound_dot_expression(
        &mut self,
        //compound_symbol: &Symbol,
        scope: &mut ModelScope,
    ) {
        let type_ref = match self.operand1.as_ref().unwrap().symbol.as_ref().unwrap() {
            Symbol::Field(z_field) => z_field.borrow().field_type.clone(),
            Symbol::Parameter(param) => {
                // A parameter dot expression happens when inside a choice or enum,
                // the type paremter is called, and the type paramter is of
                // compound type (struct or choice).
                param.as_ref().borrow().zserio_type.clone()
            }
            _ => panic!(
                "unsupported compound type symbol parameter {:?}",
                self.operand1.as_ref().unwrap().symbol.as_ref().unwrap()
            ),
        };

        let compound_symbol = scope.resolve_symbol(&type_ref.name);
        scope.scope_stack.push(ScopeLocation {
            package: compound_symbol.package.clone(),
            import_symbol: None,
            symbol_name: Option::from(compound_symbol.name.clone()),
        });

        // resolve the symbol that belongs to the compound expression.
        // it can be <struct>.<field>, for example.
        let compound_symbol: SymbolReference =
            scope.resolve_symbol(&self.operand2.as_ref().unwrap().text);

        self.result_type = symbol_to_expression_type(&compound_symbol, scope);
        self.symbol = Option::from(compound_symbol.symbol.clone());

        // The scope of the compound symbol may be different to the original
        // scope, so the scope will be restored after the expression evaluation
        // is complete.
        // This is the case if struct A in file A uses struct B from file B,
        // then in order to evaluate struct A entirely, the scope needs to shift
        // to struct B temporarily.
        scope.scope_stack.pop();
    }

    fn evaluate_lengthof_operator(&mut self) {
        self.fully_resolved = false;
        self.result_type = ExpressionType::Integer(0);
        /*
        match self.operand1.unwrap().symbol.unwrap() {
            Symbol::Field(s, index) => {
                match &s.as_ref().borrow().fields[index].array {
                    Some(array) => {
                        if let Some(array_length_expr) = array.array_length_expression {

                        }
                    }
                    _ => panic!(""),
                }
            }
            _ => panic!("not implemented")
        }
         */
    }
    fn evaluate_valueof_operator(&mut self) {
        match &self.operand1 {
            Some(op1) => match op1.result_type {
                ExpressionType::BitMask(_) | ExpressionType::Enum(_) => {
                    self.result_type = ExpressionType::Integer(0);
                    self.fully_resolved = false;
                }
                _ => panic!("valueof operator can only be applied to bitmask or enum expressions, but was called for {:?}", op1)
            },
            _ => panic!("valueof operator expression requries one operator"),
        }
    }

    fn evaluate_numbits_operator(&mut self) {
        match &self.operand1 {
            Some(op1) => match op1.result_type {
                ExpressionType::Integer(value) => {
                    self.result_type =
                        ExpressionType::Integer(32 - i32::leading_zeros(value) as i32);
                    self.fully_resolved = op1.fully_resolved;
                }
                _ => {
                    panic!("numbits operator can only be applied to integer expressions")
                }
            },
            _ => panic!("numbits operator expression requries one operator"),
        }
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

    fn evaluate_integer_comparison_expression(&mut self, mut value1: i32, mut value2: i32) {
        if !self.operand1.as_ref().unwrap().fully_resolved {
            value1 = 1;
        }
        if !self.operand2.as_ref().unwrap().fully_resolved {
            value2 = 1;
        }

        self.result_type = ExpressionType::Bool(match self.expression_type {
            EQ => value1 == value2,
            GE => value1 >= value2,
            GT => value1 > value2,
            LE => value1 <= value2,
            LT => value1 < value2,
            NE => value1 != value2,
            _ => panic!("unexpected integer comparison"),
        })
    }

    fn evaluate_comparison_expression(&mut self) {
        match (&self.operand1, &self.operand2) {
            (Some(op1), Some(op2)) => {
                match (&op1.result_type, &op2.result_type) {
                    (ExpressionType::Integer(value1), ExpressionType::Integer(value2)) => {
                        self.evaluate_integer_comparison_expression(*value1, *value2)
                    }
                    (ExpressionType::BitMask(_), ExpressionType::Integer(value2)) => {
                        self.evaluate_integer_comparison_expression(1, *value2)
                    }
                    (ExpressionType::Float(value1), ExpressionType::Float(value2)) => {
                        self.result_type = ExpressionType::Bool(match self.expression_type {
                            EQ => value1 == value2,
                            GE => value1 >= value2,
                            GT => value1 > value2,
                            LE => value1 <= value2,
                            LT => value1 < value2,
                            NE => value1 != value2,
                            _ => panic!("unexpected float comparison"),
                        })
                    }
                    (ExpressionType::String(value1), ExpressionType::String(value2)) => {
                        self.result_type = ExpressionType::Bool(match self.expression_type {
                            EQ => value1 == value2,
                            NE => value1 != value2,
                            _ => panic!("unexpected string comparison"),
                        })
                    }
                    _ => {
                        if self.expression_type != EQ && self.expression_type != NE {
                            panic!("comparsion expression can only be applied to integer, float, or string types, but the provided types are {:?} and {:?}", &op1.result_type, &op2.result_type);
                        }

                        // TODO it may be needed to evaluate equal/nonequal comparisons
                        // during compile time. For the time being, we set the output to
                        // not resolved, so that the code generator will re-generate the
                        // same expression in the rust code.
                        self.result_type = ExpressionType::Bool(false);
                        self.fully_resolved = false;
                    }
                }
            }
            _ => panic!("comparsion expression requries two operands"),
        }
    }

    fn evaluate_float_arithemtic_expression(&mut self, mut value1: f64, mut value2: f64) {
        if !self.operand1.as_ref().unwrap().fully_resolved {
            value1 = 1.0;
        }
        if !self.operand2.as_ref().unwrap().fully_resolved {
            value2 = 1.0;
        }
        let result = match self.expression_type {
            PLUS => value1 + value2,
            MINUS => value1 - value2,
            MULTIPLY => value1 * value2,
            DIVIDE => value1 / value2,
            _ => panic!("unexpected float arithmetic expression"),
        };
        self.result_type = ExpressionType::Float(result);
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
                    let mut v1 = *value1;
                    let mut v2 = *value2;
                    if !op1.fully_resolved {
                        v1 = 1;
                    }
                    if !op2.fully_resolved {
                        v2 = 1;
                    }
                    let result = match self.expression_type {
                        PLUS => v1 + v2,
                        MINUS => v1 - v2,
                        MULTIPLY => v1 * v2,
                        DIVIDE => v1 / v2,
                        MODULO => v1 % v2,
                        _ => panic!("unexpected integer arithmetic expression"),
                    };
                    self.result_type = ExpressionType::Integer(result);
                }
                // Float arithmetic expressions can be mixed with integer types.
                // If one operand is a float type, the result will also be a float type.
                (ExpressionType::Float(value1), ExpressionType::Float(value2)) => {
                    self.evaluate_float_arithemtic_expression(*value1, *value2);
                }
                (ExpressionType::Integer(value1), ExpressionType::Float(value2)) => {
                    self.evaluate_float_arithemtic_expression(*value1 as f64, *value2);
                }
                (ExpressionType::Float(value1), ExpressionType::Integer(value2)) => {
                    self.evaluate_float_arithemtic_expression(*value1, *value2 as f64);
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
                    let result = match self.expression_type {
                        AND => value1 & value2,
                        OR => value1 | value2,
                        XOR => value1 ^ value2,
                        LSHIFT => value1 << value2,
                        RSHIFT => value1 >> value2,
                        _ => panic!("unexpected bitwise expression"),
                    };
                    self.result_type = ExpressionType::Integer(result);
                }
                (ExpressionType::BitMask(_), ExpressionType::Integer(_))
                | (ExpressionType::Integer(_), ExpressionType::BitMask(_))
                | (ExpressionType::BitMask(_), ExpressionType::BitMask(_)) => {
                    // For bitmasks, we won't evaluate the values yet.
                    self.result_type = ExpressionType::Integer(0);
                    self.fully_resolved = false;
                }
                _ => {
                    panic!("bitwise expression can only be applied to integer operands, but got {:?} and {:?}", op1, op2)
                }
            },
            _ => panic!("bitwise expression requries two operands"),
        }
    }

    fn evaluate_logical_expression(&mut self) {
        self.result_type = ExpressionType::Bool(match (&self.operand1, &self.operand2) {
            (Some(op1), Some(op2)) => match (&op1.result_type, &op2.result_type) {
                (ExpressionType::Bool(v1), ExpressionType::Bool(v2)) => {
                    match self.expression_type {
                        LOGICAL_AND => *v1 && *v2,
                        LOGICAL_OR => *v1 || *v2,
                        _ => panic!("unexpected logical expression"),
                    }
                }
                _ => {
                    panic!("logical expression can only be applied to bool operands, but received {:?} and {:?}", op1, op2)
                }
            },
            _ => panic!("logical expression requries two operands"),
        });
    }

    fn evaluate_ternary_expression(&mut self) {
        match self.operand1.as_ref().unwrap().result_type {
            ExpressionType::Bool(condition) => {
                if condition {
                    self.result_type = self.operand2.as_ref().unwrap().result_type.clone();
                    self.symbol = self.operand2.as_ref().unwrap().symbol.clone();
                    self.fully_resolved = self.operand2.as_ref().unwrap().fully_resolved;
                } else {
                    self.result_type = self.operand3.as_ref().unwrap().result_type.clone();
                    self.symbol = self.operand3.as_ref().unwrap().symbol.clone();
                    self.fully_resolved = self.operand3.as_ref().unwrap().fully_resolved;
                }
            }
            _ => panic!("the first operand in a ternary expression must have a boolean type, but it has {:?}", self.operand1.as_ref().unwrap()),
        }
    }

    fn evaluate_index_expression(&mut self) {
        self.result_type = ExpressionType::Integer(0);
        self.fully_resolved = false;
    }

    fn evaluate_identifier_expression(&mut self, scope: &mut ModelScope) {
        if self.flag == ExpressionFlag::IsDotExpressionRightOperand {
            // The right-hand part of a dot expression, e.g. <ENUM>.<ENUM_VALUE>
            // cannot be evaluated without knowledge of the left operand. Evaluation
            // of the right hand operand of the expression is stopped here, and the
            // value will be evaluated when the complete dot expression is evaluated.
            return;
        }
        let symbol_reference = scope.resolve_symbol(&self.text);
        self.symbol = Option::from(symbol_reference.symbol.clone());
        self.result_type = symbol_to_expression_type(&symbol_reference, scope);
        self.fully_resolved = false;
    }
}

fn symbol_to_expression_type(
    symbol_reference: &SymbolReference,
    scope: &mut ModelScope,
) -> ExpressionType {
    match &symbol_reference.symbol {
        Symbol::Const(z_const) => {
            return type_reference_to_expression_type(
                &z_const.as_ref().borrow().zserio_type,
                scope,
            );
        }
        Symbol::Bitmask(z_bitmask) => ExpressionType::BitMask(z_bitmask.clone()),
        Symbol::Union(_) => ExpressionType::Compound,
        Symbol::Struct(_) => ExpressionType::Compound,
        Symbol::Choice(_) => ExpressionType::Compound,
        Symbol::Enum(z_enum) => ExpressionType::Enum(z_enum.clone()),
        Symbol::Field(field) => {
            return type_reference_to_expression_type(&field.borrow().field_type, scope);
        }
        Symbol::Parameter(param) => {
            return type_reference_to_expression_type(&param.as_ref().borrow().zserio_type, scope);
        }
        Symbol::EnumItem(z_enum, _) => {
            // The index doesn't really matter, as each enum value has the same type.
            // We are only interested in the type at the moment. The actual value will
            // be applied in the generated code.
            return type_reference_to_expression_type(&z_enum.as_ref().borrow().enum_type, scope);
        }
        Symbol::Subtype(subtype) => {
            return type_reference_to_expression_type(
                &subtype.as_ref().borrow().zserio_type,
                scope,
            );
        }
        Symbol::Function(z_function) => ExpressionType::Function(z_function.clone()),
    }
}

fn type_reference_to_expression_type(
    type_ref: &TypeReference,
    scope: &mut ModelScope,
) -> ExpressionType {
    if type_ref.is_builtin {
        // The return values set by this function are all dummy
        // values, because this function only returns the type,
        // not the actual value. Since this value is not hard-coded,
        // it will be retrieved after code generation.
        if type_ref.name.as_str() == "string" {
            ExpressionType::String("".into())
        } else if type_ref.name.as_str() == "bool" {
            return ExpressionType::Bool(false);
        } else if type_ref.name.starts_with("float") {
            return ExpressionType::Float(0.0);
        } else if type_ref.name.starts_with("bit")
            || type_ref.name.starts_with("int")
            || type_ref.name.starts_with("uint")
            || type_ref.name.starts_with("varsize")
            || type_ref.name.starts_with("varuint")
            || type_ref.name.starts_with("varint")
        {
            return ExpressionType::Integer(0);
        } else {
            panic!("failed to determine builtin type reference type");
        }
    } else {
        // Resolve the symbol recursively, until a fundamental type is found.
        symbol_to_expression_type(&scope.resolve_symbol(&type_ref.name), scope)
    }
}
