use crate::internal::ast::expression::{
    EvaluationState, Expression, ExpressionFlag, ExpressionType,
};
use crate::internal::compiler::symbol_scope::Symbol;
use crate::internal::generator::types::{
    constant_type_to_rust_type, convert_field_name, convert_to_enum_field_name,
    custom_type_to_rust_type, TypeGenerator,
};
use crate::internal::parser::gen::zserioparser::{
    AND, BANG, BINARY_LITERAL, BOOL_LITERAL, DECIMAL_LITERAL, DIVIDE, DOT, DOUBLE_LITERAL, EQ,
    FLOAT_LITERAL, GE, GT, HEXADECIMAL_LITERAL, ID, INDEX, LBRACKET, LE, LENGTHOF, LOGICAL_AND,
    LOGICAL_OR, LPAREN, LSHIFT, LT, MINUS, MODULO, MULTIPLY, NE, NUMBITS, OCTAL_LITERAL, OR, PLUS,
    QUESTIONMARK, RPAREN, RSHIFT, STRING_LITERAL, TILDE, VALUEOF, XOR,
};

pub struct ExpressionGenerationResult {
    pub generated_value: String,
    pub is_lvalue: bool,
    pub lvalue_name: String,
}

pub fn generate_expression(expression: &Expression, type_generator: &TypeGenerator) -> String {
    assert!(expression.evaluation_state == EvaluationState::Completed);
    match expression.expression_type {
        LPAREN => format!(
            "({})",
            generate_expression(expression.operand1.as_ref().unwrap(), type_generator)
        ),
        RPAREN => format!(
            "{}()",
            generate_expression(expression.operand1.as_ref().unwrap(), type_generator)
        ),
        LBRACKET => generate_bracketed_expression(expression, type_generator),
        DOT => generate_dot_expression(expression, type_generator),
        VALUEOF => generate_valueof_expression(expression, type_generator),
        LENGTHOF => generate_lengthof_expression(expression, type_generator),
        NUMBITS => generate_numbits_expression(expression, type_generator),
        EQ | GE | GT | LE | LT | NE => generate_comparison_expression(expression, type_generator),
        PLUS | MINUS | MULTIPLY | DIVIDE | MODULO => {
            generate_arithmetic_expression(expression, type_generator)
        }
        QUESTIONMARK => generate_ternary_expression(expression, type_generator),
        BANG => generate_logical_negation(expression, type_generator),
        TILDE => generate_bitwise_negation(expression, type_generator),
        AND | OR | XOR | LSHIFT | RSHIFT => generate_bitwise_expression(expression, type_generator),
        LOGICAL_AND | LOGICAL_OR => generate_logical_expression(expression, type_generator),
        ID => generate_identifier_expression(expression, type_generator),
        BOOL_LITERAL | OCTAL_LITERAL | HEXADECIMAL_LITERAL | BINARY_LITERAL | DECIMAL_LITERAL
        | FLOAT_LITERAL | DOUBLE_LITERAL | STRING_LITERAL => {
            generate_literal_expression(expression)
        }
        INDEX => generate_index_operator(),
        /*
        0xFFFFF => (), // Ignore
         */
        _ => panic!("unsupported expression type"),
    }
}

fn generate_unary_arithmetic_expression(
    expression: &Expression,
    type_generator: &TypeGenerator,
) -> String {
    format!(
        "{}{}",
        match expression.expression_type {
            PLUS => "+",
            MINUS => "-",
            _ => panic!("unexpected unary arithmetic expression operator"),
        },
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator),
    )
}

