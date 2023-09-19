use crate::internal::ast::expression::{
    EvaluationState, Expression, ExpressionFlag, ExpressionType,
};
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::{ModelScope, Symbol};
use crate::internal::generator::types::{
    convert_to_enum_field_name, to_rust_constant_name, TypeGenerator,
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

pub fn generate_boolean_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    let generated_code = generate_expression(expression, type_generator, scope);
    match expression.result_type {
        ExpressionType::BitMask(_) => format!("{} != 0", generated_code),
        ExpressionType::Integer(_) => format!("{} != 0", generated_code),
        _ => generated_code,
    }
}

pub fn generate_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    assert!(expression.evaluation_state == EvaluationState::Completed);
    match expression.expression_type {
        LPAREN => format!(
            "({})",
            generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope)
        ),
        RPAREN => format!(
            "{}()",
            generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope)
        ),
        LBRACKET => generate_bracketed_expression(expression, type_generator, scope),
        DOT => generate_dot_expression(expression, type_generator, scope),
        VALUEOF => generate_valueof_expression(expression, type_generator, scope),
        LENGTHOF => generate_lengthof_expression(expression, type_generator, scope),
        NUMBITS => generate_numbits_expression(expression, type_generator, scope),
        EQ | GE | GT | LE | LT | NE => {
            generate_comparison_expression(expression, type_generator, scope)
        }
        PLUS | MINUS | MULTIPLY | DIVIDE | MODULO => {
            generate_arithmetic_expression(expression, type_generator, scope)
        }
        QUESTIONMARK => generate_ternary_expression(expression, type_generator, scope),
        BANG => generate_logical_negation(expression, type_generator, scope),
        TILDE => generate_bitwise_negation(expression, type_generator, scope),
        AND | OR | XOR | LSHIFT | RSHIFT => {
            generate_bitwise_expression(expression, type_generator, scope)
        }
        LOGICAL_AND | LOGICAL_OR => generate_logical_expression(expression, type_generator, scope),
        ID => generate_identifier_expression(expression, type_generator),
        BOOL_LITERAL | OCTAL_LITERAL | HEXADECIMAL_LITERAL | BINARY_LITERAL | DECIMAL_LITERAL
        | FLOAT_LITERAL | DOUBLE_LITERAL | STRING_LITERAL => {
            generate_literal_expression(expression)
        }
        INDEX => generate_index_operator(),
        _ => panic!("unsupported expression type"),
    }
}

fn generate_unary_arithmetic_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "{}{}",
        match expression.expression_type {
            PLUS => "+",
            MINUS => "-",
            _ => panic!("unexpected unary arithmetic expression operator"),
        },
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope),
    )
}

fn generate_arithmetic_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    if expression.operand2.is_none() {
        // an arithmetic expression may be +5 or -5, i.e. a sign
        // of a float or integer expression.
        return generate_unary_arithmetic_expression(expression, type_generator, scope);
    }

    // Check if casts are required
    let mut op1 = generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope);
    let mut op2 = generate_expression(expression.operand2.as_ref().unwrap(), type_generator, scope);

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
        ExpressionType::String(_) => {
            // For string operations, using "+" sucks, due to the different issues with string
            // ownership, and mixing "String" with "str", reference, etc. The easiest
            // solution is to just use the format macro.
            assert!(expression.expression_type == PLUS);
            // Inception code
            return format!("format!(\"{{}}{{}}\", {}, {})", op1, op2,);
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
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "{}[{}]",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope),
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator, scope),
    )
}

fn is_bitmask_expression(expression: &Expression, scope: &ModelScope) -> bool {
    if let Some(expr_symbol) = &expression.symbol {
        let fund_type = match &expr_symbol.symbol {
            Symbol::Field(field) => get_fundamental_type(&field.borrow().field_type, scope),
            Symbol::Parameter(param) => get_fundamental_type(&param.borrow().zserio_type, scope),
            _ => return false,
        };
        if fund_type.fundamental_type.is_builtin {
            return false;
        }
        return matches!(
            scope.get_symbol(&fund_type.fundamental_type).symbol,
            Symbol::Bitmask(_)
        );
    }
    false
}
fn generate_dot_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    let op1 = expression.operand1.as_ref().unwrap();
    let op2 = expression.operand2.as_ref().unwrap();

    return match &op1.result_type {
        ExpressionType::Enum(_) => {
            let expr_symbol = op1.symbol.as_ref().unwrap();
            let enum_expression = format!(
                "{}::{}",
                type_generator.custom_type_to_rust_type(&expr_symbol.name),
                convert_to_enum_field_name(&op2.text)
            );
            type_generator.get_full_module_path(&expr_symbol.package, &enum_expression)
        }
        ExpressionType::BitMask(_) => {
            let bitmask_symbol = op1.symbol.as_ref().unwrap();
            let bitmask_expression = format!(
                "{}::{}",
                type_generator.to_rust_module_name(&bitmask_symbol.name),
                to_rust_constant_name(&op2.text)
            );
            type_generator.get_full_module_path(&bitmask_symbol.package, &bitmask_expression)
        }
        ExpressionType::Compound => {
            let left_operand = generate_expression(op1, type_generator, scope);
            let mut right_side = match op2.flag {
                ExpressionFlag::IsDotExpressionRightOperand => {
                    type_generator.convert_field_name(&op2.text)
                }
                _ => panic!("failed to generate right side of field dot expression"),
            };

            // Special handling for bitmask types. Bit masks are actually a struct, having a "bitmask"
            // value field. Hence, when generating bit masks, this field need to be referenced as well.
            if is_bitmask_expression(expression, scope) {
                right_side = format!("{}.bitmask_value", right_side);
            }
            format!("{}.{}", &left_operand, right_side,)
        }
        _ => panic!("unsupported dot expression {:?}", op1),
    };
}

