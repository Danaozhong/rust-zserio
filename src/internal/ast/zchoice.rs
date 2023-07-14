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
pub struct ZChoiceCase {
    pub conditions: Vec<Rc<RefCell<Expression>>>,
    pub field: Option<Box<Field>>,
}

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
                case_field.evaluate(scope);
            }
            for case_condition in &case.conditions {
                case_condition.as_ref().borrow_mut().evaluate(scope);
            }
        }
        if let Some(default_field) = &self.default_case {
            if let Some(case_field) = &default_field.field {
                case_field.evaluate(scope);
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
                                    })),
                                    operand2: Option::from(operand2),
                                    operand3: None,
                                    text: "".into(),
                                    result_type: ExpressionType::Other,
                                    symbol: None,
                                    fully_resolved: false,
                                    evaluation_state: EvaluationState::NotEvaluated,
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
    //for (i, field) in z_choice.borrow().cases.iter().enumerate() {
    //local_symbols.insert(field.name.clone(), Symbol::Field(z_choice.clone(), i));
    //}
    package_scope
        .local_symbols
        .insert(z_choice.borrow().name.clone(), local_symbols);

    // Add the struct name itself to the package scope.
    package_scope.file_symbols.insert(
        z_choice.borrow().name.clone(),
        Symbol::Choice(z_choice.clone()),
    );
}