fn generate_arithmetic_expression(
    expression: &Expression,
    type_generator: &TypeGenerator,
) -> String {
    if expression.operand2.is_none() {
        // an arithmetic expression may be +5 or -5, i.e. a sign
        // of a float or integer expression.
        return generate_unary_arithmetic_expression(expression, type_generator);
    }

    // Check if casts are required
    let mut op1 = generate_expression(expression.operand1.as_ref().unwrap(), type_generator);
    let mut op2 = generate_expression(expression.operand2.as_ref().unwrap(), type_generator);

    let mut expression_type = None;
    let mut op1_rust_type = None;
    let mut op2_rust_type = None;

    // Check if casting is needed because of different original types.
    // Casting is needed in the following cases:
    // - mixing floats with integers
    // - mixing signed/unsigned types
    // - mixing different bit lengths
    // - mixing numerals of different types
    // Casting is not needed in the following cases:
    // - The expression result is not an integer.
    // - One or both operands are numerals without type annotations, for example
    //   "17.5 + 13.6"
    match &expression.result_type {
        ExpressionType::Integer(_) | ExpressionType::BitMask(_) | ExpressionType::Float(_) => {
            if let Some(native_type) = &expression.native_type {
                expression_type = Some(type_generator.ztype_to_rust_type(native_type));
                if let Some(op1_native_type) = &expression.operand1.as_ref().unwrap().native_type {
                    op1_rust_type = Some(type_generator.ztype_to_rust_type(op1_native_type));
                }
                if let Some(op2_native_type) = &expression.operand2.as_ref().unwrap().native_type {
                    op2_rust_type = Some(type_generator.ztype_to_rust_type(op2_native_type));
                }
            }
        }
        _ => (),
    };

    // Generate the type casts
    if let Some(result_type) = expression_type {
        if let Some(op1_result_type) = op1_rust_type {
            if result_type != op1_result_type {
                op1 = format!("(({}) as {})", op1, result_type);
            }
        }
        if let Some(op2_result_type) = op2_rust_type {
            if result_type != op2_result_type {
                op2 = format!("(({}) as {})", op2, result_type);
            }
        }
    }

    format!(
        "{} {} {}",
        op1,
        match expression.expression_type {
            PLUS => "+",
            MINUS => "-",
            MULTIPLY => "*",
            DIVIDE => "/",
            MODULO => "%",
            _ => panic!("unexpected arithmetic expression operator"),
        },
        op2,
    )
}

fn generate_bracketed_expression(
    expression: &Expression,
    type_generator: &TypeGenerator,
) -> String {
    format!(
        "{}[{}]",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator),
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator),
    )
}

fn generate_dot_expression(expression: &Expression, type_generator: &TypeGenerator) -> String {
    let op1 = expression.operand1.as_ref().unwrap();

    return match &op1.result_type {
        ExpressionType::Enum(z_enum) => {
            let enum_expression = format!(
                "{}::{}",
                custom_type_to_rust_type(&z_enum.borrow().name),
                convert_to_enum_field_name(&expression.operand2.as_ref().unwrap().text)
            );
            type_generator
                .get_full_module_path(&op1.symbol.as_ref().unwrap().package, &enum_expression)
        }
        ExpressionType::BitMask(z_bitmask) => {
            format!(
                "{}::{}",
                custom_type_to_rust_type(&z_bitmask.borrow().name),
                convert_to_enum_field_name(&expression.operand2.as_ref().unwrap().text)
            )
        }
        ExpressionType::Compound => {
            match &op1
                .symbol
                .as_ref()
                .expect("failed to retrieve the symbol of compound expression")
                .symbol
            {
                Symbol::Field(z_field) => {
                    let op2 = expression
                        .operand2
                        .as_ref()
                        .expect("a dot expression must have two operands");

                    let right_side = match op2.flag {
                        ExpressionFlag::IsDotExpressionRightOperand => {
                            convert_field_name(&op2.text)
                        }
                        _ => panic!("failed to generate right side of field dot expression"),
                    };

                    format!(
                        "self.{}.{}",
                        convert_field_name(&z_field.borrow().name),
                        right_side,
                    )
                }
                Symbol::Parameter(z_param) => {
                    let op2 = expression
                        .operand2
                        .as_ref()
                        .expect("a dot expression must have two operands");

                    let right_side = match op2.flag {
                        ExpressionFlag::IsDotExpressionRightOperand => {
                            convert_field_name(&op2.text)
                        }
                        _ => panic!("failed to generate right side of field dot expression"),
                    };
                    format!(
                        "self.{}.{}",
                        convert_field_name(&z_param.borrow().name),
                        right_side,
                    )
                }
                _ => panic!(
                    "unsupported symbol type for dot expression generation {:?}",
                    op1.symbol
                ),
            }
        }
        _ => panic!("unsupported dot expression {:?}", op1),
    };
}

fn generate_valueof_expression(expression: &Expression, type_generator: &TypeGenerator) -> String {
    format!(
        "ztype::valueof({})",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator)
    )
}

fn generate_lengthof_expression(expression: &Expression, type_generator: &TypeGenerator) -> String {
    format!(
        "({}.len() as u32)",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator)
    )
}

