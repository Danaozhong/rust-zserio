use crate::internal::ast::expression::{
    EvaluationState, Expression, ExpressionFlag, ExpressionType,
};
use crate::internal::ast::field::Field;
use crate::internal::ast::parameter::Parameter;
use crate::internal::ast::zfunction::ZFunction;
use crate::internal::parser::gen::zserioparser::{DOT, ID};
use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

use crate::internal::compiler::symbol_scope::{ModelScope, PackageScope, Symbol};

#[derive(Clone, Debug)]
pub struct ZChoiceCase {
    pub conditions: Vec<Rc<RefCell<Expression>>>,
    pub field: Option<Rc<RefCell<Field>>>,
}

#[derive(Clone, Debug)]
pub struct ZChoice {
    pub name: String,
    pub template_parameters: Vec<String>,
    pub type_parameters: Vec<Rc<RefCell<Parameter>>>,
    pub selector_expression: Rc<RefCell<Expression>>,
    pub cases: Vec<ZChoiceCase>,
    pub default_case: Option<ZChoiceCase>,
    pub functions: Vec<Rc<RefCell<ZFunction>>>,
}

impl ZChoice {
    pub fn evaluate_selector_expression(&self, scope: &mut ModelScope) {
        // Ignore packages that are templated. They cannot be evaluated. Only their templated
        // instances will be evaluated.
        if !self.template_parameters.is_empty() {
            return;
        }

        self.selector_expression
            .as_ref()
            .borrow_mut()
            .evaluate(scope);
    }

    pub fn evaluate(&self, scope: &mut ModelScope) {
        // Ignore packages that are templated. They cannot be evaluated. Only their templated
        // instances will be evaluated.
        if !self.template_parameters.is_empty() {
            return;
        }

        for param in &self.type_parameters {
            param.as_ref().borrow().zserio_type.evaluate(scope);
        }
        for case in &self.cases {
            if let Some(case_field) = &case.field {
                case_field.as_ref().borrow_mut().evaluate(scope);
            }
            for case_condition in &case.conditions {
                case_condition.as_ref().borrow_mut().evaluate(scope);
            }
        }
        if let Some(default_field) = &self.default_case {
            if let Some(case_field) = &default_field.field {
                case_field.as_ref().borrow_mut().evaluate(scope);
            }
            for case_condition in &default_field.conditions {
                case_condition.as_ref().borrow_mut().evaluate(scope);
            }
        }
        for function in &self.functions {
            function.as_ref().borrow_mut().evaluate(scope);
        }
    }

    pub fn add_enumeration_type_prefix_to_choice_cases(&mut self) {
        // For enumerator parameters, this function will look for choice cases that are not fully
        // described, and replaces them by a dot expression.
        // This is needed to satisfy the zserio rule:
        // -- When the selector expression has an enumeration type, the enumeration type prefix
        // -- may be omitted from the case label literals.
        // An example would look like this:
        // case ENUM_VALUE:
        // will be replaced by:
        // case EnumType.ENUM_VALUE

        // Ignore packages that are templated. They cannot be evaluated. Only their templated
        // instances will be evaluated.
        if !self.template_parameters.is_empty() {
            return;
        }

        for case in &mut self.cases {
            for condition in &mut case.conditions.iter_mut() {
                let mut dot_expression = None;
                {
                    let condition_expression = condition.as_ref().borrow();
                    if condition_expression.expression_type == ID {
                        match &self
                            .selector_expression
                            .as_ref()
                            .borrow()
                            .symbol
                            .as_ref()
                            .unwrap()
                            .symbol
                        {
                            Symbol::Parameter(p) => {
                                let mut operand2 = Box::from(condition_expression.clone());
                                operand2.flag = ExpressionFlag::IsDotExpressionRightOperand;

                                dot_expression = Option::from(Expression {
                                    expression_type: DOT,
                                    flag: ExpressionFlag::None,
                                    operand1: Option::from(Box::new(Expression {
                                        expression_type: ID,
                                        text: p.as_ref().borrow().zserio_type.name.clone(),
                                        flag: ExpressionFlag::None,
                                        operand1: None,
                                        operand2: None,
                                        operand3: None,
                                        result_type: ExpressionType::Other,
                                        symbol: None,
                                        fully_resolved: false,
                                        evaluation_state: EvaluationState::NotEvaluated,
                                        native_type: None,
                                    })),
                                    operand2: Option::from(operand2),
                                    operand3: None,
                                    text: "".into(),
                                    result_type: ExpressionType::Other,
                                    symbol: None,
                                    fully_resolved: false,
                                    evaluation_state: EvaluationState::NotEvaluated,
                                    native_type: None,
                                });
                            }
                            _ => panic!("selector expression is not a parameter"),
                        }
                    }
                }
                if let Some(new_dot_expression) = dot_expression {
                    // TODO fully understand how the rust FnOnce() works. Using
                    // replace_with() would make this function non-mutable.
                    //condition.replace_with(|dot_expression| dot_expression.clone());
                    *condition = Rc::from(RefCell::from(new_dot_expression));
                }
            }
        }
    }
}

pub fn add_choice_to_scope(z_choice: &Rc<RefCell<ZChoice>>, package_scope: &mut PackageScope) {
    // Create a local scope, which contains all symbols within this structure
    let mut local_symbols = HashMap::new();
    for rc_param in z_choice.borrow().type_parameters.iter() {
        let param = rc_param.as_ref().borrow();
        local_symbols.insert(param.name.clone(), Symbol::Parameter(rc_param.clone()));
    }
    // Add the fields to the scope.
    for (_i, choice_case) in z_choice.borrow().cases.iter().enumerate() {
        if let Some(field_rc) = &choice_case.field {
            let field = field_rc.as_ref().borrow();
            local_symbols.insert(field.name.clone(), Symbol::Field(field_rc.clone()));
        }
    }
    if let Some(default_case) = &z_choice.borrow().default_case {
        if let Some(field_rc) = &default_case.field {
            let field = field_rc.as_ref().borrow();
            local_symbols.insert(field.name.clone(), Symbol::Field(field_rc.clone()));
        }
    }

    // add the functions to the local symbol scope
    for function in &z_choice.borrow().functions {
        local_symbols.insert(
            function.as_ref().borrow().name.clone(),
            Symbol::Function(function.clone()),
        );
    }

    package_scope
        .local_symbols
        .insert(z_choice.borrow().name.clone(), local_symbols);

    // Add the struct name itself to the package scope.
    package_scope.file_symbols.insert(
        z_choice.borrow().name.clone(),
        Symbol::Choice(z_choice.clone()),
    );
}