fn generate_valueof_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "ztype::valueof({})",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope)
    )
}

fn generate_lengthof_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "({}.len() as u32)",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope)
    )
}

fn generate_numbits_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "(ztype::numbits({}) as u32)",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope)
    )
}

fn generate_identifier_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
) -> String {
    let symbol_ref = expression.symbol.as_ref().unwrap();

    // Workaround for bitmasks - bitmasks are wrapped in a special structure,
    // so when referencing them, we need to apply the wrapper.
    let bitmask_workaround = match &expression.result_type {
        ExpressionType::BitMask(_) => String::from(".bitmask_value"),
        _ => String::from(""),
    };

    // check for early returns, where no full type addressing is needed.
    match &symbol_ref.symbol {
        Symbol::Field(f) => {
            return format!(
                "self.{}{}",
                type_generator.convert_field_name(&f.borrow().name),
                &bitmask_workaround
            );
        }
        Symbol::Parameter(p) => {
            return format!(
                "self.{}{}",
                type_generator.convert_field_name(&p.borrow().name),
                &bitmask_workaround
            )
        }
        Symbol::Function(z_function) => {
            return format!(
                "self.{}",
                type_generator.convert_field_name(&z_function.borrow().name)
            )
        }
        _ => (),
    }

    let rust_symbol_name = match &symbol_ref.symbol {
        Symbol::Struct(s) => type_generator.custom_type_to_rust_type(&s.borrow().name),
        Symbol::Choice(c) => type_generator.custom_type_to_rust_type(&c.borrow().name),
        Symbol::Union(u) => type_generator.custom_type_to_rust_type(&u.borrow().name),
        Symbol::Enum(e) => type_generator.custom_type_to_rust_type(&e.borrow().name),
        Symbol::Bitmask(bitmask) => {
            type_generator.custom_type_to_rust_type(&bitmask.borrow().name) + ".bitmask_value"
        }
        Symbol::Const(zconst) => type_generator.constant_type_to_rust_type(&zconst.borrow().name),
        _ => panic!("unsupported identifier type {:?}", expression.symbol),
    };
    type_generator.get_full_module_path(&symbol_ref.package, &rust_symbol_name)
}

fn generate_ternary_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "
if {} {{
    {}
}} else {{
    {}
}}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope),
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator, scope),
        generate_expression(expression.operand3.as_ref().unwrap(), type_generator, scope),
    )
}

fn generate_logical_negation(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "!{}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope)
    )
}

fn generate_bitwise_negation(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "~{}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope)
    )
}

fn generate_bitwise_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope),
        match expression.expression_type {
            AND => "&",
            OR => "|",
            XOR => "^",
            LSHIFT => "<<",
            RSHIFT => ">>",
            _ => panic!("unexpected bitwise expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator, scope),
    )
}

fn generate_logical_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope),
        match expression.expression_type {
            LOGICAL_AND => "&&",
            LOGICAL_OR => "||",
            _ => panic!("unexpected logical expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator, scope),
    )
}

fn generate_comparison_expression(
    expression: &Expression,
    type_generator: &mut TypeGenerator,
    scope: &ModelScope,
) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap(), type_generator, scope),
        match expression.expression_type {
            EQ => "==",
            LE => "<=",
            LT => "<",
            GT => ">",
            GE => ">=",
            NE => "!=",
            _ => panic!("unexpected comparison expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap(), type_generator, scope),
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
        STRING_LITERAL => {
            if expression.fully_resolved {
                return format!("\"{}\"", &string_value);
            }
            // need to cast into a rust string
            format!("\"{}\".into()", &string_value)
        }

        _ => panic!("unexpected comparison expression operator"),
    }
}

fn generate_index_operator() -> String {
    // Make sure this variable name matches the one used in encode/decode code generators
    // where the parameters are passed.
    "param_index".into()
}
