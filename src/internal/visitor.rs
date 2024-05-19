use crate::internal::ast::package::{ZImport, ZPackage};
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::ast::zconst::ZConst;
use crate::internal::ast::zenum::{ZEnum, ZEnumItem};
use crate::internal::ast::zstruct::ZStruct;
use crate::internal::ast::{
    type_reference::InstantiateType, zchoice::ZChoice, zchoice::ZChoiceCase, zfunction::ZFunction,
    zsubtype::Subtype, zunion::ZUnion,
};
use crate::internal::parser::gen::zserioparservisitor::ZserioParserVisitorCompat;
use std::collections::HashMap;

use crate::internal::ast::{
    expression::{EvaluationState, Expression, ExpressionFlag, ExpressionType},
    field::Array,
    field::Field,
    parameter::Parameter,
    zbitmask::ZBitmaskType,
    zbitmask::ZBitmaskValue,
};
use crate::internal::parser::gen::zserioparser::{
    AdditiveExpressionContext, AdditiveExpressionContextAttrs, ArrayExpressionContext,
    ArrayExpressionContextAttrs, BitmaskDeclarationContext, BitmaskDeclarationContextAttrs,
    BitmaskValueContext, BitmaskValueContextAttrs, BitwiseAndExpressionContext,
    BitwiseAndExpressionContextAttrs, BitwiseOrExpressionContext, BitwiseOrExpressionContextAttrs,
    BitwiseXorExpressionContext, BitwiseXorExpressionContextAttrs, ChoiceCaseContext,
    ChoiceCaseContextAttrs, ChoiceCasesContext, ChoiceCasesContextAttrs, ChoiceDeclarationContext,
    ChoiceDeclarationContextAttrs, ChoiceDefaultContext, ChoiceDefaultContextAttrs,
    ChoiceFieldDefinitionContext, ChoiceFieldDefinitionContextAttrs, ConstDefinitionContext,
    ConstDefinitionContextAttrs, DotExpressionContext, DotExpressionContextAttrs,
    DynamicLengthArgumentContext, DynamicLengthArgumentContextAttrs, EnumDeclarationContext,
    EnumDeclarationContextAttrs, EnumItemContext, EnumItemContextAttrs, EqualityExpressionContext,
    EqualityExpressionContextAttrs, FieldAlignmentContext, FieldAlignmentContextAttrs,
    FieldArrayRangeContextAttrs, FieldConstraintContext, FieldConstraintContextAttrs,
    FieldInitializerContext, FieldInitializerContextAttrs, FieldOptionalClauseContext,
    FieldOptionalClauseContextAttrs, FieldTypeIdContext, FieldTypeIdContextAttrs,
    FunctionBodyContextAttrs, FunctionCallExpressionContext, FunctionCallExpressionContextAttrs,
    FunctionDefinitionContext, FunctionDefinitionContextAttrs, FunctionTypeContextAttrs, IdContext,
    IdentifierExpressionContext, IdentifierExpressionContextAttrs, ImportDeclarationContext,
    ImportDeclarationContextAttrs, IndexExpressionContext, IndexExpressionContextAttrs,
    InstantiateDeclarationContext, InstantiateDeclarationContextAttrs, LengthofExpressionContext,
    LengthofExpressionContextAttrs, LiteralContextAttrs, LiteralExpressionContext,
    LiteralExpressionContextAttrs, LogicalAndExpressionContext, LogicalAndExpressionContextAttrs,
    LogicalOrExpressionContext, LogicalOrExpressionContextAttrs, MultiplicativeExpressionContext,
    MultiplicativeExpressionContextAttrs, NumbitsExpressionContext, NumbitsExpressionContextAttrs,
    PackageDeclarationContext, PackageDeclarationContextAttrs, PackageNameDefinitionContext,
    PackageNameDefinitionContextAttrs, ParameterDefinitionContext, ParameterDefinitionContextAttrs,
    ParenthesizedExpressionContext, ParenthesizedExpressionContextAttrs, QualifiedNameContext,
    RelationalExpressionContext, RelationalExpressionContextAttrs, ShiftExpressionContext,
    ShiftExpressionContextAttrs, StructureDeclarationContext, StructureDeclarationContextAttrs,
    StructureFieldDefinitionContext, StructureFieldDefinitionContextAttrs,
    SubtypeDeclarationContext, SubtypeDeclarationContextAttrs, TemplateArgumentContext,
    TemplateArgumentContextAttrs, TemplateArgumentsContext, TemplateArgumentsContextAttrs,
    TemplateParametersContext, TemplateParametersContextAttrs, TernaryExpressionContext,
    TernaryExpressionContextAttrs, TypeArgumentContext, TypeArgumentContextAttrs,
    TypeArgumentsContext, TypeArgumentsContextAttrs, TypeInstantiationContext,
    TypeInstantiationContextAttrs, TypeParametersContext, TypeParametersContextAttrs,
    TypeReferenceContext, TypeReferenceContextAttrs, UnaryExpressionContext,
    UnaryExpressionContextAttrs, UnionDeclarationContext, UnionDeclarationContextAttrs,
    ValueofExpressionContext, ValueofExpressionContextAttrs, ZserioParserContextType,
};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};

use std::cell::RefCell;
use std::rc::Rc;

use super::parser::gen::zserioparser::{
    IdContextAttrs, UnionFieldDefinitionContextAttrs, DECIMAL_LITERAL, GT, RSHIFT,
};