fn generate_numbits_expression(expression: &Expression, type_generator: &TypeGenerator) -> String {
    format!(
        "({} as u32)",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator)
    )
}

fn generate_identifier_expression(
    expression: &Expression,
    type_generator: &TypeGenerator,
) -> String {
    let symbol_ref = expression.symbol.as_ref().unwrap();

    // check for early returns, where no full type addressing is needed.
    match &symbol_ref.symbol {
        Symbol::Field(f) => return format!("self.{}", convert_field_name(&f.borrow().name)),
        Symbol::Parameter(p) => return format!("self.{}", convert_field_name(&p.borrow().name)),
        Symbol::Function(z_function) => {
            return format!("self.{}", convert_field_name(&z_function.borrow().name))
        }
        _ => (),
    }

    let rust_symbol_name = match &symbol_ref.symbol {
        Symbol::Struct(s) => custom_type_to_rust_type(&s.borrow().name),
        Symbol::Choice(c) => custom_type_to_rust_type(&c.borrow().name),
        Symbol::Union(u) => custom_type_to_rust_type(&u.borrow().name),
        Symbol::Enum(e) => custom_type_to_rust_type(&e.borrow().name),
        Symbol::Bitmask(bitmask) => custom_type_to_rust_type(&bitmask.borrow().name),
        Symbol::Const(zconst) => constant_type_to_rust_type(&zconst.borrow().name),
        _ => panic!("unsupported identifier type {:?}", expression.symbol),
    };
    type_generator.get_full_module_path(&symbol_ref.package, &rust_symbol_name)
}

fn generate_ternary_expression(expression: &Expression, type_generator: &TypeGenerator) -> String {
    format!(
        "
if {} {{
    {}
}} else {{
    {}
}}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator),
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator),
        generate_expression(expression.operand3.as_ref().unwrap(), type_generator),
    )
}

fn generate_logical_negation(expression: &Expression, type_generator: &TypeGenerator) -> String {
    format!(
        "!{}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator)
    )
}

fn generate_bitwise_negation(expression: &Expression, type_generator: &TypeGenerator) -> String {
    format!(
        "~{}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator)
    )
}

fn generate_bitwise_expression(expression: &Expression, type_generator: &TypeGenerator) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator),
        match expression.expression_type {
            AND => "&",
            OR => "|",
            XOR => "^",
            LSHIFT => "<<",
            RSHIFT => ">>",
            _ => panic!("unexpected bitwise expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator),
    )
}

fn generate_logical_expression(expression: &Expression, type_generator: &TypeGenerator) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator),
        match expression.expression_type {
            LOGICAL_AND => "&&",
            LOGICAL_OR => "||",
            _ => panic!("unexpected logical expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator),
    )
}

fn generate_comparison_expression(
    expression: &Expression,
    type_generator: &TypeGenerator,
) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator),
        match expression.expression_type {
            EQ => "==",
            LE => "<=",
            LT => "<",
            GT => ">",
            GE => ">=",
            NE => "!=",
            _ => panic!("unexpected comparison expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator),
    )
}

fn generate_literal_expression(expression: &Expression) -> String {
    let mut float_value = 0.0;
    let mut int_value = 0;
    let mut bool_value = false;
    let mut string_value = String::from("");
    match &expression.result_type {
        ExpressionType::Integer(v) => int_value = *v,
        ExpressionType::Bool(b) => bool_value = *b,
        ExpressionType::Float(f) => float_value = *f,
        ExpressionType::String(s) => string_value = s.clone(),
        _ => panic!(),
    };

    match expression.expression_type {
        DECIMAL_LITERAL => i32::to_string(&int_value),
        HEXADECIMAL_LITERAL => format!("{int_value:#x}"),
        OCTAL_LITERAL => format!("{int_value:#o}"),
        FLOAT_LITERAL | DOUBLE_LITERAL => format!("{}", &float_value),
        BOOL_LITERAL => {
            if bool_value {
                String::from("true")
            } else {
                String::from("false")
            }
        }
        STRING_LITERAL => format!("\"{}\".into()", &string_value),

        _ => panic!("unexpected comparison expression operator"),
    }
}

fn generate_index_operator() -> String {
    // Make sure this variable name matches the one used in encode/decode code generators
    // where the parameters are passed.
    "param_index".into()
}
