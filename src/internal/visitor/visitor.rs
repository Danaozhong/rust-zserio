

use crate::internal::parser::gen::zserioparservisitor::ZserioParserVisitorCompat;

use crate::internal::ast::package::{ZImport, ZPackage};
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::ast::zstruct::ZStruct;
use crate::internal::ast::{
    expression::{Expression, ExpressionType},
    field::Array,
    field::Field,
    zenum::{ZEnum, ZEnumItem},
};
use crate::internal::parser::gen::zserioparser::{
    DotExpressionContext, DotExpressionContextAttrs, DynamicLengthArgumentContext,
    DynamicLengthArgumentContextAttrs, EnumDeclarationContext, EnumDeclarationContextAttrs,
    EnumItemContext, EnumItemContextAttrs, FieldArrayRangeContextAttrs,
    FieldTypeIdContext, FieldTypeIdContextAttrs, IdContext, IdentifierExpressionContext,
    IdentifierExpressionContextAttrs, ImportDeclarationContext, ImportDeclarationContextAttrs,
    LiteralContextAttrs, LiteralExpressionContext, LiteralExpressionContextAttrs,
    PackageDeclarationContext, PackageDeclarationContextAttrs, PackageNameDefinitionContext,
    PackageNameDefinitionContextAttrs, QualifiedNameContext, StructureDeclarationContext,
    StructureDeclarationContextAttrs, StructureFieldDefinitionContext,
    StructureFieldDefinitionContextAttrs, TemplateArgumentContext, TemplateArgumentContextAttrs,
    TemplateArgumentsContext, TemplateArgumentsContextAttrs, TemplateParametersContext,
    TemplateParametersContextAttrs, TypeInstantiationContext, TypeInstantiationContextAttrs,
    TypeReferenceContext, TypeReferenceContextAttrs, ZserioParserContextType,
};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{
    ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree,
};
// the antlr implementation for Rust requires to use one single return type,
// but depending on the node, the types returned while parsing the tree may
// vary. As such, this wrapper enum was introduced, which contains all possible
// return types.
pub enum ZserioTreeReturnType {
    Str(String),
    StrVec(Vec<String>),
    Package(Box<ZPackage>),
    Structure(Box<ZStruct>),
    Enumeration(Box<ZEnum>),
    EnumItem(Box<ZEnumItem>),
    Expression(Box<Expression>),
    Field(Box<Field>),
    TypeReference(Box<TypeReference>),
    TypeReferences(Vec<Box<TypeReference>>),
    Vec(Vec<ZserioTreeReturnType>),
    Import(Box<ZImport>),
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
            _ => println!("should not happen"),
        }

        println!("package found with name {package_name}");

        let mut imports = Vec::new();
        for import in ctx.importDeclaration_all() {
            let import_node: Box<ZImport>;
            match self.visit(&*import) {
                ZserioTreeReturnType::Import(n) => import_node = n,
                _ => panic!("should not happen"),
            }
            imports.push(import_node);
        }

        // store globally (sorry, don't know how to do this better)
        /*
        CURRENT_PKG.with(|f|{
            let mut pkg_ref = f.borrow_mut();
            pkg_ref.name = package_name;
            pkg_ref.imports = imports;
        });
        */
        let mut structs = Vec::new();
        let mut enums = Vec::new();

        for directive in ctx.languageDirective_all() {
            match self.visit(&*directive) {
                ZserioTreeReturnType::Str(s) => println!("unknown: {0}", s),
                ZserioTreeReturnType::Vec(v) => {
                    for ve in v {
                        match ve {
                            ZserioTreeReturnType::Structure(s) => structs.push(s),
                            ZserioTreeReturnType::Enumeration(e) => enums.push(e),
                            ZserioTreeReturnType::Str(s) => println!("unknown str: {0}", s),
                            ZserioTreeReturnType::StrVec(s) => {
                                println!("unknown str vec: {0}", s[0])
                            }
                            ZserioTreeReturnType::TypeReference(z) => {
                                println!("unknown type ref: {0}", z.bits)
                            }
                            ZserioTreeReturnType::Field(_z) => print!("field found"),
                            ZserioTreeReturnType::Expression(_e) => print!("expression found"),
                            _ => panic!("should not happen2"),
                        }
                    }
                }
                _ => panic!("should not happen"),
            };
        }

        //self.visit_children(ctx)
        ZserioTreeReturnType::Package(Box::new(ZPackage {
            name: package_name,
            comment: "".into(),
            imports,
            structs,
            enums,
        }))
    }

    fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'_>) -> Self::Return {
        let mut import_paths = Vec::new();

        for id in ctx.id_all() {
            import_paths.push(id.get_text());
        }

        let mut symbol_name = String::from("");
        match ctx.MULTIPLY() {
            _is_set => {
                symbol_name = String::from("*");
            }
            None => {
                symbol_name = import_paths.pop().unwrap();
            }
        }

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
            match ZserioParserVisitorCompat::visit_id(self, &*id_ctx) {
                ZserioTreeReturnType::Str(id) => ids.push(id),
                _ => panic!(),
            }
        }

        // join the components to a . separated string, e.g.
        // package.subpackage.currentpackage
        ZserioTreeReturnType::Str(ids.join("."))
    }

    fn visit_structureDeclaration(
        &mut self,
        ctx: &StructureDeclarationContext<'_>,
    ) -> Self::Return {
        let mut struct_name = "".into();
        match self.visit(&*ctx.id().unwrap()) {
            ZserioTreeReturnType::Str(n) => struct_name = n,
            _ => println!("should not happen"),
        }

        let mut template_params = Vec::new();
        match ctx.templateParameters() {
            Some(x) => match ZserioParserVisitorCompat::visit_templateParameters(self, &*x) {
                ZserioTreeReturnType::StrVec(n) => template_params = n,
                _ => println!("should not happen"),
            },
            None => println!("struct has no template parameters"),
        }

        /*
        typ := &ast.Struct{
            Name:    v.Visit(ctx.Id().(antlr.ParseTree)).(string),
            Comment: getComment(ctx.GetParser(), ctx.GetStart()),
        }

        if tp := ctx.TemplateParameters(); tp != nil {
            typ.TemplateParameters = v.Visit(tp.(antlr.ParseTree)).([]string)
        }

        if tp := ctx.TypeParameters(); tp != nil {
            typ.TypeParameters = v.VisitTypeParameters(tp.(*parser.TypeParametersContext)).([]*ast.Parameter)
        }
        */

        let mut fields = Vec::new();
        for field_context in ctx.structureFieldDefinition_all() {
            match self.visit(&*field_context) {
                ZserioTreeReturnType::Field(f) => fields.push(f),
                _ => println!("should not happen"),
            }
        }
        /*
        for _, functionContext := range ctx.AllFunctionDefinition() {
            typ.Functions = append(typ.Functions, v.VisitFunctionDefinition(functionContext.(*parser.FunctionDefinitionContext)).(*ast.Function))
        }

        return typ
        */
        ZserioTreeReturnType::Structure(Box::new(ZStruct {
            name: struct_name,
            comment: "todo".into(),
            template_params,
            fields,
        }))
    }

    fn visit_structureFieldDefinition(
        &mut self,
        ctx: &StructureFieldDefinitionContext<'_>,
    ) -> Self::Return {
        // Clemens TODO
        let mut field: Box<Field>;
        match ZserioParserVisitorCompat::visit_fieldTypeId(self, &*ctx.fieldTypeId().unwrap()) {
            ZserioTreeReturnType::Field(f) => field = f,
            _ => panic!("should not happen"),
        }

        field.is_optional = !ctx.OPTIONAL().is_none();

        ZserioTreeReturnType::Field(field)
    }

    fn visit_fieldTypeId(&mut self, ctx: &FieldTypeIdContext<'_>) -> Self::Return {
        // retrieve the name of the field (e.g. member name)
        let mut field_name = "".into();
        match self.visit(&*ctx.id().unwrap()) {
            ZserioTreeReturnType::Str(n) => field_name = n,
            _ => panic!("should not happen"),
        }

        // the field data type
        let type_reference: Box<TypeReference>;
        match ZserioParserVisitorCompat::visit_typeInstantiation(
            self,
            &*ctx.typeInstantiation().unwrap(),
        ) {
            ZserioTreeReturnType::TypeReference(t) => type_reference = t,
            _ => panic!("should not happen"),
        }
        // TODO check if alignment is set
        
        // check if the field is an array
        let mut array: Option<Array> = None;
        if let Some(rc_arr_ctx) = ctx.fieldArrayRange() {
            let mut array_length_exp: Option<Box<Expression>> = None;
            if let Some(array_length_expression_ctx) = rc_arr_ctx.expression() {
                match self.visit(&*array_length_expression_ctx) {
                    ZserioTreeReturnType::Expression(expr) => array_length_exp = Option::from(expr),
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
            zserio_name: field_name,
            comment: "".into(),
            is_optional: false,
            alignment: 0,
            field_type: type_reference,
            array: array,
        }))
    }

    /*


    func (v *Visitor) VisitFieldTypeId(ctx *parser.FieldTypeIdContext) any {
        field := &ast.Field{
            Name: ctx.Id().GetText(),
            Type: v.Visit(ctx.TypeInstantiation().(antlr.ParseTree)).(*ast.TypeReference),
        }

        if ctx.FieldArrayRange() != nil {
            field.Array = &ast.Array{}
            expressionContext := ctx.FieldArrayRange().(*parser.FieldArrayRangeContext).Expression()
            if expressionContext != nil {
                field.Array.Length = v.Visit(expressionContext).(*ast.Expression)
            }
            if ctx.IMPLICIT() != nil {
                // http://zserio.org/doc/ZserioLanguageOverview.html#implicit-length-arrays
                return ErrImplicitLengthArraysNotSupported
            }
            if ctx.PACKED() != nil {
                field.Array.IsPacked = true
            }
        }
        return field
    }

    */
    /*

    func (v *Visitor) VisitStructureFieldDefinition(ctx *parser.StructureFieldDefinitionContext) any {
        field := v.Visit(ctx.FieldTypeId().(antlr.ParseTree)).(*ast.Field)
        field.Comment = getComment(ctx.GetParser(), ctx.GetStart())
        field.IsOptional = ctx.OPTIONAL() != nil
        if a := ctx.FieldAlignment(); a != nil {
            a, ok := v.Visit(a.(antlr.ParseTree)).(uint8)
            if !ok {
                return ErrAlignmentMustBeInteger
            }
            field.Alignment = a
        }

        if ctx.FieldInitializer() != nil {
            field.Initializer = v.VisitFieldInitializer(ctx.FieldInitializer().(*parser.FieldInitializerContext)).(*ast.Expression)
        }

        if ctx.FieldOptionalClause() != nil {
            field.OptionalClause = v.VisitFieldOptionalClause(ctx.FieldOptionalClause().(*parser.FieldOptionalClauseContext)).(*ast.Expression)
        }
        if ctx.FieldConstraint() != nil {
            field.Constraint = v.VisitFieldConstraint(ctx.FieldConstraint().(*parser.FieldConstraintContext)).(*ast.Expression)
        }
        return field
    }

    func (v *Visitor) VisitFieldAlignment(ctx *parser.FieldAlignmentContext) any {
        r := v.Visit(ctx.Expression().(antlr.ParseTree))
        expr, ok := r.(*ast.Expression)
        if !ok {
            return r
        }
        if expr.Type != parser.ZserioParserDECIMAL_LITERAL {
            return ErrAlignmentMustBeInteger
        }
        value, err := strconv.ParseUint(expr.Text, 10, 8)
        if err != nil {
            return ErrAlignmentMustBeInteger
        }
        if value > math.MaxUint8 {
            return ErrAlignmentMustBeInteger
        }
        return uint8(value)
    }

    func (v *Visitor) VisitFieldInitializer(ctx *parser.FieldInitializerContext) any {
        return v.Visit(ctx.Expression()).(*ast.Expression)
    }

    func (v *Visitor) VisitFieldOptionalClause(ctx *parser.FieldOptionalClauseContext) any {
        return v.Visit(ctx.Expression()).(*ast.Expression)
    }

    func (v *Visitor) VisitFieldConstraint(ctx *parser.FieldConstraintContext) any {
        return v.Visit(ctx.Expression()).(*ast.Expression)
    }
    */

    //fn visit_structureFieldDefinition(&mut self, ctx: &StructureFieldDefinitionContext<'_>) -> Self::Return {

    fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'_>) -> Self::Return {
        // Retrieve the name of the Enum
        let name: String;
        match ZserioParserVisitorCompat::visit_id(self, &*ctx.id().unwrap()) {
            ZserioTreeReturnType::Str(n) => name = n,
            _ => panic!(),
        };

        // Retrieve the type
        let enum_type: Box<TypeReference>;
        match ZserioParserVisitorCompat::visit_typeInstantiation(
            self,
            &*ctx.typeInstantiation().unwrap(),
        ) {
            ZserioTreeReturnType::TypeReference(t) => enum_type = t,
            _ => panic!(),
        };

        // parse all enum items
        let mut items = Vec::new();
        for enum_item_ctx in ctx.enumItem_all() {
            match ZserioParserVisitorCompat::visit_enumItem(self, &*enum_item_ctx) {
                ZserioTreeReturnType::EnumItem(item) => items.push(item),
                _ => panic!(),
            }
        }

        ZserioTreeReturnType::Enumeration(Box::new(ZEnum {
            name: name,
            comment: "".into(),
            enum_type,
            items,
        }))
    }

    fn visit_enumItem(&mut self, ctx: &EnumItemContext<'_>) -> Self::Return {
        let name: String;
        match ZserioParserVisitorCompat::visit_id(self, &*ctx.id().unwrap()) {
            ZserioTreeReturnType::Str(n) => name = n,
            _ => panic!(),
        }

        /*
            // an enum item can have an optional expression, specifying the value
            if ctx.Expression() != nil {
                typ.Expression = v.Visit(ctx.Expression()).(*ast.Expression)
            }

        */

        return ZserioTreeReturnType::EnumItem(Box::new(ZEnumItem {
            name: name,
            comment: "".into(),
        }));
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
            match ZserioParserVisitorCompat::visit_templateArgument(self, &*template_argument) {
                ZserioTreeReturnType::TypeReference(t) => template_aguments.push(t),
                _ => panic!("unexpected return type"),
            }
        }
        ZserioTreeReturnType::TypeReferences(template_aguments)
    }

    fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'_>) -> Self::Return {
        // a template argument is basically just a type reference
        ZserioParserVisitorCompat::visit_typeReference(self, &*ctx.typeReference().unwrap())
    }

    fn visit_id(&mut self, ctx: &IdContext<'_>) -> Self::Return {
        self.visit(&*ctx.get_child(0).unwrap())
    }

    fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'_>) -> Self::Return {
        ZserioTreeReturnType::Str(ctx.get_text())
    }
    /*


        func (v *Visitor) VisitId(ctx *parser.IdContext) any {
        idNode := ctx.GetChild(0)
        return v.Visit(idNode.(antlr.ParseTree))
    }

    func (v *Visitor) VisitQualifiedName(ctx *parser.QualifiedNameContext) any {
        return ctx.GetText()
    }
    */

    fn visit_dotExpression(&mut self, ctx: &DotExpressionContext<'_>) -> Self::Return {
        let _expression_ctx = ctx.expression();

        let op1;
        match self.visit(&*ctx.expression().unwrap()) {
            ZserioTreeReturnType::Expression(exp) => op1 = exp,
            _ => panic!("unexpected first dot operand"),
        }

        /*
                return &ast.Expression{
            Operand1: v.Visit(ctx.Expression()).(*ast.Expression),
            Operand2: &ast.Expression{
                Type: parser.UnevaluatableExpressionType,
                Text: ctx.Id().GetText(),
            },
            Type: ctx.GetOperator().GetTokenType(),
            Text: ctx.GetOperator().GetText(),
        }
         */
        ZserioTreeReturnType::Expression(Box::new(Expression {
            text: ctx.DOT().unwrap().get_text(),
            operand1: Option::from(op1),
            operand2: Option::from(Box::new(Expression {
                text: ctx.id().unwrap().get_text(),
                operand1: None,
                operand2: None,
                operand3: None,
                result_type: ExpressionType::OtherExpression,
                fully_resolved: true,
            })),
            operand3: None,
            result_type: ExpressionType::OtherExpression,
            fully_resolved: false,
        }))
    }

    fn visit_identifierExpression(
        &mut self,
        ctx: &IdentifierExpressionContext<'_>,
    ) -> Self::Return {
        let id_context = ctx.id().unwrap();
        return ZserioTreeReturnType::Expression(Box::new(Expression {
            text: id_context.get_text(),
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::OtherExpression,
            fully_resolved: false,
        }));
    }

    fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'_>) -> Self::Return {
        let literal_ctx = ctx.literal().unwrap();

        // identify which literal this is
        let literal_text = literal_ctx.get_text();
        let mut result_type = ExpressionType::OtherExpression;
        if literal_ctx.BOOL_LITERAL().is_some() {
            result_type = ExpressionType::BoolExpression(
                literal_text
                    .parse::<bool>()
                    .expect("failed to parse bool expression"),
            );
        } else if literal_ctx.DECIMAL_LITERAL().is_some() {
            result_type = ExpressionType::IntegerExpression(
                literal_text
                    .parse::<i32>()
                    .expect("failed to parse integer expression"),
            );
        } else if literal_ctx.HEXADECIMAL_LITERAL().is_some() {
            result_type = ExpressionType::IntegerExpression(
                i32::from_str_radix(literal_text.trim_start_matches("0x"), 16)
                    .expect("Not a hex number!"),
            );
        } else if literal_ctx.OCTAL_LITERAL().is_some() {
            result_type = ExpressionType::IntegerExpression(
                i32::from_str_radix(literal_text.as_str(), 8).expect("Not an octal number!"),
            );
        } else if literal_ctx.BINARY_LITERAL().is_some() {
            result_type = ExpressionType::IntegerExpression(
                i32::from_str_radix(literal_text.as_str(), 2).expect("Not a binary number!"),
            );
        } else {
            panic!("expression not found");
        }

        let _tokens = literal_ctx.get_tokens(1);
        ZserioTreeReturnType::Expression(Box::new(Expression {
            text: literal_ctx.get_text(), // ctx.literal().unwrap().get_text(),
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: result_type,
            fully_resolved: true,
        }))
    }

    fn visit_dynamicLengthArgument(
        &mut self,
        _ctx: &DynamicLengthArgumentContext<'_>,
    ) -> Self::Return {
        ZserioTreeReturnType::Expression(Box::new(Expression {
            text: "".into(),
            operand1: None,
            operand2: None,
            operand3: None,
            result_type: ExpressionType::BoolExpression(true),
            fully_resolved: false,
        }))
    }

    fn visit_typeInstantiation(&mut self, ctx: &TypeInstantiationContext<'_>) -> Self::Return {
        let type_reference: Box<TypeReference>;
        match ZserioParserVisitorCompat::visit_typeReference(self, &*ctx.typeReference().unwrap()) {
            ZserioTreeReturnType::TypeReference(t) => type_reference = t,
            _ => panic!("should not happen"),
        }

        // TODO type arguments and dynamic length argument
        /*
                if ctx.TypeArguments() != nil {
            typ.TypeArguments = v.VisitTypeArguments(ctx.TypeArguments().(*parser.TypeArgumentsContext)).([]*ast.Expression)
        }
        if ctx.DynamicLengthArgument() != nil {
            typ.LengthExpression = v.Visit(ctx.DynamicLengthArgument().(antlr.ParseTree)).(*ast.Expression)
        }
         */
        ZserioTreeReturnType::TypeReference(type_reference)
    }

    fn visit_typeReference(&mut self, ctx: &TypeReferenceContext<'_>) -> Self::Return {
        if ctx.builtinType().is_some() {
            // This is a built-in type, e.g. string, uint32, bit:x, ....
            let mut name = ctx.get_text();
            let mut bits: i8 = 0;

            // check if the type is template, e.g. bit<expression>
            if let Some(template_arguments) = ctx.templateArguments() {
                // TODO use the template arguments
                let _template_argument =
                    ZserioParserVisitorCompat::visit_templateArguments(self, &*template_arguments);
            }

            if name.contains(":") {
                let bits_subst: Vec<&str> = name.split(":").collect();
                bits = i8::from_str_radix(bits_subst[1], 10).expect("failed to convert to i8");
                name = bits_subst[0].into();
            }

            return ZserioTreeReturnType::TypeReference(Box::new(TypeReference {
                is_builtin: true,
                package: "".into(),
                name,
                bits,
            }));
        }

        let mut name = "".into();
        match ZserioParserVisitorCompat::visit_qualifiedName(self, &*ctx.qualifiedName().unwrap()) {
            ZserioTreeReturnType::Str(s) => name = s,
            _ => panic!("error"),
        }
        let mut package: String = "".into();
        if name.contains(".") {
            let (new_name, new_package) = name.rsplit_once(".").unwrap();
            package = new_package.into();
            name = new_name.into();
        }

        /*
        var templateArguments []*ast.TypeReference
        if ta := ctx.TemplateArguments(); ta != nil {
            templateArguments = v.Visit(ta.(antlr.ParseTree)).([]*ast.TypeReference)
        }
        name := v.Visit(ctx.QualifiedName().(antlr.ParseTree)).(string)
        pkg := ""
        if ix := strings.LastIndex(name, "."); ix != -1 {
            pkg = name[:ix]
            name = name[ix+1:]
        }
        return &ast.TypeReference{
            IsBuiltin:         false,
            Package:           pkg,
            Name:              name,
            TemplateArguments: templateArguments,
        }
        */
        ZserioTreeReturnType::TypeReference(Box::new(TypeReference {
            is_builtin: false,
            package,
            name,
            bits: 0,
        }))
    }

    /*
    func (v *Visitor) VisitTypeArguments(ctx *parser.TypeArgumentsContext) any {
        var params []*ast.Expression
        for _, ta := range ctx.AllTypeArgument() {
            params = append(params, v.VisitTypeArgument(ta.(*parser.TypeArgumentContext)).(*ast.Expression))
        }
        return params
    }
     */
}