// the antlr implementation for Rust requires to use one single return type,
// but depending on the node, the types returned while parsing the tree may
// vary. As such, this wrapper enum was introduced, which contains all possible
// return types.
pub enum ZserioTreeReturnType {
    Str(String),
    UInt8(u8),
    StrVec(Vec<String>),
    Package(Box<ZPackage>),
    Const(Box<ZConst>),
    Subtype(Box<Subtype>),
    InstantiateType(Box<InstantiateType>),
    Structure(Box<ZStruct>),
    Enumeration(Box<ZEnum>),
    Union(Box<ZUnion>),
    EnumItem(Box<ZEnumItem>),
    Expression(Box<Expression>),
    Expressions(Vec<Expression>),
    Field(Box<Field>),
    TypeReference(Box<TypeReference>),
    TypeReferences(Vec<TypeReference>),
    Vec(Vec<ZserioTreeReturnType>),
    Import(Box<ZImport>),
    BitmaskType(Box<ZBitmaskType>),
    BitmaskValue(ZBitmaskValue),
    ChoiceCase(ZChoiceCase),
    Choice(Box<ZChoice>),
    Parameter(Parameter),
    Parameters(Vec<Rc<RefCell<Parameter>>>),
    Function(ZFunction),
}

// We need to provide a default value for the enum return
impl Default for ZserioTreeReturnType {
    fn default() -> Self {
        ZserioTreeReturnType::Str("".into())
    }
}
pub struct Visitor(pub ZserioTreeReturnType);
impl ParseTreeVisitorCompat<'_> for Visitor {
    type Node = ZserioParserContextType;
    type Return = ZserioTreeReturnType;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        ZserioTreeReturnType::Str(_node.symbol.get_text().into())
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        let mut final_return_types: Vec<ZserioTreeReturnType> = Vec::new();

        match aggregate {
            ZserioTreeReturnType::Vec(mut v) => {
                final_return_types.append(&mut v);
            }
            _ => final_return_types.push(aggregate),
        }
        match next {
            ZserioTreeReturnType::Vec(mut v) => {
                final_return_types.append(&mut v);
            }
            _ => final_return_types.push(next),
        }
        ZserioTreeReturnType::Vec(final_return_types)
    }
}

