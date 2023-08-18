#![allow(nonstandard_style)]
// Generated from ZserioParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::zserioparser::*;

pub trait ZserioParserListener<'input> : ParseTreeListener<'input,ZserioParserContextType>{
/**
 * Enter a parse tree produced by {@link ZserioParser#packageDeclaration}.
 * @param ctx the parse tree
 */
fn enter_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#packageDeclaration}.
 * @param ctx the parse tree
 */
fn exit_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#compatibilityVersionDirective}.
 * @param ctx the parse tree
 */
fn enter_compatibilityVersionDirective(&mut self, _ctx: &CompatibilityVersionDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#compatibilityVersionDirective}.
 * @param ctx the parse tree
 */
fn exit_compatibilityVersionDirective(&mut self, _ctx: &CompatibilityVersionDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#packageNameDefinition}.
 * @param ctx the parse tree
 */
fn enter_packageNameDefinition(&mut self, _ctx: &PackageNameDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#packageNameDefinition}.
 * @param ctx the parse tree
 */
fn exit_packageNameDefinition(&mut self, _ctx: &PackageNameDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#importDeclaration}.
 * @param ctx the parse tree
 */
fn enter_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#importDeclaration}.
 * @param ctx the parse tree
 */
fn exit_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#languageDirective}.
 * @param ctx the parse tree
 */
fn enter_languageDirective(&mut self, _ctx: &LanguageDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#languageDirective}.
 * @param ctx the parse tree
 */
fn exit_languageDirective(&mut self, _ctx: &LanguageDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#typeDeclaration}.
 * @param ctx the parse tree
 */
fn enter_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#typeDeclaration}.
 * @param ctx the parse tree
 */
fn exit_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#symbolDefinition}.
 * @param ctx the parse tree
 */
fn enter_symbolDefinition(&mut self, _ctx: &SymbolDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#symbolDefinition}.
 * @param ctx the parse tree
 */
fn exit_symbolDefinition(&mut self, _ctx: &SymbolDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#constDefinition}.
 * @param ctx the parse tree
 */
fn enter_constDefinition(&mut self, _ctx: &ConstDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#constDefinition}.
 * @param ctx the parse tree
 */
fn exit_constDefinition(&mut self, _ctx: &ConstDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#ruleGroupDefinition}.
 * @param ctx the parse tree
 */
fn enter_ruleGroupDefinition(&mut self, _ctx: &RuleGroupDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#ruleGroupDefinition}.
 * @param ctx the parse tree
 */
fn exit_ruleGroupDefinition(&mut self, _ctx: &RuleGroupDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#ruleDefinition}.
 * @param ctx the parse tree
 */
fn enter_ruleDefinition(&mut self, _ctx: &RuleDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#ruleDefinition}.
 * @param ctx the parse tree
 */
fn exit_ruleDefinition(&mut self, _ctx: &RuleDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#subtypeDeclaration}.
 * @param ctx the parse tree
 */
fn enter_subtypeDeclaration(&mut self, _ctx: &SubtypeDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#subtypeDeclaration}.
 * @param ctx the parse tree
 */
fn exit_subtypeDeclaration(&mut self, _ctx: &SubtypeDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#structureDeclaration}.
 * @param ctx the parse tree
 */
fn enter_structureDeclaration(&mut self, _ctx: &StructureDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#structureDeclaration}.
 * @param ctx the parse tree
 */
fn exit_structureDeclaration(&mut self, _ctx: &StructureDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#structureFieldDefinition}.
 * @param ctx the parse tree
 */
fn enter_structureFieldDefinition(&mut self, _ctx: &StructureFieldDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#structureFieldDefinition}.
 * @param ctx the parse tree
 */
fn exit_structureFieldDefinition(&mut self, _ctx: &StructureFieldDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#fieldAlignment}.
 * @param ctx the parse tree
 */
fn enter_fieldAlignment(&mut self, _ctx: &FieldAlignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#fieldAlignment}.
 * @param ctx the parse tree
 */
fn exit_fieldAlignment(&mut self, _ctx: &FieldAlignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#fieldOffset}.
 * @param ctx the parse tree
 */
fn enter_fieldOffset(&mut self, _ctx: &FieldOffsetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#fieldOffset}.
 * @param ctx the parse tree
 */
fn exit_fieldOffset(&mut self, _ctx: &FieldOffsetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#fieldTypeId}.
 * @param ctx the parse tree
 */
fn enter_fieldTypeId(&mut self, _ctx: &FieldTypeIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#fieldTypeId}.
 * @param ctx the parse tree
 */
fn exit_fieldTypeId(&mut self, _ctx: &FieldTypeIdContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#fieldArrayRange}.
 * @param ctx the parse tree
 */
fn enter_fieldArrayRange(&mut self, _ctx: &FieldArrayRangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#fieldArrayRange}.
 * @param ctx the parse tree
 */
fn exit_fieldArrayRange(&mut self, _ctx: &FieldArrayRangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#fieldInitializer}.
 * @param ctx the parse tree
 */
fn enter_fieldInitializer(&mut self, _ctx: &FieldInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#fieldInitializer}.
 * @param ctx the parse tree
 */
fn exit_fieldInitializer(&mut self, _ctx: &FieldInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#fieldOptionalClause}.
 * @param ctx the parse tree
 */
fn enter_fieldOptionalClause(&mut self, _ctx: &FieldOptionalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#fieldOptionalClause}.
 * @param ctx the parse tree
 */
fn exit_fieldOptionalClause(&mut self, _ctx: &FieldOptionalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#fieldConstraint}.
 * @param ctx the parse tree
 */
fn enter_fieldConstraint(&mut self, _ctx: &FieldConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#fieldConstraint}.
 * @param ctx the parse tree
 */
fn exit_fieldConstraint(&mut self, _ctx: &FieldConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#choiceDeclaration}.
 * @param ctx the parse tree
 */
fn enter_choiceDeclaration(&mut self, _ctx: &ChoiceDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#choiceDeclaration}.
 * @param ctx the parse tree
 */
fn exit_choiceDeclaration(&mut self, _ctx: &ChoiceDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#choiceCases}.
 * @param ctx the parse tree
 */
fn enter_choiceCases(&mut self, _ctx: &ChoiceCasesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#choiceCases}.
 * @param ctx the parse tree
 */
fn exit_choiceCases(&mut self, _ctx: &ChoiceCasesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#choiceCase}.
 * @param ctx the parse tree
 */
fn enter_choiceCase(&mut self, _ctx: &ChoiceCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#choiceCase}.
 * @param ctx the parse tree
 */
fn exit_choiceCase(&mut self, _ctx: &ChoiceCaseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#choiceDefault}.
 * @param ctx the parse tree
 */
fn enter_choiceDefault(&mut self, _ctx: &ChoiceDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#choiceDefault}.
 * @param ctx the parse tree
 */
fn exit_choiceDefault(&mut self, _ctx: &ChoiceDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#choiceFieldDefinition}.
 * @param ctx the parse tree
 */
fn enter_choiceFieldDefinition(&mut self, _ctx: &ChoiceFieldDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#choiceFieldDefinition}.
 * @param ctx the parse tree
 */
fn exit_choiceFieldDefinition(&mut self, _ctx: &ChoiceFieldDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#unionDeclaration}.
 * @param ctx the parse tree
 */
fn enter_unionDeclaration(&mut self, _ctx: &UnionDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#unionDeclaration}.
 * @param ctx the parse tree
 */
fn exit_unionDeclaration(&mut self, _ctx: &UnionDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#unionFieldDefinition}.
 * @param ctx the parse tree
 */
fn enter_unionFieldDefinition(&mut self, _ctx: &UnionFieldDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#unionFieldDefinition}.
 * @param ctx the parse tree
 */
fn exit_unionFieldDefinition(&mut self, _ctx: &UnionFieldDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#enumDeclaration}.
 * @param ctx the parse tree
 */
fn enter_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#enumDeclaration}.
 * @param ctx the parse tree
 */
fn exit_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#enumItem}.
 * @param ctx the parse tree
 */
fn enter_enumItem(&mut self, _ctx: &EnumItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#enumItem}.
 * @param ctx the parse tree
 */
fn exit_enumItem(&mut self, _ctx: &EnumItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#bitmaskDeclaration}.
 * @param ctx the parse tree
 */
fn enter_bitmaskDeclaration(&mut self, _ctx: &BitmaskDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#bitmaskDeclaration}.
 * @param ctx the parse tree
 */
fn exit_bitmaskDeclaration(&mut self, _ctx: &BitmaskDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#bitmaskValue}.
 * @param ctx the parse tree
 */
fn enter_bitmaskValue(&mut self, _ctx: &BitmaskValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#bitmaskValue}.
 * @param ctx the parse tree
 */
fn exit_bitmaskValue(&mut self, _ctx: &BitmaskValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#sqlTableDeclaration}.
 * @param ctx the parse tree
 */
fn enter_sqlTableDeclaration(&mut self, _ctx: &SqlTableDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#sqlTableDeclaration}.
 * @param ctx the parse tree
 */
fn exit_sqlTableDeclaration(&mut self, _ctx: &SqlTableDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#sqlTableFieldDefinition}.
 * @param ctx the parse tree
 */
fn enter_sqlTableFieldDefinition(&mut self, _ctx: &SqlTableFieldDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#sqlTableFieldDefinition}.
 * @param ctx the parse tree
 */
fn exit_sqlTableFieldDefinition(&mut self, _ctx: &SqlTableFieldDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#sqlConstraintDefinition}.
 * @param ctx the parse tree
 */
fn enter_sqlConstraintDefinition(&mut self, _ctx: &SqlConstraintDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#sqlConstraintDefinition}.
 * @param ctx the parse tree
 */
fn exit_sqlConstraintDefinition(&mut self, _ctx: &SqlConstraintDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#sqlConstraint}.
 * @param ctx the parse tree
 */
fn enter_sqlConstraint(&mut self, _ctx: &SqlConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#sqlConstraint}.
 * @param ctx the parse tree
 */
fn exit_sqlConstraint(&mut self, _ctx: &SqlConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#sqlWithoutRowId}.
 * @param ctx the parse tree
 */
fn enter_sqlWithoutRowId(&mut self, _ctx: &SqlWithoutRowIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#sqlWithoutRowId}.
 * @param ctx the parse tree
 */
fn exit_sqlWithoutRowId(&mut self, _ctx: &SqlWithoutRowIdContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#sqlDatabaseDefinition}.
 * @param ctx the parse tree
 */
fn enter_sqlDatabaseDefinition(&mut self, _ctx: &SqlDatabaseDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#sqlDatabaseDefinition}.
 * @param ctx the parse tree
 */
fn exit_sqlDatabaseDefinition(&mut self, _ctx: &SqlDatabaseDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#sqlDatabaseFieldDefinition}.
 * @param ctx the parse tree
 */
fn enter_sqlDatabaseFieldDefinition(&mut self, _ctx: &SqlDatabaseFieldDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#sqlDatabaseFieldDefinition}.
 * @param ctx the parse tree
 */
fn exit_sqlDatabaseFieldDefinition(&mut self, _ctx: &SqlDatabaseFieldDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#serviceDefinition}.
 * @param ctx the parse tree
 */
fn enter_serviceDefinition(&mut self, _ctx: &ServiceDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#serviceDefinition}.
 * @param ctx the parse tree
 */
fn exit_serviceDefinition(&mut self, _ctx: &ServiceDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#serviceMethodDefinition}.
 * @param ctx the parse tree
 */
fn enter_serviceMethodDefinition(&mut self, _ctx: &ServiceMethodDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#serviceMethodDefinition}.
 * @param ctx the parse tree
 */
fn exit_serviceMethodDefinition(&mut self, _ctx: &ServiceMethodDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#pubsubDefinition}.
 * @param ctx the parse tree
 */
fn enter_pubsubDefinition(&mut self, _ctx: &PubsubDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#pubsubDefinition}.
 * @param ctx the parse tree
 */
fn exit_pubsubDefinition(&mut self, _ctx: &PubsubDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#pubsubMessageDefinition}.
 * @param ctx the parse tree
 */
fn enter_pubsubMessageDefinition(&mut self, _ctx: &PubsubMessageDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#pubsubMessageDefinition}.
 * @param ctx the parse tree
 */
fn exit_pubsubMessageDefinition(&mut self, _ctx: &PubsubMessageDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#topicDefinition}.
 * @param ctx the parse tree
 */
fn enter_topicDefinition(&mut self, _ctx: &TopicDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#topicDefinition}.
 * @param ctx the parse tree
 */
fn exit_topicDefinition(&mut self, _ctx: &TopicDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#functionDefinition}.
 * @param ctx the parse tree
 */
fn enter_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#functionDefinition}.
 * @param ctx the parse tree
 */
fn exit_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#functionType}.
 * @param ctx the parse tree
 */
fn enter_functionType(&mut self, _ctx: &FunctionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#functionType}.
 * @param ctx the parse tree
 */
fn exit_functionType(&mut self, _ctx: &FunctionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#functionName}.
 * @param ctx the parse tree
 */
fn enter_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#functionName}.
 * @param ctx the parse tree
 */
fn exit_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#functionBody}.
 * @param ctx the parse tree
 */
fn enter_functionBody(&mut self, _ctx: &FunctionBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#functionBody}.
 * @param ctx the parse tree
 */
fn exit_functionBody(&mut self, _ctx: &FunctionBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#typeParameters}.
 * @param ctx the parse tree
 */
fn enter_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#typeParameters}.
 * @param ctx the parse tree
 */
fn exit_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#parameterDefinition}.
 * @param ctx the parse tree
 */
fn enter_parameterDefinition(&mut self, _ctx: &ParameterDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#parameterDefinition}.
 * @param ctx the parse tree
 */
fn exit_parameterDefinition(&mut self, _ctx: &ParameterDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#templateParameters}.
 * @param ctx the parse tree
 */
fn enter_templateParameters(&mut self, _ctx: &TemplateParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#templateParameters}.
 * @param ctx the parse tree
 */
fn exit_templateParameters(&mut self, _ctx: &TemplateParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#templateArguments}.
 * @param ctx the parse tree
 */
fn enter_templateArguments(&mut self, _ctx: &TemplateArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#templateArguments}.
 * @param ctx the parse tree
 */
fn exit_templateArguments(&mut self, _ctx: &TemplateArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#templateArgument}.
 * @param ctx the parse tree
 */
fn enter_templateArgument(&mut self, _ctx: &TemplateArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#templateArgument}.
 * @param ctx the parse tree
 */
fn exit_templateArgument(&mut self, _ctx: &TemplateArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#instantiateDeclaration}.
 * @param ctx the parse tree
 */
fn enter_instantiateDeclaration(&mut self, _ctx: &InstantiateDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#instantiateDeclaration}.
 * @param ctx the parse tree
 */
fn exit_instantiateDeclaration(&mut self, _ctx: &InstantiateDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code bitwiseXorExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_bitwiseXorExpression(&mut self, _ctx: &BitwiseXorExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bitwiseXorExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_bitwiseXorExpression(&mut self, _ctx: &BitwiseXorExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dotExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_dotExpression(&mut self, _ctx: &DotExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dotExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_dotExpression(&mut self, _ctx: &DotExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueofExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_valueofExpression(&mut self, _ctx: &ValueofExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueofExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_valueofExpression(&mut self, _ctx: &ValueofExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code shiftExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code shiftExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code arrayExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_arrayExpression(&mut self, _ctx: &ArrayExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code arrayExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_arrayExpression(&mut self, _ctx: &ArrayExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numbitsExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_numbitsExpression(&mut self, _ctx: &NumbitsExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numbitsExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_numbitsExpression(&mut self, _ctx: &NumbitsExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code additiveExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code additiveExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationalExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationalExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code lengthofExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_lengthofExpression(&mut self, _ctx: &LengthofExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code lengthofExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_lengthofExpression(&mut self, _ctx: &LengthofExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code identifierExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_identifierExpression(&mut self, _ctx: &IdentifierExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code identifierExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_identifierExpression(&mut self, _ctx: &IdentifierExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multiplicativeExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multiplicativeExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalOrExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_logicalOrExpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalOrExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_logicalOrExpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code isSetExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_isSetExpression(&mut self, _ctx: &IsSetExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code isSetExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_isSetExpression(&mut self, _ctx: &IsSetExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code bitwiseOrExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_bitwiseOrExpression(&mut self, _ctx: &BitwiseOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bitwiseOrExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_bitwiseOrExpression(&mut self, _ctx: &BitwiseOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parenthesizedExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_parenthesizedExpression(&mut self, _ctx: &ParenthesizedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenthesizedExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_parenthesizedExpression(&mut self, _ctx: &ParenthesizedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code bitwiseAndExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_bitwiseAndExpression(&mut self, _ctx: &BitwiseAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bitwiseAndExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_bitwiseAndExpression(&mut self, _ctx: &BitwiseAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code equalityExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code equalityExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalAndExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_logicalAndExpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalAndExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_logicalAndExpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code functionCallExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_functionCallExpression(&mut self, _ctx: &FunctionCallExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code functionCallExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_functionCallExpression(&mut self, _ctx: &FunctionCallExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code indexExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_indexExpression(&mut self, _ctx: &IndexExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code indexExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_indexExpression(&mut self, _ctx: &IndexExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unaryExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unaryExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code literalExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_literalExpression(&mut self, _ctx: &LiteralExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code literalExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_literalExpression(&mut self, _ctx: &LiteralExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ternaryExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn enter_ternaryExpression(&mut self, _ctx: &TernaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ternaryExpression}
 * labeled alternative in {@link ZserioParser#expression}.
 * @param ctx the parse tree
 */
fn exit_ternaryExpression(&mut self, _ctx: &TernaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#id}.
 * @param ctx the parse tree
 */
fn enter_id(&mut self, _ctx: &IdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#id}.
 * @param ctx the parse tree
 */
fn exit_id(&mut self, _ctx: &IdContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#typeReference}.
 * @param ctx the parse tree
 */
fn enter_typeReference(&mut self, _ctx: &TypeReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#typeReference}.
 * @param ctx the parse tree
 */
fn exit_typeReference(&mut self, _ctx: &TypeReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#typeInstantiation}.
 * @param ctx the parse tree
 */
fn enter_typeInstantiation(&mut self, _ctx: &TypeInstantiationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#typeInstantiation}.
 * @param ctx the parse tree
 */
fn exit_typeInstantiation(&mut self, _ctx: &TypeInstantiationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#builtinType}.
 * @param ctx the parse tree
 */
fn enter_builtinType(&mut self, _ctx: &BuiltinTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#builtinType}.
 * @param ctx the parse tree
 */
fn exit_builtinType(&mut self, _ctx: &BuiltinTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#qualifiedName}.
 * @param ctx the parse tree
 */
fn enter_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#qualifiedName}.
 * @param ctx the parse tree
 */
fn exit_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#typeArguments}.
 * @param ctx the parse tree
 */
fn enter_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#typeArguments}.
 * @param ctx the parse tree
 */
fn exit_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#typeArgument}.
 * @param ctx the parse tree
 */
fn enter_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#typeArgument}.
 * @param ctx the parse tree
 */
fn exit_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#dynamicLengthArgument}.
 * @param ctx the parse tree
 */
fn enter_dynamicLengthArgument(&mut self, _ctx: &DynamicLengthArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#dynamicLengthArgument}.
 * @param ctx the parse tree
 */
fn exit_dynamicLengthArgument(&mut self, _ctx: &DynamicLengthArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#intType}.
 * @param ctx the parse tree
 */
fn enter_intType(&mut self, _ctx: &IntTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#intType}.
 * @param ctx the parse tree
 */
fn exit_intType(&mut self, _ctx: &IntTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#varintType}.
 * @param ctx the parse tree
 */
fn enter_varintType(&mut self, _ctx: &VarintTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#varintType}.
 * @param ctx the parse tree
 */
fn exit_varintType(&mut self, _ctx: &VarintTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#fixedBitFieldType}.
 * @param ctx the parse tree
 */
fn enter_fixedBitFieldType(&mut self, _ctx: &FixedBitFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#fixedBitFieldType}.
 * @param ctx the parse tree
 */
fn exit_fixedBitFieldType(&mut self, _ctx: &FixedBitFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#dynamicBitFieldType}.
 * @param ctx the parse tree
 */
fn enter_dynamicBitFieldType(&mut self, _ctx: &DynamicBitFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#dynamicBitFieldType}.
 * @param ctx the parse tree
 */
fn exit_dynamicBitFieldType(&mut self, _ctx: &DynamicBitFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#boolType}.
 * @param ctx the parse tree
 */
fn enter_boolType(&mut self, _ctx: &BoolTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#boolType}.
 * @param ctx the parse tree
 */
fn exit_boolType(&mut self, _ctx: &BoolTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#stringType}.
 * @param ctx the parse tree
 */
fn enter_stringType(&mut self, _ctx: &StringTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#stringType}.
 * @param ctx the parse tree
 */
fn exit_stringType(&mut self, _ctx: &StringTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#floatType}.
 * @param ctx the parse tree
 */
fn enter_floatType(&mut self, _ctx: &FloatTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#floatType}.
 * @param ctx the parse tree
 */
fn exit_floatType(&mut self, _ctx: &FloatTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#externType}.
 * @param ctx the parse tree
 */
fn enter_externType(&mut self, _ctx: &ExternTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#externType}.
 * @param ctx the parse tree
 */
fn exit_externType(&mut self, _ctx: &ExternTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ZserioParser#bytesType}.
 * @param ctx the parse tree
 */
fn enter_bytesType(&mut self, _ctx: &BytesTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ZserioParser#bytesType}.
 * @param ctx the parse tree
 */
fn exit_bytesType(&mut self, _ctx: &BytesTypeContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : ZserioParserListener<'input> }


