#![allow(nonstandard_style)]
// Generated from ZserioParser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::zserioparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ZserioParser}.
 */
pub trait ZserioParserVisitor<'input>: ParseTreeVisitor<'input,ZserioParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ZserioParser#packageDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_packageDeclaration(&mut self, ctx: &PackageDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#packageNameDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_packageNameDefinition(&mut self, ctx: &PackageNameDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#importDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#languageDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_languageDirective(&mut self, ctx: &LanguageDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_typeDeclaration(&mut self, ctx: &TypeDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#symbolDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolDefinition(&mut self, ctx: &SymbolDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#constDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_constDefinition(&mut self, ctx: &ConstDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#ruleGroupDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_ruleGroupDefinition(&mut self, ctx: &RuleGroupDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#ruleDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_ruleDefinition(&mut self, ctx: &RuleDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#subtypeDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_subtypeDeclaration(&mut self, ctx: &SubtypeDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#structureDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_structureDeclaration(&mut self, ctx: &StructureDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#structureFieldDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_structureFieldDefinition(&mut self, ctx: &StructureFieldDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldAlignment}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldAlignment(&mut self, ctx: &FieldAlignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldOffset}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldOffset(&mut self, ctx: &FieldOffsetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldTypeId}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldTypeId(&mut self, ctx: &FieldTypeIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldArrayRange}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldArrayRange(&mut self, ctx: &FieldArrayRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldInitializer(&mut self, ctx: &FieldInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldOptionalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldOptionalClause(&mut self, ctx: &FieldOptionalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldConstraint(&mut self, ctx: &FieldConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_choiceDeclaration(&mut self, ctx: &ChoiceDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceCases}.
	 * @param ctx the parse tree
	 */
	fn visit_choiceCases(&mut self, ctx: &ChoiceCasesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceCase}.
	 * @param ctx the parse tree
	 */
	fn visit_choiceCase(&mut self, ctx: &ChoiceCaseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceDefault}.
	 * @param ctx the parse tree
	 */
	fn visit_choiceDefault(&mut self, ctx: &ChoiceDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceFieldDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_choiceFieldDefinition(&mut self, ctx: &ChoiceFieldDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#unionDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_unionDeclaration(&mut self, ctx: &UnionDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#unionFieldDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_unionFieldDefinition(&mut self, ctx: &UnionFieldDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#enumDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#enumItem}.
	 * @param ctx the parse tree
	 */
	fn visit_enumItem(&mut self, ctx: &EnumItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#bitmaskDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_bitmaskDeclaration(&mut self, ctx: &BitmaskDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#bitmaskValue}.
	 * @param ctx the parse tree
	 */
	fn visit_bitmaskValue(&mut self, ctx: &BitmaskValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlTableDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlTableDeclaration(&mut self, ctx: &SqlTableDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlTableFieldDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlTableFieldDefinition(&mut self, ctx: &SqlTableFieldDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlConstraintDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlConstraintDefinition(&mut self, ctx: &SqlConstraintDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlConstraint(&mut self, ctx: &SqlConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlWithoutRowId}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlWithoutRowId(&mut self, ctx: &SqlWithoutRowIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlDatabaseDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlDatabaseDefinition(&mut self, ctx: &SqlDatabaseDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlDatabaseFieldDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlDatabaseFieldDefinition(&mut self, ctx: &SqlDatabaseFieldDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#serviceDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceDefinition(&mut self, ctx: &ServiceDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#serviceMethodDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceMethodDefinition(&mut self, ctx: &ServiceMethodDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#pubsubDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_pubsubDefinition(&mut self, ctx: &PubsubDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#pubsubMessageDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_pubsubMessageDefinition(&mut self, ctx: &PubsubMessageDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#topicDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_topicDefinition(&mut self, ctx: &TopicDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#functionDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#functionType}.
	 * @param ctx the parse tree
	 */
	fn visit_functionType(&mut self, ctx: &FunctionTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#functionName}.
	 * @param ctx the parse tree
	 */
	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#functionBody}.
	 * @param ctx the parse tree
	 */
	fn visit_functionBody(&mut self, ctx: &FunctionBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeParameters}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParameters(&mut self, ctx: &TypeParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#parameterDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterDefinition(&mut self, ctx: &ParameterDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#templateParameters}.
	 * @param ctx the parse tree
	 */
	fn visit_templateParameters(&mut self, ctx: &TemplateParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#templateArguments}.
	 * @param ctx the parse tree
	 */
	fn visit_templateArguments(&mut self, ctx: &TemplateArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#templateArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#instantiateDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_instantiateDeclaration(&mut self, ctx: &InstantiateDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bitwiseXorExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwiseXorExpression(&mut self, ctx: &BitwiseXorExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dotExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_dotExpression(&mut self, ctx: &DotExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueofExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_valueofExpression(&mut self, ctx: &ValueofExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code shiftExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code arrayExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayExpression(&mut self, ctx: &ArrayExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numbitsExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_numbitsExpression(&mut self, ctx: &NumbitsExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code additiveExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationalExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code lengthofExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_lengthofExpression(&mut self, ctx: &LengthofExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code identifierExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierExpression(&mut self, ctx: &IdentifierExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multiplicativeExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalOrExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bitwiseOrExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parenthesizedExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_parenthesizedExpression(&mut self, ctx: &ParenthesizedExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bitwiseAndExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code equalityExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalAndExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code functionCallExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code indexExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unaryExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code literalExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ternaryExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_ternaryExpression(&mut self, ctx: &TernaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#id}.
	 * @param ctx the parse tree
	 */
	fn visit_id(&mut self, ctx: &IdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeReference}.
	 * @param ctx the parse tree
	 */
	fn visit_typeReference(&mut self, ctx: &TypeReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeInstantiation}.
	 * @param ctx the parse tree
	 */
	fn visit_typeInstantiation(&mut self, ctx: &TypeInstantiationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#builtinType}.
	 * @param ctx the parse tree
	 */
	fn visit_builtinType(&mut self, ctx: &BuiltinTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#qualifiedName}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeArguments}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArguments(&mut self, ctx: &TypeArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArgument(&mut self, ctx: &TypeArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#dynamicLengthArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_dynamicLengthArgument(&mut self, ctx: &DynamicLengthArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#intType}.
	 * @param ctx the parse tree
	 */
	fn visit_intType(&mut self, ctx: &IntTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#varintType}.
	 * @param ctx the parse tree
	 */
	fn visit_varintType(&mut self, ctx: &VarintTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fixedBitFieldType}.
	 * @param ctx the parse tree
	 */
	fn visit_fixedBitFieldType(&mut self, ctx: &FixedBitFieldTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#dynamicBitFieldType}.
	 * @param ctx the parse tree
	 */
	fn visit_dynamicBitFieldType(&mut self, ctx: &DynamicBitFieldTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#boolType}.
	 * @param ctx the parse tree
	 */
	fn visit_boolType(&mut self, ctx: &BoolTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#stringType}.
	 * @param ctx the parse tree
	 */
	fn visit_stringType(&mut self, ctx: &StringTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#floatType}.
	 * @param ctx the parse tree
	 */
	fn visit_floatType(&mut self, ctx: &FloatTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ZserioParser#externType}.
	 * @param ctx the parse tree
	 */
	fn visit_externType(&mut self, ctx: &ExternTypeContext<'input>) { self.visit_children(ctx) }

}

pub trait ZserioParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= ZserioParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ZserioParser#packageDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_packageDeclaration(&mut self, ctx: &PackageDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#packageNameDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_packageNameDefinition(&mut self, ctx: &PackageNameDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#importDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#languageDirective}.
	 * @param ctx the parse tree
	 */
		fn visit_languageDirective(&mut self, ctx: &LanguageDirectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_typeDeclaration(&mut self, ctx: &TypeDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#symbolDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolDefinition(&mut self, ctx: &SymbolDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#constDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_constDefinition(&mut self, ctx: &ConstDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#ruleGroupDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_ruleGroupDefinition(&mut self, ctx: &RuleGroupDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#ruleDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_ruleDefinition(&mut self, ctx: &RuleDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#subtypeDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_subtypeDeclaration(&mut self, ctx: &SubtypeDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#structureDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_structureDeclaration(&mut self, ctx: &StructureDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#structureFieldDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_structureFieldDefinition(&mut self, ctx: &StructureFieldDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldAlignment}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldAlignment(&mut self, ctx: &FieldAlignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldOffset}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldOffset(&mut self, ctx: &FieldOffsetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldTypeId}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldTypeId(&mut self, ctx: &FieldTypeIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldArrayRange}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldArrayRange(&mut self, ctx: &FieldArrayRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldInitializer(&mut self, ctx: &FieldInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldOptionalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldOptionalClause(&mut self, ctx: &FieldOptionalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fieldConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldConstraint(&mut self, ctx: &FieldConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_choiceDeclaration(&mut self, ctx: &ChoiceDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceCases}.
	 * @param ctx the parse tree
	 */
		fn visit_choiceCases(&mut self, ctx: &ChoiceCasesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceCase}.
	 * @param ctx the parse tree
	 */
		fn visit_choiceCase(&mut self, ctx: &ChoiceCaseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceDefault}.
	 * @param ctx the parse tree
	 */
		fn visit_choiceDefault(&mut self, ctx: &ChoiceDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#choiceFieldDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_choiceFieldDefinition(&mut self, ctx: &ChoiceFieldDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#unionDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_unionDeclaration(&mut self, ctx: &UnionDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#unionFieldDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_unionFieldDefinition(&mut self, ctx: &UnionFieldDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#enumDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#enumItem}.
	 * @param ctx the parse tree
	 */
		fn visit_enumItem(&mut self, ctx: &EnumItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#bitmaskDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_bitmaskDeclaration(&mut self, ctx: &BitmaskDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#bitmaskValue}.
	 * @param ctx the parse tree
	 */
		fn visit_bitmaskValue(&mut self, ctx: &BitmaskValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlTableDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlTableDeclaration(&mut self, ctx: &SqlTableDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlTableFieldDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlTableFieldDefinition(&mut self, ctx: &SqlTableFieldDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlConstraintDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlConstraintDefinition(&mut self, ctx: &SqlConstraintDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlConstraint(&mut self, ctx: &SqlConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlWithoutRowId}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlWithoutRowId(&mut self, ctx: &SqlWithoutRowIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlDatabaseDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlDatabaseDefinition(&mut self, ctx: &SqlDatabaseDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#sqlDatabaseFieldDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlDatabaseFieldDefinition(&mut self, ctx: &SqlDatabaseFieldDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#serviceDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceDefinition(&mut self, ctx: &ServiceDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#serviceMethodDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceMethodDefinition(&mut self, ctx: &ServiceMethodDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#pubsubDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_pubsubDefinition(&mut self, ctx: &PubsubDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#pubsubMessageDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_pubsubMessageDefinition(&mut self, ctx: &PubsubMessageDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#topicDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_topicDefinition(&mut self, ctx: &TopicDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#functionDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#functionType}.
	 * @param ctx the parse tree
	 */
		fn visit_functionType(&mut self, ctx: &FunctionTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#functionName}.
	 * @param ctx the parse tree
	 */
		fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#functionBody}.
	 * @param ctx the parse tree
	 */
		fn visit_functionBody(&mut self, ctx: &FunctionBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeParameters}.
	 * @param ctx the parse tree
	 */
		fn visit_typeParameters(&mut self, ctx: &TypeParametersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#parameterDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterDefinition(&mut self, ctx: &ParameterDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#templateParameters}.
	 * @param ctx the parse tree
	 */
		fn visit_templateParameters(&mut self, ctx: &TemplateParametersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#templateArguments}.
	 * @param ctx the parse tree
	 */
		fn visit_templateArguments(&mut self, ctx: &TemplateArgumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#templateArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#instantiateDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_instantiateDeclaration(&mut self, ctx: &InstantiateDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bitwiseXorExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwiseXorExpression(&mut self, ctx: &BitwiseXorExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dotExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_dotExpression(&mut self, ctx: &DotExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueofExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_valueofExpression(&mut self, ctx: &ValueofExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code shiftExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code arrayExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayExpression(&mut self, ctx: &ArrayExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numbitsExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_numbitsExpression(&mut self, ctx: &NumbitsExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code additiveExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationalExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code lengthofExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_lengthofExpression(&mut self, ctx: &LengthofExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code identifierExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierExpression(&mut self, ctx: &IdentifierExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multiplicativeExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalOrExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bitwiseOrExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parenthesizedExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_parenthesizedExpression(&mut self, ctx: &ParenthesizedExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bitwiseAndExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code equalityExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalAndExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code functionCallExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code indexExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unaryExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code literalExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ternaryExpression}
	 * labeled alternative in {@link ZserioParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_ternaryExpression(&mut self, ctx: &TernaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#id}.
	 * @param ctx the parse tree
	 */
		fn visit_id(&mut self, ctx: &IdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeReference}.
	 * @param ctx the parse tree
	 */
		fn visit_typeReference(&mut self, ctx: &TypeReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeInstantiation}.
	 * @param ctx the parse tree
	 */
		fn visit_typeInstantiation(&mut self, ctx: &TypeInstantiationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#builtinType}.
	 * @param ctx the parse tree
	 */
		fn visit_builtinType(&mut self, ctx: &BuiltinTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#qualifiedName}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeArguments}.
	 * @param ctx the parse tree
	 */
		fn visit_typeArguments(&mut self, ctx: &TypeArgumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#typeArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_typeArgument(&mut self, ctx: &TypeArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#dynamicLengthArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_dynamicLengthArgument(&mut self, ctx: &DynamicLengthArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#intType}.
	 * @param ctx the parse tree
	 */
		fn visit_intType(&mut self, ctx: &IntTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#varintType}.
	 * @param ctx the parse tree
	 */
		fn visit_varintType(&mut self, ctx: &VarintTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#fixedBitFieldType}.
	 * @param ctx the parse tree
	 */
		fn visit_fixedBitFieldType(&mut self, ctx: &FixedBitFieldTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#dynamicBitFieldType}.
	 * @param ctx the parse tree
	 */
		fn visit_dynamicBitFieldType(&mut self, ctx: &DynamicBitFieldTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#boolType}.
	 * @param ctx the parse tree
	 */
		fn visit_boolType(&mut self, ctx: &BoolTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#stringType}.
	 * @param ctx the parse tree
	 */
		fn visit_stringType(&mut self, ctx: &StringTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#floatType}.
	 * @param ctx the parse tree
	 */
		fn visit_floatType(&mut self, ctx: &FloatTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ZserioParser#externType}.
	 * @param ctx the parse tree
	 */
		fn visit_externType(&mut self, ctx: &ExternTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> ZserioParserVisitor<'input> for T
where
	T: ZserioParserVisitorCompat<'input>
{
	fn visit_packageDeclaration(&mut self, ctx: &PackageDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_packageDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_packageNameDefinition(&mut self, ctx: &PackageNameDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_packageNameDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_importDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_languageDirective(&mut self, ctx: &LanguageDirectiveContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_languageDirective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeDeclaration(&mut self, ctx: &TypeDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_typeDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolDefinition(&mut self, ctx: &SymbolDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_symbolDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constDefinition(&mut self, ctx: &ConstDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_constDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ruleGroupDefinition(&mut self, ctx: &RuleGroupDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_ruleGroupDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ruleDefinition(&mut self, ctx: &RuleDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_ruleDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtypeDeclaration(&mut self, ctx: &SubtypeDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_subtypeDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structureDeclaration(&mut self, ctx: &StructureDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_structureDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structureFieldDefinition(&mut self, ctx: &StructureFieldDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_structureFieldDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldAlignment(&mut self, ctx: &FieldAlignmentContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_fieldAlignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldOffset(&mut self, ctx: &FieldOffsetContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_fieldOffset(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldTypeId(&mut self, ctx: &FieldTypeIdContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_fieldTypeId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldArrayRange(&mut self, ctx: &FieldArrayRangeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_fieldArrayRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldInitializer(&mut self, ctx: &FieldInitializerContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_fieldInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldOptionalClause(&mut self, ctx: &FieldOptionalClauseContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_fieldOptionalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldConstraint(&mut self, ctx: &FieldConstraintContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_fieldConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_choiceDeclaration(&mut self, ctx: &ChoiceDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_choiceDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_choiceCases(&mut self, ctx: &ChoiceCasesContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_choiceCases(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_choiceCase(&mut self, ctx: &ChoiceCaseContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_choiceCase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_choiceDefault(&mut self, ctx: &ChoiceDefaultContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_choiceDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_choiceFieldDefinition(&mut self, ctx: &ChoiceFieldDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_choiceFieldDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unionDeclaration(&mut self, ctx: &UnionDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_unionDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unionFieldDefinition(&mut self, ctx: &UnionFieldDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_unionFieldDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_enumDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumItem(&mut self, ctx: &EnumItemContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_enumItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitmaskDeclaration(&mut self, ctx: &BitmaskDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_bitmaskDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitmaskValue(&mut self, ctx: &BitmaskValueContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_bitmaskValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlTableDeclaration(&mut self, ctx: &SqlTableDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_sqlTableDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlTableFieldDefinition(&mut self, ctx: &SqlTableFieldDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_sqlTableFieldDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlConstraintDefinition(&mut self, ctx: &SqlConstraintDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_sqlConstraintDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlConstraint(&mut self, ctx: &SqlConstraintContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_sqlConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlWithoutRowId(&mut self, ctx: &SqlWithoutRowIdContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_sqlWithoutRowId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlDatabaseDefinition(&mut self, ctx: &SqlDatabaseDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_sqlDatabaseDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlDatabaseFieldDefinition(&mut self, ctx: &SqlDatabaseFieldDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_sqlDatabaseFieldDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceDefinition(&mut self, ctx: &ServiceDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_serviceDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceMethodDefinition(&mut self, ctx: &ServiceMethodDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_serviceMethodDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pubsubDefinition(&mut self, ctx: &PubsubDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_pubsubDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pubsubMessageDefinition(&mut self, ctx: &PubsubMessageDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_pubsubMessageDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_topicDefinition(&mut self, ctx: &TopicDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_topicDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_functionDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionType(&mut self, ctx: &FunctionTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_functionType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_functionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionBody(&mut self, ctx: &FunctionBodyContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_functionBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeParameters(&mut self, ctx: &TypeParametersContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_typeParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterDefinition(&mut self, ctx: &ParameterDefinitionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_parameterDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateParameters(&mut self, ctx: &TemplateParametersContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_templateParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateArguments(&mut self, ctx: &TemplateArgumentsContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_templateArguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_templateArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_instantiateDeclaration(&mut self, ctx: &InstantiateDeclarationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_instantiateDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwiseXorExpression(&mut self, ctx: &BitwiseXorExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_bitwiseXorExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dotExpression(&mut self, ctx: &DotExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_dotExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueofExpression(&mut self, ctx: &ValueofExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_valueofExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_shiftExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayExpression(&mut self, ctx: &ArrayExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_arrayExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numbitsExpression(&mut self, ctx: &NumbitsExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_numbitsExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_additiveExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_relationalExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lengthofExpression(&mut self, ctx: &LengthofExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_lengthofExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierExpression(&mut self, ctx: &IdentifierExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_identifierExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_multiplicativeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_logicalOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_bitwiseOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parenthesizedExpression(&mut self, ctx: &ParenthesizedExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_parenthesizedExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_bitwiseAndExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_equalityExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_logicalAndExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionCallExpression(&mut self, ctx: &FunctionCallExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_functionCallExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_indexExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_unaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_literalExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ternaryExpression(&mut self, ctx: &TernaryExpressionContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_ternaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_id(&mut self, ctx: &IdContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_id(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeReference(&mut self, ctx: &TypeReferenceContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_typeReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeInstantiation(&mut self, ctx: &TypeInstantiationContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_typeInstantiation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_builtinType(&mut self, ctx: &BuiltinTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_builtinType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_qualifiedName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeArguments(&mut self, ctx: &TypeArgumentsContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_typeArguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeArgument(&mut self, ctx: &TypeArgumentContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_typeArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dynamicLengthArgument(&mut self, ctx: &DynamicLengthArgumentContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_dynamicLengthArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_intType(&mut self, ctx: &IntTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_intType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_varintType(&mut self, ctx: &VarintTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_varintType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fixedBitFieldType(&mut self, ctx: &FixedBitFieldTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_fixedBitFieldType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dynamicBitFieldType(&mut self, ctx: &DynamicBitFieldTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_dynamicBitFieldType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_boolType(&mut self, ctx: &BoolTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_boolType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringType(&mut self, ctx: &StringTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_stringType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_floatType(&mut self, ctx: &FloatTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_floatType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_externType(&mut self, ctx: &ExternTypeContext<'input>){
		let result = <Self as ZserioParserVisitorCompat>::visit_externType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}