impl ZserioParserVisitorCompat<'_> for Visitor {
    fn visit_packageDeclaration(&mut self, ctx: &PackageDeclarationContext<'_>) -> Self::Return {
        let mut package_name = "".into();
        match self.visit(&*ctx.packageNameDefinition().unwrap()) {
            ZserioTreeReturnType::Str(n) => package_name = n,
            _ => println!("package declaration missing"),
        }

        println!("package found with name {package_name}");

        let mut imports = Vec::new();
        for import in ctx.importDeclaration_all() {
            let import_node: Box<ZImport> = match self.visit(&*import) {
                ZserioTreeReturnType::Import(n) => n,
                _ => panic!("should not happen"),
            };
            imports.push(*import_node);
        }

        let mut package = Box::new(ZPackage {
            name: package_name,
            comment: "".into(),
            imports,
            structs: HashMap::new(),
            zchoices: HashMap::new(),
            zunions: vec![],
            enums: vec![],
            consts: vec![],
            subtypes: vec![],
            instantiated_types: vec![],
            bitmask_types: vec![],
        });

        for directive in ctx.languageDirective_all() {
            match self.visit(&*directive) {
                ZserioTreeReturnType::Str(s) => println!("unknown: {0}", s),
                ZserioTreeReturnType::Vec(v) => {
                    for ve in v {
                        match ve {
                            ZserioTreeReturnType::Structure(s) => {
                                let name = s.name.clone();
                                package.structs.insert(name, Rc::new(RefCell::new(*s)));
                            }
                            ZserioTreeReturnType::Choice(c) => {
                                let name = c.name.clone();
                                package.zchoices.insert(name, Rc::new(RefCell::new(*c)));
                            }
                            ZserioTreeReturnType::Enumeration(e) => {
                                package.enums.push(Rc::new(RefCell::new(*e)))
                            }
                            ZserioTreeReturnType::Const(c) => {
                                package.consts.push(Rc::new(RefCell::new(*c)))
                            }
                            ZserioTreeReturnType::Union(u) => {
                                package.zunions.push(Rc::new(RefCell::new(*u)))
                            }
                            ZserioTreeReturnType::Subtype(s) => {
                                package.subtypes.push(Rc::new(RefCell::new(*s)))
                            }
                            ZserioTreeReturnType::BitmaskType(bitmask_type) => package
                                .bitmask_types
                                .push(Rc::new(RefCell::new(*bitmask_type))),
                            ZserioTreeReturnType::Str(_) => {}
                            ZserioTreeReturnType::StrVec(s) => {
                                println!("unknown str vec: {0}", s[0])
                            }
                            ZserioTreeReturnType::TypeReference(z) => {
                                println!("unknown type ref: {0}", z.bits)
                            }
                            ZserioTreeReturnType::Field(_z) => print!("field found"),
                            ZserioTreeReturnType::Expression(_e) => print!("expression found"),
                            ZserioTreeReturnType::InstantiateType(t) => {
                                package.instantiated_types.push(Rc::new(RefCell::new(*t)))
                            }
                            _ => panic!("should not happen2"),
                        }
                    }
                }
                _ => panic!("should not happen"),
            };
        }

        //self.visit_children(ctx)
        ZserioTreeReturnType::Package(package)
    }

    fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'_>) -> Self::Return {
        let mut import_paths = Vec::new();

        for id in ctx.id_all() {
            import_paths.push(id.get_text());
        }

        let symbol_name = match ctx.MULTIPLY() {
            None => import_paths.pop(),
            _is_set => None,
        };
        ZserioTreeReturnType::Import(Box::new(ZImport {
            package_dir: import_paths,
            symbol_name,
        }))
    }

    fn visit_packageNameDefinition(
        &mut self,
        ctx: &PackageNameDefinitionContext<'_>,
    ) -> Self::Return {
        let mut ids = Vec::new();

        for id_ctx in ctx.id_all() {
            match ZserioParserVisitorCompat::visit_id(self, &id_ctx) {
                ZserioTreeReturnType::Str(id) => ids.push(id),
                _ => panic!(),
            }
        }

        // join the components to a . separated string, e.g.
        // package.subpackage.currentpackage
        ZserioTreeReturnType::Str(ids.join("."))
    }

    fn visit_constDefinition(&mut self, ctx: &ConstDefinitionContext<'_>) -> Self::Return {
        let z_const = Box::new(ZConst {
            name: ctx.id().unwrap().get_text(),
            comment: "".into(),
            zserio_type: match ZserioParserVisitorCompat::visit_typeInstantiation(
                self,
                &ctx.typeInstantiation().unwrap(),
            ) {
                ZserioTreeReturnType::TypeReference(t) => t,
                _ => panic!("should not happen"),
            },
            value_expression: match self.visit(&*ctx.expression().unwrap()) {
                ZserioTreeReturnType::Expression(e) => Rc::from(RefCell::from(*e)),
                _ => panic!("should not happen"),
            },
        });
        ZserioTreeReturnType::Const(z_const)
    }

    fn visit_subtypeDeclaration(&mut self, ctx: &SubtypeDeclarationContext<'_>) -> Self::Return {
        let zserio_type = match ZserioParserVisitorCompat::visit_typeReference(
            self,
            &ctx.typeReference().unwrap(),
        ) {
            ZserioTreeReturnType::TypeReference(t) => t,
            _ => panic!("should not happen"),
        };
        ZserioTreeReturnType::Subtype(Box::new(Subtype {
            name: ctx.id().unwrap().get_text(),
            zserio_type,
        }))
    }

    fn visit_instantiateDeclaration(
        &mut self,
        ctx: &InstantiateDeclarationContext<'_>,
    ) -> Self::Return {
        let zserio_type = match ZserioParserVisitorCompat::visit_typeReference(
            self,
            &ctx.typeReference().unwrap(),
        ) {
            ZserioTreeReturnType::TypeReference(t) => t,
            _ => panic!("should not happen"),
        };
        ZserioTreeReturnType::InstantiateType(Box::new(InstantiateType {
            name: ctx.id().unwrap().get_text(),
            zserio_type,
        }))
    }

    fn visit_bitmaskDeclaration(&mut self, ctx: &BitmaskDeclarationContext<'_>) -> Self::Return {
        let zserio_type = match ZserioParserVisitorCompat::visit_typeInstantiation(
            self,
            &ctx.typeInstantiation().unwrap(),
        ) {
            ZserioTreeReturnType::TypeReference(t) => t,
            _ => panic!("should not happen"),
        };
        let mut values = vec![];
        for bitmask_context in ctx.bitmaskValue_all() {
            match self.visit(&*bitmask_context) {
                ZserioTreeReturnType::BitmaskValue(v) => values.push(v),
                _ => panic!("should not happen"),
            }
        }
        ZserioTreeReturnType::BitmaskType(Box::new(ZBitmaskType {
            name: ctx.id().unwrap().get_text(),
            zserio_type,
            values,
        }))
    }

    fn visit_bitmaskValue(&mut self, ctx: &BitmaskValueContext<'_>) -> Self::Return {
        let mut z_bitmask_value = ZBitmaskValue {
            name: ctx.id().unwrap().get_text(),
            value: 0,
            value_expression: None,
        };
        if let Some(expression) = ctx.expression() {
            match self.visit(&*expression) {
                ZserioTreeReturnType::Expression(expr) => {
                    z_bitmask_value.value_expression = Option::from(Rc::from(RefCell::from(*expr)))
                }
                _ => panic!("wrong type returned from expression"),
            }
        }
        ZserioTreeReturnType::BitmaskValue(z_bitmask_value)
    }

    fn visit_structureDeclaration(
        &mut self,
        ctx: &StructureDeclarationContext<'_>,
    ) -> Self::Return {
        let mut zserio_struct = Box::new(ZStruct {
            name: ctx.id().unwrap().get_text(),
            comment: "todo".into(),
            template_parameters: vec![],
            type_parameters: vec![],
            fields: vec![],
            functions: vec![],
        });

        if let Some(template_params_ctx) = ctx.templateParameters() {
            match ZserioParserVisitorCompat::visit_templateParameters(self, &template_params_ctx) {
                ZserioTreeReturnType::StrVec(n) => zserio_struct.template_parameters = n,
                _ => panic!(),
            }
        }
        if let Some(type_params_ctx) = ctx.typeParameters() {
            match ZserioParserVisitorCompat::visit_typeParameters(self, &type_params_ctx) {
                ZserioTreeReturnType::Parameters(params) => zserio_struct.type_parameters = params,
                _ => panic!(),
            }
        }
        for field_context in ctx.structureFieldDefinition_all() {
            match self.visit(&*field_context) {
                ZserioTreeReturnType::Field(f) => {
                    zserio_struct.fields.push(Rc::from(RefCell::from(*f)))
                }
                _ => panic!(),
            }
        }
        for function_ctx in ctx.functionDefinition_all() {
            match ZserioParserVisitorCompat::visit_functionDefinition(self, &function_ctx) {
                ZserioTreeReturnType::Function(f) => {
                    zserio_struct.functions.push(Rc::from(RefCell::from(f)))
                }
                _ => panic!(),
            }
        }
        ZserioTreeReturnType::Structure(zserio_struct)
    }

    fn visit_structureFieldDefinition(
        &mut self,
        ctx: &StructureFieldDefinitionContext<'_>,
    ) -> Self::Return {
        // Clemens TODO
        let mut field: Box<Field>;
        match ZserioParserVisitorCompat::visit_fieldTypeId(self, &ctx.fieldTypeId().unwrap()) {
            ZserioTreeReturnType::Field(f) => field = f,
            _ => panic!("should not happen"),
        }

        field.is_optional = ctx.OPTIONAL().is_some();

        if let Some(field_alignment_ctx) = ctx.fieldAlignment() {
            match ZserioParserVisitorCompat::visit_fieldAlignment(self, &field_alignment_ctx) {
                ZserioTreeReturnType::UInt8(alignment) => {
                    field.alignment = alignment;
                }
                _ => panic!("unexpected field alignment type"),
            }
        }

        if let Some(field_initializer_ctx) = ctx.fieldInitializer() {
            match ZserioParserVisitorCompat::visit_fieldInitializer(self, &field_initializer_ctx) {
                ZserioTreeReturnType::Expression(expr) => {
                    field.initializer = Option::from(Rc::from(RefCell::from(*expr)))
                }
                _ => panic!("unexpected field initializer type"),
            }
        }

        if let Some(field_optional_clause_ctx) = ctx.fieldOptionalClause() {
            match ZserioParserVisitorCompat::visit_fieldOptionalClause(
                self,
                &field_optional_clause_ctx,
            ) {
                ZserioTreeReturnType::Expression(expr) => {
                    field.optional_clause = Option::from(Rc::from(RefCell::from(*expr)))
                }
                _ => panic!("unexpected field optional clause type"),
            }
        }

        if let Some(field_constraint_ctx) = ctx.fieldConstraint() {
            match ZserioParserVisitorCompat::visit_fieldConstraint(self, &field_constraint_ctx) {
                ZserioTreeReturnType::Expression(expr) => {
                    field.constraint = Option::from(Rc::from(RefCell::from(*expr)))
                }
                _ => panic!("unexpected field constraint type"),
            }
        }
        ZserioTreeReturnType::Field(field)
    }

    fn visit_choiceCases(&mut self, ctx: &ChoiceCasesContext<'_>) -> Self::Return {
        let mut choice_cases = ZChoiceCase {
            conditions: vec![],
            field: None,
        };
        for choice_case in ctx.choiceCase_all() {
            match self.visit_choiceCase(&choice_case) {
                ZserioTreeReturnType::Expression(expr) => {
                    choice_cases.conditions.push(Rc::from(RefCell::from(*expr)))
                }
                _ => panic!("wrong type returned from expression"),
            }
        }
        if let Some(choice_field) = ctx.choiceFieldDefinition() {
            match self.visit(&*choice_field) {
                ZserioTreeReturnType::Field(field) => {
                    choice_cases.field = Option::from(Rc::from(RefCell::from(*field)))
                }
                _ => panic!("wrong type returned from expression"),
            }
        }
        ZserioTreeReturnType::ChoiceCase(choice_cases)
    }

    fn visit_choiceCase(&mut self, ctx: &ChoiceCaseContext<'_>) -> Self::Return {
        match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(expr) => ZserioTreeReturnType::Expression(expr),
            _ => panic!("wrong type returned from expression"),
        }
    }

    fn visit_choiceDefault(&mut self, ctx: &ChoiceDefaultContext<'_>) -> Self::Return {
        let mut choice_case = ZChoiceCase {
            conditions: vec![],
            field: None,
        };
        if let Some(choice_field) = ctx.choiceFieldDefinition() {
            match self.visit(&*choice_field) {
                ZserioTreeReturnType::Field(field) => {
                    choice_case.field = Option::from(Rc::from(RefCell::from(*field)))
                }
                _ => panic!("wrong type returned from expression"),
            }
        }
        ZserioTreeReturnType::ChoiceCase(choice_case)
    }

    fn visit_choiceFieldDefinition(
        &mut self,
        ctx: &ChoiceFieldDefinitionContext<'_>,
    ) -> Self::Return {
        self.visit_fieldTypeId(&ctx.fieldTypeId().unwrap())
    }
    fn visit_choiceDeclaration(&mut self, ctx: &ChoiceDeclarationContext<'_>) -> Self::Return {
        let mut choice = Box::new(ZChoice {
            name: ctx.id().unwrap().get_text(),
            template_parameters: vec![],
            type_parameters: vec![],
            selector_expression: match self.visit(&*ctx.expression().unwrap()) {
                ZserioTreeReturnType::Expression(e) => Rc::from(RefCell::from(*e)),
                _ => panic!("failed to valuate choice selector expression"),
            },
            cases: vec![],
            default_case: None,
            functions: vec![],
        });
        if let Some(x) = ctx.templateParameters() {
            match ZserioParserVisitorCompat::visit_templateParameters(self, &x) {
                ZserioTreeReturnType::StrVec(n) => choice.template_parameters = n,
                _ => println!("template parameters should be a list of strings"),
            }
        }

        if let Some(ctx) = ctx.typeParameters() {
            match ZserioParserVisitorCompat::visit_typeParameters(self, &ctx) {
                ZserioTreeReturnType::Parameters(ps) => choice.type_parameters = ps,
                _ => panic!("type parameters return type mismatch"),
            }
        }

        // visit all the cases of the choice
        for choice_case_ctx in ctx.choiceCases_all() {
            match self.visit_choiceCases(&choice_case_ctx) {
                ZserioTreeReturnType::ChoiceCase(choice_case) => choice.cases.push(choice_case),
                _ => panic!("choice case type mismatch"),
            };
        }

        // TODO visit the default type

        // Visit all functions defined for this choice.
        for function_ctx in ctx.functionDefinition_all() {
            match ZserioParserVisitorCompat::visit_functionDefinition(self, &function_ctx) {
                ZserioTreeReturnType::Function(f) => {
                    choice.functions.push(Rc::from(RefCell::from(f)))
                }
                _ => panic!(),
            }
        }

        ZserioTreeReturnType::Choice(choice)
    }

    fn visit_fieldAlignment(&mut self, ctx: &FieldAlignmentContext<'_>) -> Self::Return {
        let field_alignment_expression = match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(expr) => *expr,
            _ => panic!("failed to evaluate field alignment expression"),
        };

        if field_alignment_expression.expression_type != DECIMAL_LITERAL {
            panic!("field alignment expression must be a decimal literal");
        }
        ZserioTreeReturnType::UInt8(
            field_alignment_expression
                .text
                .parse::<u8>()
                .expect("failed to parse field alignment decimal expression"),
        )
    }

    fn visit_fieldTypeId(&mut self, ctx: &FieldTypeIdContext<'_>) -> Self::Return {
        // retrieve the name of the field (e.g. member name)
        let field_name = match self.visit(&*ctx.id().unwrap()) {
            ZserioTreeReturnType::Str(n) => n,
            _ => panic!("should not happen"),
        };

        // Retrieve the field data type.
        let type_reference: Box<TypeReference> =
            match ZserioParserVisitorCompat::visit_typeInstantiation(
                self,
                &ctx.typeInstantiation().unwrap(),
            ) {
                ZserioTreeReturnType::TypeReference(t) => t,
                _ => panic!("should not happen"),
            };

        // check if the field is an array
        let mut array: Option<Array> = None;
        if let Some(rc_arr_ctx) = ctx.fieldArrayRange() {
            let mut array_length_exp = None;
            if let Some(array_length_expression_ctx) = rc_arr_ctx.expression() {
                match self.visit(&*array_length_expression_ctx) {
                    ZserioTreeReturnType::Expression(expr) => {
                        array_length_exp = Option::from(Rc::from(RefCell::from(*expr)))
                    }
                    _ => panic!("wrong type returned from expression"),
                }
            }

            array = Option::from(Array {
                is_implicit: ctx.IMPLICIT().is_some(),
                is_packed: ctx.PACKED().is_some(),
                array_length_expression: array_length_exp,
            });
        }
        ZserioTreeReturnType::Field(Box::new(Field {
            name: field_name.clone(),
            comment: "".into(),
            is_optional: false,
            alignment: 0,
            field_type: type_reference,
            constraint: None,
            initializer: None,
            optional_clause: None,
            array,
        }))
    }

    fn visit_unionDeclaration(&mut self, ctx: &UnionDeclarationContext<'_>) -> Self::Return {
        let mut zserio_union = Box::new(ZUnion {
            name: ctx.id().unwrap().get_text(),
            comment: "todo".into(),
            template_parameters: vec![],
            type_parameters: vec![],
            fields: vec![],
            functions: vec![],
        });

        if let Some(template_params_ctx) = ctx.templateParameters() {
            match ZserioParserVisitorCompat::visit_templateParameters(self, &template_params_ctx) {
                ZserioTreeReturnType::StrVec(n) => zserio_union.template_parameters = n,
                _ => panic!(),
            }
        }
        if let Some(type_params_ctx) = ctx.typeParameters() {
            match ZserioParserVisitorCompat::visit_typeParameters(self, &type_params_ctx) {
                ZserioTreeReturnType::Parameters(params) => zserio_union.type_parameters = params,
                _ => panic!(),
            }
        }
        for field_context in ctx.unionFieldDefinition_all() {
            match self.visit(&*field_context.choiceFieldDefinition().unwrap()) {
                ZserioTreeReturnType::Field(field) => {
                    zserio_union.fields.push(Rc::from(RefCell::from(*field)))
                }
                _ => panic!("failed to decode the union fields."),
            }
        }
        for function_ctx in ctx.functionDefinition_all() {
            match ZserioParserVisitorCompat::visit_functionDefinition(self, &function_ctx) {
                ZserioTreeReturnType::Function(f) => {
                    zserio_union.functions.push(Rc::from(RefCell::from(f)))
                }
                _ => panic!(),
            }
        }
        ZserioTreeReturnType::Union(zserio_union)
    }

    fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'_>) -> Self::Return {
        // Retrieve the name of the Enum
        let name: String = match ZserioParserVisitorCompat::visit_id(self, &ctx.id().unwrap()) {
            ZserioTreeReturnType::Str(n) => n,
            _ => panic!(),
        };

        // Retrieve the type
        let enum_type: Box<TypeReference> = match ZserioParserVisitorCompat::visit_typeInstantiation(
            self,
            &ctx.typeInstantiation().unwrap(),
        ) {
            ZserioTreeReturnType::TypeReference(t) => t,
            _ => panic!(),
        };

        // parse all enum items
        let mut items = Vec::new();
        for enum_item_ctx in ctx.enumItem_all() {
            match ZserioParserVisitorCompat::visit_enumItem(self, &enum_item_ctx) {
                ZserioTreeReturnType::EnumItem(item) => items.push(*item),
                _ => panic!(),
            }
        }

        ZserioTreeReturnType::Enumeration(Box::new(ZEnum {
            name,
            comment: "".into(),
            enum_type,
            items,
        }))
    }

    fn visit_enumItem(&mut self, ctx: &EnumItemContext<'_>) -> Self::Return {
        let mut enum_item = Box::new(ZEnumItem {
            name: ctx.id().unwrap().get_text(),
            comment: "".into(),
            value: None,
        });
        if let Some(expression_ctx) = ctx.expression() {
            match self.visit(&*expression_ctx) {
                ZserioTreeReturnType::Expression(e) => {
                    enum_item.value = Option::from(Rc::from(RefCell::from(*e)))
                }
                _ => panic!(),
            }
        }
        ZserioTreeReturnType::EnumItem(enum_item)
    }

    fn visit_templateParameters(&mut self, ctx: &TemplateParametersContext<'_>) -> Self::Return {
        let tree_ids = ctx.id_all();
        let mut ids = Vec::new();

        for id in tree_ids {
            ids.push(id.get_text())
        }

        ZserioTreeReturnType::StrVec(ids)
    }

    fn visit_templateArguments(&mut self, ctx: &TemplateArgumentsContext<'_>) -> Self::Return {
        // retrieves the template arguments to a type. A template argument is basically just a collection
        // of types.
        let mut template_aguments = Vec::new();
        for template_argument in ctx.templateArgument_all() {
            match ZserioParserVisitorCompat::visit_templateArgument(self, &template_argument) {
                ZserioTreeReturnType::TypeReference(t) => template_aguments.push(*t),
                _ => panic!("unexpected return type"),
            }
        }
        ZserioTreeReturnType::TypeReferences(template_aguments)
    }

    fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'_>) -> Self::Return {
        // a template argument is basically just a type reference
        ZserioParserVisitorCompat::visit_typeReference(self, &ctx.typeReference().unwrap())
    }

    fn visit_parameterDefinition(&mut self, ctx: &ParameterDefinitionContext<'_>) -> Self::Return {
        let zserio_type = match self.visit(&*ctx.typeReference().unwrap()) {
            ZserioTreeReturnType::TypeReference(t) => t,
            _ => panic!(),
        };
        ZserioTreeReturnType::Parameter(Parameter {
            name: ctx.id().unwrap().get_text(),
            zserio_type,
        })
    }

    fn visit_id(&mut self, ctx: &IdContext<'_>) -> Self::Return {
        self.visit(&*ctx.get_child(0).unwrap())
    }

    fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'_>) -> Self::Return {
        ZserioTreeReturnType::Str(ctx.get_text())
    }

    fn visit_fieldInitializer(&mut self, ctx: &FieldInitializerContext<'_>) -> Self::Return {
        self.visit(&*ctx.expression().unwrap())
    }

    fn visit_fieldOptionalClause(&mut self, ctx: &FieldOptionalClauseContext<'_>) -> Self::Return {
        self.visit(&*ctx.expression().unwrap())
    }

    fn visit_fieldConstraint(&mut self, ctx: &FieldConstraintContext<'_>) -> Self::Return {
        self.visit(&*ctx.expression().unwrap())
    }

    fn visit_parenthesizedExpression(
        &mut self,
        ctx: &ParenthesizedExpressionContext<'_>,
    ) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_functionCallExpression(
        &mut self,
        ctx: &FunctionCallExpressionContext<'_>,
    ) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_arrayExpression(&mut self, ctx: &ArrayExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'_>) -> Self::Return {
        let expression = Box::new(Expression {
            expression_type: ctx.INDEX().unwrap().symbol.token_type,
            text: ctx.INDEX().unwrap().get_text(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_dotExpression(&mut self, ctx: &DotExpressionContext<'_>) -> Self::Return {
        let _expression_ctx = ctx.expression();

        let op1 = match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(exp) => exp,
            _ => panic!("unexpected first dot operand"),
        };
        ZserioTreeReturnType::Expression(Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.DOT().unwrap().get_text(),
            flag: ExpressionFlag::None,
            operand1: Option::from(op1),
            operand2: Option::from(Box::new(Expression {
                expression_type: 0xFFFFF, // TODO make this a constant
                text: ctx.id().unwrap().get_text(),
                flag: ExpressionFlag::IsDotExpressionRightOperand,
                operand1: None,
                operand2: None,
                operand3: None,
                result_type: ExpressionType::Other,
                symbol: None,
                fully_resolved: false,
                evaluation_state: EvaluationState::NotEvaluated,
                native_type: None,
            })),
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        }))
    }

    fn visit_identifierExpression(
        &mut self,
        ctx: &IdentifierExpressionContext<'_>,
    ) -> Self::Return {
        let id_context = ctx.id().unwrap();
        ZserioTreeReturnType::Expression(Box::new(Expression {
            expression_type: id_context.ID().unwrap().symbol.get_token_type(),
            text: id_context.get_text(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        }))
    }

    fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'_>) -> Self::Return {
        let literal_ctx = ctx.literal().unwrap();

        // identify which literal this is
        let literal_text = literal_ctx.get_text();
        let result_type = match literal_ctx {
            _ if literal_ctx.BOOL_LITERAL().is_some() => ExpressionType::Bool(
                literal_text
                    .parse::<bool>()
                    .expect("failed to parse bool expression"),
            ),
            _ if literal_ctx.DECIMAL_LITERAL().is_some() => ExpressionType::Integer(
                literal_text
                    .parse::<i32>()
                    .expect("failed to parse integer expression"),
            ),
            _ if literal_ctx.HEXADECIMAL_LITERAL().is_some() => ExpressionType::Integer(
                i32::from_str_radix(literal_text.trim_start_matches("0x"), 16)
                    .expect("Not a hex number!"),
            ),
            _ if literal_ctx.OCTAL_LITERAL().is_some() => ExpressionType::Integer(
                i32::from_str_radix(literal_text.as_str(), 8).expect("Not an octal number!"),
            ),
            _ if literal_ctx.BINARY_LITERAL().is_some() => ExpressionType::Integer(
                i32::from_str_radix(literal_text.trim_end_matches('b'), 2)
                    .expect("Not a binary number!"),
            ),
            _ if literal_ctx.STRING_LITERAL().is_some() => {
                assert!(literal_text.len() >= 2);
                ExpressionType::String(literal_text[1..literal_text.len() - 1].into())
            }
            _ if literal_ctx.FLOAT_LITERAL().is_some() => ExpressionType::Float(
                literal_text
                    .parse::<f64>()
                    .expect("failed to parse float32 expression"),
            ),
            _ if literal_ctx.DOUBLE_LITERAL().is_some() => ExpressionType::Float(
                literal_text
                    .parse::<f64>()
                    .expect("failed to parse float64 expression"),
            ),
            _ => panic!("unexpected expression type {:?}, ", literal_text),
        };

        // Determine the exact zserio type for this literal.
        let native_type = match literal_ctx {
            _ if literal_ctx.BOOL_LITERAL().is_some() => {
                Some(TypeReference::new_native_type("bool"))
            }
            _ if literal_ctx.STRING_LITERAL().is_some() => {
                Some(TypeReference::new_native_type("string"))
            }
            _ => None,
        };

        let expression_type = literal_ctx.start().token_type;
        ZserioTreeReturnType::Expression(Box::new(Expression {
            expression_type,
            symbol: None,
            text: literal_text,
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type,
            fully_resolved: true,
            evaluation_state: EvaluationState::Completed,
            native_type,
        }))
    }

    fn visit_dynamicLengthArgument(
        &mut self,
        ctx: &DynamicLengthArgumentContext<'_>,
    ) -> Self::Return {
        self.visit(&*ctx.expression().unwrap())
    }

    fn visit_isSetExpression(&mut self, ctx: &IsSetExpressionContext<'_>) -> Self::Return {
        // Parses an isset() expression of a bitmask.
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Bool(true),
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => {
                // A special rule seems to apply for isset(), that for the second
                // operand, explicit typing the bitmask type is not needed,
                // as it can be deduced from the first operand.
                // for example:
                // isset(value, Bitmask.HAS_A)
                // and
                // isset(value, HAS_A)
                // are both legal zserio code, but:
                // (value & Bitmask.HAS_A) == Bitmask.HAS_A
                // is legal, while the variant below is not:
                // (value & HAS_A) == Bitmask.HAS_A
                // One could argue that this is inconsistent.
                // Raised it here: https://github.com/ndsev/zserio/discussions/617
                expression.operand2 = Option::from(e);
            }
            _ => panic!("the isset expression requires two expressions"),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_lengthofExpression(&mut self, ctx: &LengthofExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            // We assume that a lengthof expression always returns an varsize type.
            // This must match with the generated type during expression generation.
            native_type: Some(TypeReference::new_native_type("varsize")),
        });
        match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_valueofExpression(&mut self, ctx: &ValueofExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_numbitsExpression(&mut self, ctx: &NumbitsExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: Some(TypeReference::new_native_type("varsize")),
        });
        match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_multiplicativeExpression(
        &mut self,
        ctx: &MultiplicativeExpressionContext<'_>,
    ) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        // The RSHIFT token is not correctly identified by the ANTLR parser. It should be RSHIFT, but the parser
        // identifies it as GT.
        if expression.expression_type == GT {
            expression.expression_type = RSHIFT;
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_relationalExpression(
        &mut self,
        ctx: &RelationalExpressionContext<'_>,
    ) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_bitwiseAndExpression(
        &mut self,
        ctx: &BitwiseAndExpressionContext<'_>,
    ) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_bitwiseXorExpression(
        &mut self,
        ctx: &BitwiseXorExpressionContext<'_>,
    ) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_logicalAndExpression(
        &mut self,
        ctx: &LogicalAndExpressionContext<'_>,
    ) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: Some(TypeReference::new_native_type("bool")),
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: Some(TypeReference::new_native_type("bool")),
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_ternaryExpression(&mut self, ctx: &TernaryExpressionContext<'_>) -> Self::Return {
        let mut expression = Box::new(Expression {
            expression_type: ctx.operator.as_ref().unwrap().token_type,
            text: ctx.operator.as_ref().unwrap().get_text().into(),
            flag: ExpressionFlag::None,
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::Other,
            symbol: None,
            fully_resolved: false,
            evaluation_state: EvaluationState::NotEvaluated,
            native_type: None,
        });
        match self.visit(&*ctx.expression(0).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand1 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(1).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand2 = Option::from(e),
            _ => panic!(),
        }
        match self.visit(&*ctx.expression(2).unwrap()) {
            ZserioTreeReturnType::Expression(e) => expression.operand3 = Option::from(e),
            _ => panic!(),
        }
        ZserioTreeReturnType::Expression(expression)
    }

    fn visit_typeInstantiation(&mut self, ctx: &TypeInstantiationContext<'_>) -> Self::Return {
        let mut type_reference: Box<TypeReference> =
            match ZserioParserVisitorCompat::visit_typeReference(
                self,
                &ctx.typeReference().unwrap(),
            ) {
                ZserioTreeReturnType::TypeReference(t) => t,
                _ => panic!("should not happen"),
            };

        if let Some(type_arguments_ctx) = ctx.typeArguments() {
            match self.visit(&*type_arguments_ctx) {
                ZserioTreeReturnType::Expressions(e) => {
                    for expression in e {
                        type_reference
                            .type_arguments
                            .push(Rc::from(RefCell::from(expression)))
                    }
                }
                _ => panic!(),
            }
        }
        if let Some(dynamic_argument_ctx) = ctx.dynamicLengthArgument() {
            match self.visit(&*dynamic_argument_ctx) {
                ZserioTreeReturnType::Expression(e) => {
                    type_reference.length_expression = Option::from(Rc::from(RefCell::from(*e)))
                }
                _ => panic!(),
            }
        }
        ZserioTreeReturnType::TypeReference(type_reference)
    }

    fn visit_typeReference(&mut self, ctx: &TypeReferenceContext<'_>) -> Self::Return {
        let mut type_reference = Box::new(TypeReference {
            is_builtin: false,
            package: "".into(),
            name: "".into(),
            bits: 0,
            template_arguments: vec![],
            type_arguments: vec![],
            length_expression: None,
        });
        // check if the type is template, e.g. bit<expression>
        if let Some(template_arguments) = ctx.templateArguments() {
            match ZserioParserVisitorCompat::visit_templateArguments(self, &template_arguments) {
                ZserioTreeReturnType::TypeReferences(ta) => type_reference.template_arguments = ta,
                _ => panic!(),
            }
        }
        if ctx.builtinType().is_some() {
            // This is a built-in type, e.g. string, uint32, bit:x, ....
            type_reference.is_builtin = true;
            let mut name = ctx.get_text();
            let mut bits: u8 = 0;
            if name.contains(':') {
                let bits_subst: Vec<&str> = name.split(':').collect();
                bits = bits_subst[1]
                    .parse::<u8>()
                    .expect("failed to convert to u8");
                name = bits_subst[0].into();
            }
            type_reference.name = name;
            type_reference.bits = bits;

            return ZserioTreeReturnType::TypeReference(type_reference);
        }

        let mut name = match ZserioParserVisitorCompat::visit_qualifiedName(
            self,
            &ctx.qualifiedName().unwrap(),
        ) {
            ZserioTreeReturnType::Str(s) => s,
            _ => panic!("error"),
        };
        let mut package = "".into();
        if name.contains('.') {
            let (new_package, new_name) = name.rsplit_once('.').unwrap();
            package = new_package.into();
            name = new_name.into();
        }
        type_reference.name = name;
        type_reference.package = package;

        ZserioTreeReturnType::TypeReference(type_reference)
    }

    fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'_>) -> Self::Return {
        // Reads a zserio function from the parse tree.
        ZserioTreeReturnType::Function(ZFunction {
            name: ctx.functionName().unwrap().get_text(),
            result: match self.visit(&*ctx.functionBody().unwrap().expression().unwrap()) {
                ZserioTreeReturnType::Expression(e) => Rc::from(RefCell::from(*e)),
                _ => panic!(
                    "zserio function {} does not provide a valid content",
                    ctx.functionName().unwrap().get_text()
                ),
            },
            return_type: match self.visit(&*ctx.functionType().unwrap().typeReference().unwrap()) {
                ZserioTreeReturnType::TypeReference(t) => t,
                _ => panic!(),
            },
        })
    }

    fn visit_typeParameters(&mut self, ctx: &TypeParametersContext<'_>) -> Self::Return {
        let mut type_parameters = Vec::new();
        for type_param_ctx in ctx.parameterDefinition_all() {
            match self.visit(&*type_param_ctx) {
                ZserioTreeReturnType::Parameter(p) => {
                    type_parameters.push(Rc::from(RefCell::from(p)))
                }
                _ => panic!(),
            };
        }
        ZserioTreeReturnType::Parameters(type_parameters)
    }

    fn visit_typeArguments(&mut self, ctx: &TypeArgumentsContext<'_>) -> Self::Return {
        let mut type_arguments = Vec::new();
        for type_arg_ctx in ctx.typeArgument_all() {
            match ZserioParserVisitorCompat::visit_typeArgument(self, &type_arg_ctx) {
                ZserioTreeReturnType::Expression(t) => type_arguments.push(*t),
                _ => panic!(),
            }
        }
        ZserioTreeReturnType::Expressions(type_arguments)
    }

    fn visit_typeArgument(&mut self, ctx: &TypeArgumentContext<'_>) -> Self::Return {
        if ctx.EXPLICIT().is_some() {
            return ZserioTreeReturnType::Expression(Box::new(Expression {
                expression_type: ctx.id().unwrap().ID().unwrap().symbol.token_type,
                text: ctx.id().unwrap().get_text(),
                flag: ExpressionFlag::None,
                operand1: None,
                operand2: None,
                operand3: None,
                result_type: ExpressionType::Other,
                symbol: None,
                fully_resolved: false,
                evaluation_state: EvaluationState::NotEvaluated,
                native_type: None,
            }));
        }
        self.visit(&*ctx.expression().unwrap())
    }
}
