// Generated from ZserioParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use super::zserioparserlistener::*;
use super::zserioparservisitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const AND: isize = 1;
pub const ASSIGN: isize = 2;
pub const BANG: isize = 3;
pub const COLON: isize = 4;
pub const COMMA: isize = 5;
pub const DIVIDE: isize = 6;
pub const DOT: isize = 7;
pub const EQ: isize = 8;
pub const GE: isize = 9;
pub const GT: isize = 10;
pub const LBRACE: isize = 11;
pub const LBRACKET: isize = 12;
pub const LE: isize = 13;
pub const LOGICAL_AND: isize = 14;
pub const LOGICAL_OR: isize = 15;
pub const LPAREN: isize = 16;
pub const LSHIFT: isize = 17;
pub const LT: isize = 18;
pub const MINUS: isize = 19;
pub const MODULO: isize = 20;
pub const MULTIPLY: isize = 21;
pub const NE: isize = 22;
pub const OR: isize = 23;
pub const PLUS: isize = 24;
pub const QUESTIONMARK: isize = 25;
pub const RBRACE: isize = 26;
pub const RBRACKET: isize = 27;
pub const RPAREN: isize = 28;
pub const SEMICOLON: isize = 29;
pub const TILDE: isize = 30;
pub const XOR: isize = 31;
pub const ALIGN: isize = 32;
pub const BIT_FIELD: isize = 33;
pub const BOOL: isize = 34;
pub const BITMASK: isize = 35;
pub const CASE: isize = 36;
pub const CHOICE: isize = 37;
pub const CONST: isize = 38;
pub const DEFAULT: isize = 39;
pub const ENUM: isize = 40;
pub const EXPLICIT: isize = 41;
pub const EXTERN: isize = 42;
pub const FLOAT16: isize = 43;
pub const FLOAT32: isize = 44;
pub const FLOAT64: isize = 45;
pub const FUNCTION: isize = 46;
pub const IF: isize = 47;
pub const IMPLICIT: isize = 48;
pub const IMPORT: isize = 49;
pub const INDEX: isize = 50;
pub const INSTANTIATE: isize = 51;
pub const INT_FIELD: isize = 52;
pub const INT16: isize = 53;
pub const INT32: isize = 54;
pub const INT64: isize = 55;
pub const INT8: isize = 56;
pub const LENGTHOF: isize = 57;
pub const NUMBITS: isize = 58;
pub const ON: isize = 59;
pub const OPTIONAL: isize = 60;
pub const PACKAGE: isize = 61;
pub const PACKED: isize = 62;
pub const PUBSUB: isize = 63;
pub const PUBLISH: isize = 64;
pub const RETURN: isize = 65;
pub const RULE: isize = 66;
pub const RULE_GROUP: isize = 67;
pub const SERVICE: isize = 68;
pub const SQL: isize = 69;
pub const SQL_DATABASE: isize = 70;
pub const SQL_TABLE: isize = 71;
pub const SQL_VIRTUAL: isize = 72;
pub const SQL_WITHOUT_ROWID: isize = 73;
pub const STRING: isize = 74;
pub const STRUCTURE: isize = 75;
pub const SUBSCRIBE: isize = 76;
pub const SUBTYPE: isize = 77;
pub const TOPIC: isize = 78;
pub const UINT16: isize = 79;
pub const UINT32: isize = 80;
pub const UINT64: isize = 81;
pub const UINT8: isize = 82;
pub const UNION: isize = 83;
pub const USING: isize = 84;
pub const VALUEOF: isize = 85;
pub const VARINT: isize = 86;
pub const VARINT16: isize = 87;
pub const VARINT32: isize = 88;
pub const VARINT64: isize = 89;
pub const VARSIZE: isize = 90;
pub const VARUINT: isize = 91;
pub const VARUINT16: isize = 92;
pub const VARUINT32: isize = 93;
pub const VARUINT64: isize = 94;
pub const WS: isize = 95;
pub const DOC_COMMENT: isize = 96;
pub const MARKDOWN_COMMENT: isize = 97;
pub const BLOCK_COMMENT: isize = 98;
pub const LINE_COMMENT: isize = 99;
pub const BOOL_LITERAL: isize = 100;
pub const STRING_LITERAL: isize = 101;
pub const BINARY_LITERAL: isize = 102;
pub const OCTAL_LITERAL: isize = 103;
pub const HEXADECIMAL_LITERAL: isize = 104;
pub const DOUBLE_LITERAL: isize = 105;
pub const FLOAT_LITERAL: isize = 106;
pub const DECIMAL_LITERAL: isize = 107;
pub const ID: isize = 108;
pub const INVALID_STRING_LITERAL: isize = 109;
pub const INVALID_TOKEN: isize = 110;
pub const RSHIFT: isize = 111;
pub const RULE_packageDeclaration: usize = 0;
pub const RULE_packageNameDefinition: usize = 1;
pub const RULE_importDeclaration: usize = 2;
pub const RULE_languageDirective: usize = 3;
pub const RULE_typeDeclaration: usize = 4;
pub const RULE_symbolDefinition: usize = 5;
pub const RULE_constDefinition: usize = 6;
pub const RULE_ruleGroupDefinition: usize = 7;
pub const RULE_ruleDefinition: usize = 8;
pub const RULE_subtypeDeclaration: usize = 9;
pub const RULE_structureDeclaration: usize = 10;
pub const RULE_structureFieldDefinition: usize = 11;
pub const RULE_fieldAlignment: usize = 12;
pub const RULE_fieldOffset: usize = 13;
pub const RULE_fieldTypeId: usize = 14;
pub const RULE_fieldArrayRange: usize = 15;
pub const RULE_fieldInitializer: usize = 16;
pub const RULE_fieldOptionalClause: usize = 17;
pub const RULE_fieldConstraint: usize = 18;
pub const RULE_choiceDeclaration: usize = 19;
pub const RULE_choiceCases: usize = 20;
pub const RULE_choiceCase: usize = 21;
pub const RULE_choiceDefault: usize = 22;
pub const RULE_choiceFieldDefinition: usize = 23;
pub const RULE_unionDeclaration: usize = 24;
pub const RULE_unionFieldDefinition: usize = 25;
pub const RULE_enumDeclaration: usize = 26;
pub const RULE_enumItem: usize = 27;
pub const RULE_bitmaskDeclaration: usize = 28;
pub const RULE_bitmaskValue: usize = 29;
pub const RULE_sqlTableDeclaration: usize = 30;
pub const RULE_sqlTableFieldDefinition: usize = 31;
pub const RULE_sqlConstraintDefinition: usize = 32;
pub const RULE_sqlConstraint: usize = 33;
pub const RULE_sqlWithoutRowId: usize = 34;
pub const RULE_sqlDatabaseDefinition: usize = 35;
pub const RULE_sqlDatabaseFieldDefinition: usize = 36;
pub const RULE_serviceDefinition: usize = 37;
pub const RULE_serviceMethodDefinition: usize = 38;
pub const RULE_pubsubDefinition: usize = 39;
pub const RULE_pubsubMessageDefinition: usize = 40;
pub const RULE_topicDefinition: usize = 41;
pub const RULE_functionDefinition: usize = 42;
pub const RULE_functionType: usize = 43;
pub const RULE_functionName: usize = 44;
pub const RULE_functionBody: usize = 45;
pub const RULE_typeParameters: usize = 46;
pub const RULE_parameterDefinition: usize = 47;
pub const RULE_templateParameters: usize = 48;
pub const RULE_templateArguments: usize = 49;
pub const RULE_templateArgument: usize = 50;
pub const RULE_instantiateDeclaration: usize = 51;
pub const RULE_expression: usize = 52;
pub const RULE_literal: usize = 53;
pub const RULE_id: usize = 54;
pub const RULE_typeReference: usize = 55;
pub const RULE_typeInstantiation: usize = 56;
pub const RULE_builtinType: usize = 57;
pub const RULE_qualifiedName: usize = 58;
pub const RULE_typeArguments: usize = 59;
pub const RULE_typeArgument: usize = 60;
pub const RULE_dynamicLengthArgument: usize = 61;
pub const RULE_intType: usize = 62;
pub const RULE_varintType: usize = 63;
pub const RULE_fixedBitFieldType: usize = 64;
pub const RULE_dynamicBitFieldType: usize = 65;
pub const RULE_boolType: usize = 66;
pub const RULE_stringType: usize = 67;
pub const RULE_floatType: usize = 68;
pub const RULE_externType: usize = 69;
pub const ruleNames: [&'static str; 70] = [
    "packageDeclaration",
    "packageNameDefinition",
    "importDeclaration",
    "languageDirective",
    "typeDeclaration",
    "symbolDefinition",
    "constDefinition",
    "ruleGroupDefinition",
    "ruleDefinition",
    "subtypeDeclaration",
    "structureDeclaration",
    "structureFieldDefinition",
    "fieldAlignment",
    "fieldOffset",
    "fieldTypeId",
    "fieldArrayRange",
    "fieldInitializer",
    "fieldOptionalClause",
    "fieldConstraint",
    "choiceDeclaration",
    "choiceCases",
    "choiceCase",
    "choiceDefault",
    "choiceFieldDefinition",
    "unionDeclaration",
    "unionFieldDefinition",
    "enumDeclaration",
    "enumItem",
    "bitmaskDeclaration",
    "bitmaskValue",
    "sqlTableDeclaration",
    "sqlTableFieldDefinition",
    "sqlConstraintDefinition",
    "sqlConstraint",
    "sqlWithoutRowId",
    "sqlDatabaseDefinition",
    "sqlDatabaseFieldDefinition",
    "serviceDefinition",
    "serviceMethodDefinition",
    "pubsubDefinition",
    "pubsubMessageDefinition",
    "topicDefinition",
    "functionDefinition",
    "functionType",
    "functionName",
    "functionBody",
    "typeParameters",
    "parameterDefinition",
    "templateParameters",
    "templateArguments",
    "templateArgument",
    "instantiateDeclaration",
    "expression",
    "literal",
    "id",
    "typeReference",
    "typeInstantiation",
    "builtinType",
    "qualifiedName",
    "typeArguments",
    "typeArgument",
    "dynamicLengthArgument",
    "intType",
    "varintType",
    "fixedBitFieldType",
    "dynamicBitFieldType",
    "boolType",
    "stringType",
    "floatType",
    "externType",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 95] = [
    None,
    Some("'&'"),
    Some("'='"),
    Some("'!'"),
    Some("':'"),
    Some("','"),
    Some("'/'"),
    Some("'.'"),
    Some("'=='"),
    Some("'>='"),
    Some("'>'"),
    Some("'{'"),
    Some("'['"),
    Some("'<='"),
    Some("'&&'"),
    Some("'||'"),
    Some("'('"),
    Some("'<<'"),
    Some("'<'"),
    Some("'-'"),
    Some("'%'"),
    Some("'*'"),
    Some("'!='"),
    Some("'|'"),
    Some("'+'"),
    Some("'?'"),
    Some("'}'"),
    Some("']'"),
    Some("')'"),
    Some("';'"),
    Some("'~'"),
    Some("'^'"),
    Some("'align'"),
    Some("'bit'"),
    Some("'bool'"),
    Some("'bitmask'"),
    Some("'case'"),
    Some("'choice'"),
    Some("'const'"),
    Some("'default'"),
    Some("'enum'"),
    Some("'explicit'"),
    Some("'extern'"),
    Some("'float16'"),
    Some("'float32'"),
    Some("'float64'"),
    Some("'function'"),
    Some("'if'"),
    Some("'implicit'"),
    Some("'import'"),
    Some("'@index'"),
    Some("'instantiate'"),
    Some("'int'"),
    Some("'int16'"),
    Some("'int32'"),
    Some("'int64'"),
    Some("'int8'"),
    Some("'lengthof'"),
    Some("'numbits'"),
    Some("'on'"),
    Some("'optional'"),
    Some("'package'"),
    Some("'packed'"),
    Some("'pubsub'"),
    Some("'publish'"),
    Some("'return'"),
    Some("'rule'"),
    Some("'rule_group'"),
    Some("'service'"),
    Some("'sql'"),
    Some("'sql_database'"),
    Some("'sql_table'"),
    Some("'sql_virtual'"),
    Some("'sql_without_rowid'"),
    Some("'string'"),
    Some("'struct'"),
    Some("'subscribe'"),
    Some("'subtype'"),
    Some("'topic'"),
    Some("'uint16'"),
    Some("'uint32'"),
    Some("'uint64'"),
    Some("'uint8'"),
    Some("'union'"),
    Some("'using'"),
    Some("'valueof'"),
    Some("'varint'"),
    Some("'varint16'"),
    Some("'varint32'"),
    Some("'varint64'"),
    Some("'varsize'"),
    Some("'varuint'"),
    Some("'varuint16'"),
    Some("'varuint32'"),
    Some("'varuint64'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 112] = [
    None,
    Some("AND"),
    Some("ASSIGN"),
    Some("BANG"),
    Some("COLON"),
    Some("COMMA"),
    Some("DIVIDE"),
    Some("DOT"),
    Some("EQ"),
    Some("GE"),
    Some("GT"),
    Some("LBRACE"),
    Some("LBRACKET"),
    Some("LE"),
    Some("LOGICAL_AND"),
    Some("LOGICAL_OR"),
    Some("LPAREN"),
    Some("LSHIFT"),
    Some("LT"),
    Some("MINUS"),
    Some("MODULO"),
    Some("MULTIPLY"),
    Some("NE"),
    Some("OR"),
    Some("PLUS"),
    Some("QUESTIONMARK"),
    Some("RBRACE"),
    Some("RBRACKET"),
    Some("RPAREN"),
    Some("SEMICOLON"),
    Some("TILDE"),
    Some("XOR"),
    Some("ALIGN"),
    Some("BIT_FIELD"),
    Some("BOOL"),
    Some("BITMASK"),
    Some("CASE"),
    Some("CHOICE"),
    Some("CONST"),
    Some("DEFAULT"),
    Some("ENUM"),
    Some("EXPLICIT"),
    Some("EXTERN"),
    Some("FLOAT16"),
    Some("FLOAT32"),
    Some("FLOAT64"),
    Some("FUNCTION"),
    Some("IF"),
    Some("IMPLICIT"),
    Some("IMPORT"),
    Some("INDEX"),
    Some("INSTANTIATE"),
    Some("INT_FIELD"),
    Some("INT16"),
    Some("INT32"),
    Some("INT64"),
    Some("INT8"),
    Some("LENGTHOF"),
    Some("NUMBITS"),
    Some("ON"),
    Some("OPTIONAL"),
    Some("PACKAGE"),
    Some("PACKED"),
    Some("PUBSUB"),
    Some("PUBLISH"),
    Some("RETURN"),
    Some("RULE"),
    Some("RULE_GROUP"),
    Some("SERVICE"),
    Some("SQL"),
    Some("SQL_DATABASE"),
    Some("SQL_TABLE"),
    Some("SQL_VIRTUAL"),
    Some("SQL_WITHOUT_ROWID"),
    Some("STRING"),
    Some("STRUCTURE"),
    Some("SUBSCRIBE"),
    Some("SUBTYPE"),
    Some("TOPIC"),
    Some("UINT16"),
    Some("UINT32"),
    Some("UINT64"),
    Some("UINT8"),
    Some("UNION"),
    Some("USING"),
    Some("VALUEOF"),
    Some("VARINT"),
    Some("VARINT16"),
    Some("VARINT32"),
    Some("VARINT64"),
    Some("VARSIZE"),
    Some("VARUINT"),
    Some("VARUINT16"),
    Some("VARUINT32"),
    Some("VARUINT64"),
    Some("WS"),
    Some("DOC_COMMENT"),
    Some("MARKDOWN_COMMENT"),
    Some("BLOCK_COMMENT"),
    Some("LINE_COMMENT"),
    Some("BOOL_LITERAL"),
    Some("STRING_LITERAL"),
    Some("BINARY_LITERAL"),
    Some("OCTAL_LITERAL"),
    Some("HEXADECIMAL_LITERAL"),
    Some("DOUBLE_LITERAL"),
    Some("FLOAT_LITERAL"),
    Some("DECIMAL_LITERAL"),
    Some("ID"),
    Some("INVALID_STRING_LITERAL"),
    Some("INVALID_TOKEN"),
    Some("RSHIFT"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    ZserioParserExt<'input>,
    I,
    ZserioParserContextType,
    dyn ZserioParserListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type ZserioParserTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, ZserioParserContextType, dyn ZserioParserListener<'input> + 'a>;

/// Parser for ZserioParser grammar
pub struct ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                ZserioParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> ZserioParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> ZserioParser<'input, I, DefaultErrorStrategy<'input, ZserioParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for ZserioParser
pub trait ZserioParserContext<'input>:
    for<'x> Listenable<dyn ZserioParserListener<'input> + 'x>
    + for<'x> Visitable<dyn ZserioParserVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = ZserioParserContextType>
{
}

antlr_rust::coerce_from! { 'input : ZserioParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn ZserioParserContext<'input> + 'input
where
    T: ZserioParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn ZserioParserVisitor<'input> + 'x))
    }
}

impl<'input> ZserioParserContext<'input> for TerminalNode<'input, ZserioParserContextType> {}
impl<'input> ZserioParserContext<'input> for ErrorNode<'input, ZserioParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ZserioParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ZserioParserListener<'input> + 'input }

pub struct ZserioParserContextType;
antlr_rust::tid! {ZserioParserContextType}

impl<'input> ParserNodeType<'input> for ZserioParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn ZserioParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct ZserioParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> ZserioParserExt<'input> {}
antlr_rust::tid! { ZserioParserExt<'a> }

impl<'input> TokenAware<'input> for ZserioParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for ZserioParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for ZserioParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "ZserioParser.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn ZserioParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            52 => ZserioParser::<'input, I, _>::expression_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> ZserioParser<'input, I, DefaultErrorStrategy<'input, ZserioParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn expression_sempred(
        _localctx: Option<&ExpressionContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 14),
            1 => recog.precpred(None, 13),
            2 => recog.precpred(None, 12),
            3 => recog.precpred(None, 11),
            4 => recog.precpred(None, 10),
            5 => recog.precpred(None, 9),
            6 => recog.precpred(None, 8),
            7 => recog.precpred(None, 7),
            8 => recog.precpred(None, 6),
            9 => recog.precpred(None, 5),
            10 => recog.precpred(None, 4),
            11 => recog.precpred(None, 21),
            12 => recog.precpred(None, 20),
            13 => recog.precpred(None, 19),
            _ => true,
        }
    }
}
//------------------- packageDeclaration ----------------
pub type PackageDeclarationContextAll<'input> = PackageDeclarationContext<'input>;

pub type PackageDeclarationContext<'input> =
    BaseParserRuleContext<'input, PackageDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct PackageDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for PackageDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for PackageDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_packageDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_packageDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for PackageDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_packageDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for PackageDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_packageDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_packageDeclaration }
}
antlr_rust::tid! {PackageDeclarationContextExt<'a>}

impl<'input> PackageDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PackageDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PackageDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait PackageDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<PackageDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    fn packageNameDefinition(&self) -> Option<Rc<PackageNameDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn importDeclaration_all(&self) -> Vec<Rc<ImportDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn importDeclaration(&self, i: usize) -> Option<Rc<ImportDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn languageDirective_all(&self) -> Vec<Rc<LanguageDirectiveContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn languageDirective(&self, i: usize) -> Option<Rc<LanguageDirectiveContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> PackageDeclarationContextAttrs<'input> for PackageDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn packageDeclaration(
        &mut self,
    ) -> Result<Rc<PackageDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            PackageDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 0, RULE_packageDeclaration);
        let mut _localctx: Rc<PackageDeclarationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(141);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == PACKAGE {
                    {
                        /*InvokeRule packageNameDefinition*/
                        recog.base.set_state(140);
                        recog.packageNameDefinition()?;
                    }
                }

                recog.base.set_state(146);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == IMPORT {
                    {
                        {
                            /*InvokeRule importDeclaration*/
                            recog.base.set_state(143);
                            recog.importDeclaration()?;
                        }
                    }
                    recog.base.set_state(148);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(152);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 35) & !0x3f) == 0
                    && ((1usize << (_la - 35))
                        & ((1usize << (BITMASK - 35))
                            | (1usize << (CHOICE - 35))
                            | (1usize << (CONST - 35))
                            | (1usize << (ENUM - 35))
                            | (1usize << (INSTANTIATE - 35))
                            | (1usize << (PUBSUB - 35))))
                        != 0)
                    || (((_la - 67) & !0x3f) == 0
                        && ((1usize << (_la - 67))
                            & ((1usize << (RULE_GROUP - 67))
                                | (1usize << (SERVICE - 67))
                                | (1usize << (SQL_DATABASE - 67))
                                | (1usize << (SQL_TABLE - 67))
                                | (1usize << (STRUCTURE - 67))
                                | (1usize << (SUBTYPE - 67))
                                | (1usize << (UNION - 67))))
                            != 0)
                {
                    {
                        {
                            /*InvokeRule languageDirective*/
                            recog.base.set_state(149);
                            recog.languageDirective()?;
                        }
                    }
                    recog.base.set_state(154);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(155);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- packageNameDefinition ----------------
pub type PackageNameDefinitionContextAll<'input> = PackageNameDefinitionContext<'input>;

pub type PackageNameDefinitionContext<'input> =
    BaseParserRuleContext<'input, PackageNameDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct PackageNameDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for PackageNameDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for PackageNameDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_packageNameDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_packageNameDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for PackageNameDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_packageNameDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for PackageNameDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_packageNameDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_packageNameDefinition }
}
antlr_rust::tid! {PackageNameDefinitionContextExt<'a>}

impl<'input> PackageNameDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PackageNameDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PackageNameDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait PackageNameDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<PackageNameDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PACKAGE
    /// Returns `None` if there is no child corresponding to token PACKAGE
    fn PACKAGE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PACKAGE, 0)
    }
    fn id_all(&self) -> Vec<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(DOT)
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
}

impl<'input> PackageNameDefinitionContextAttrs<'input> for PackageNameDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn packageNameDefinition(
        &mut self,
    ) -> Result<Rc<PackageNameDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            PackageNameDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 2, RULE_packageNameDefinition);
        let mut _localctx: Rc<PackageNameDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(157);
                recog.base.match_token(PACKAGE, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(158);
                recog.id()?;

                recog.base.set_state(163);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == DOT {
                    {
                        {
                            recog.base.set_state(159);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;

                            /*InvokeRule id*/
                            recog.base.set_state(160);
                            recog.id()?;
                        }
                    }
                    recog.base.set_state(165);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(166);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- importDeclaration ----------------
pub type ImportDeclarationContextAll<'input> = ImportDeclarationContext<'input>;

pub type ImportDeclarationContext<'input> =
    BaseParserRuleContext<'input, ImportDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ImportDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ImportDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ImportDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_importDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_importDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ImportDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_importDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for ImportDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_importDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_importDeclaration }
}
antlr_rust::tid! {ImportDeclarationContextExt<'a>}

impl<'input> ImportDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ImportDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ImportDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait ImportDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ImportDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IMPORT
    /// Returns `None` if there is no child corresponding to token IMPORT
    fn IMPORT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IMPORT, 0)
    }
    fn id_all(&self) -> Vec<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(DOT)
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MULTIPLY
    /// Returns `None` if there is no child corresponding to token MULTIPLY
    fn MULTIPLY(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MULTIPLY, 0)
    }
}

impl<'input> ImportDeclarationContextAttrs<'input> for ImportDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn importDeclaration(
        &mut self,
    ) -> Result<Rc<ImportDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ImportDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 4, RULE_importDeclaration);
        let mut _localctx: Rc<ImportDeclarationContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(168);
                recog.base.match_token(IMPORT, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(169);
                recog.id()?;

                recog.base.set_state(170);
                recog.base.match_token(DOT, &mut recog.err_handler)?;

                recog.base.set_state(176);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule id*/
                                recog.base.set_state(171);
                                recog.id()?;

                                recog.base.set_state(172);
                                recog.base.match_token(DOT, &mut recog.err_handler)?;
                            }
                        }
                    }
                    recog.base.set_state(178);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                }
                recog.base.set_state(181);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    ID => {
                        {
                            /*InvokeRule id*/
                            recog.base.set_state(179);
                            recog.id()?;
                        }
                    }

                    MULTIPLY => {
                        recog.base.set_state(180);
                        recog.base.match_token(MULTIPLY, &mut recog.err_handler)?;
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(183);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- languageDirective ----------------
pub type LanguageDirectiveContextAll<'input> = LanguageDirectiveContext<'input>;

pub type LanguageDirectiveContext<'input> =
    BaseParserRuleContext<'input, LanguageDirectiveContextExt<'input>>;

#[derive(Clone)]
pub struct LanguageDirectiveContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for LanguageDirectiveContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for LanguageDirectiveContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_languageDirective(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_languageDirective(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for LanguageDirectiveContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_languageDirective(self);
    }
}

impl<'input> CustomRuleContext<'input> for LanguageDirectiveContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_languageDirective
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_languageDirective }
}
antlr_rust::tid! {LanguageDirectiveContextExt<'a>}

impl<'input> LanguageDirectiveContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<LanguageDirectiveContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            LanguageDirectiveContextExt { ph: PhantomData },
        ))
    }
}

pub trait LanguageDirectiveContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<LanguageDirectiveContextExt<'input>>
{
    fn symbolDefinition(&self) -> Option<Rc<SymbolDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn typeDeclaration(&self) -> Option<Rc<TypeDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> LanguageDirectiveContextAttrs<'input> for LanguageDirectiveContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn languageDirective(
        &mut self,
    ) -> Result<Rc<LanguageDirectiveContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            LanguageDirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 6, RULE_languageDirective);
        let mut _localctx: Rc<LanguageDirectiveContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(187);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                CONST | RULE_GROUP => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule symbolDefinition*/
                        recog.base.set_state(185);
                        recog.symbolDefinition()?;
                    }
                }

                BITMASK | CHOICE | ENUM | INSTANTIATE | PUBSUB | SERVICE | SQL_DATABASE
                | SQL_TABLE | STRUCTURE | SUBTYPE | UNION => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule typeDeclaration*/
                        recog.base.set_state(186);
                        recog.typeDeclaration()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- typeDeclaration ----------------
pub type TypeDeclarationContextAll<'input> = TypeDeclarationContext<'input>;

pub type TypeDeclarationContext<'input> =
    BaseParserRuleContext<'input, TypeDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct TypeDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TypeDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TypeDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_typeDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_typeDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for TypeDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_typeDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for TypeDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_typeDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_typeDeclaration }
}
antlr_rust::tid! {TypeDeclarationContextExt<'a>}

impl<'input> TypeDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TypeDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TypeDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait TypeDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TypeDeclarationContextExt<'input>>
{
    fn subtypeDeclaration(&self) -> Option<Rc<SubtypeDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn structureDeclaration(&self) -> Option<Rc<StructureDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn choiceDeclaration(&self) -> Option<Rc<ChoiceDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn unionDeclaration(&self) -> Option<Rc<UnionDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn enumDeclaration(&self) -> Option<Rc<EnumDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn bitmaskDeclaration(&self) -> Option<Rc<BitmaskDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sqlTableDeclaration(&self) -> Option<Rc<SqlTableDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sqlDatabaseDefinition(&self) -> Option<Rc<SqlDatabaseDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn serviceDefinition(&self) -> Option<Rc<ServiceDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn pubsubDefinition(&self) -> Option<Rc<PubsubDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn instantiateDeclaration(&self) -> Option<Rc<InstantiateDeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TypeDeclarationContextAttrs<'input> for TypeDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn typeDeclaration(&mut self) -> Result<Rc<TypeDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TypeDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 8, RULE_typeDeclaration);
        let mut _localctx: Rc<TypeDeclarationContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(200);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SUBTYPE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule subtypeDeclaration*/
                        recog.base.set_state(189);
                        recog.subtypeDeclaration()?;
                    }
                }

                STRUCTURE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule structureDeclaration*/
                        recog.base.set_state(190);
                        recog.structureDeclaration()?;
                    }
                }

                CHOICE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule choiceDeclaration*/
                        recog.base.set_state(191);
                        recog.choiceDeclaration()?;
                    }
                }

                UNION => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule unionDeclaration*/
                        recog.base.set_state(192);
                        recog.unionDeclaration()?;
                    }
                }

                ENUM => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule enumDeclaration*/
                        recog.base.set_state(193);
                        recog.enumDeclaration()?;
                    }
                }

                BITMASK => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule bitmaskDeclaration*/
                        recog.base.set_state(194);
                        recog.bitmaskDeclaration()?;
                    }
                }

                SQL_TABLE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule sqlTableDeclaration*/
                        recog.base.set_state(195);
                        recog.sqlTableDeclaration()?;
                    }
                }

                SQL_DATABASE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule sqlDatabaseDefinition*/
                        recog.base.set_state(196);
                        recog.sqlDatabaseDefinition()?;
                    }
                }

                SERVICE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        /*InvokeRule serviceDefinition*/
                        recog.base.set_state(197);
                        recog.serviceDefinition()?;
                    }
                }

                PUBSUB => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        /*InvokeRule pubsubDefinition*/
                        recog.base.set_state(198);
                        recog.pubsubDefinition()?;
                    }
                }

                INSTANTIATE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 11);
                    recog.base.enter_outer_alt(None, 11);
                    {
                        /*InvokeRule instantiateDeclaration*/
                        recog.base.set_state(199);
                        recog.instantiateDeclaration()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- symbolDefinition ----------------
pub type SymbolDefinitionContextAll<'input> = SymbolDefinitionContext<'input>;

pub type SymbolDefinitionContext<'input> =
    BaseParserRuleContext<'input, SymbolDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct SymbolDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SymbolDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SymbolDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_symbolDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_symbolDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for SymbolDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_symbolDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for SymbolDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_symbolDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_symbolDefinition }
}
antlr_rust::tid! {SymbolDefinitionContextExt<'a>}

impl<'input> SymbolDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SymbolDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SymbolDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait SymbolDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SymbolDefinitionContextExt<'input>>
{
    fn constDefinition(&self) -> Option<Rc<ConstDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn ruleGroupDefinition(&self) -> Option<Rc<RuleGroupDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> SymbolDefinitionContextAttrs<'input> for SymbolDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn symbolDefinition(
        &mut self,
    ) -> Result<Rc<SymbolDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SymbolDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_symbolDefinition);
        let mut _localctx: Rc<SymbolDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(204);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                CONST => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule constDefinition*/
                        recog.base.set_state(202);
                        recog.constDefinition()?;
                    }
                }

                RULE_GROUP => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule ruleGroupDefinition*/
                        recog.base.set_state(203);
                        recog.ruleGroupDefinition()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- constDefinition ----------------
pub type ConstDefinitionContextAll<'input> = ConstDefinitionContext<'input>;

pub type ConstDefinitionContext<'input> =
    BaseParserRuleContext<'input, ConstDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ConstDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ConstDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ConstDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_constDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_constDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ConstDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_constDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for ConstDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constDefinition }
}
antlr_rust::tid! {ConstDefinitionContextExt<'a>}

impl<'input> ConstDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ConstDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ConstDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ConstDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ConstDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CONST
    /// Returns `None` if there is no child corresponding to token CONST
    fn CONST(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CONST, 0)
    }
    fn typeInstantiation(&self) -> Option<Rc<TypeInstantiationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> ConstDefinitionContextAttrs<'input> for ConstDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn constDefinition(&mut self) -> Result<Rc<ConstDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ConstDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_constDefinition);
        let mut _localctx: Rc<ConstDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(206);
                recog.base.match_token(CONST, &mut recog.err_handler)?;

                /*InvokeRule typeInstantiation*/
                recog.base.set_state(207);
                recog.typeInstantiation()?;

                /*InvokeRule id*/
                recog.base.set_state(208);
                recog.id()?;

                recog.base.set_state(209);
                recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(210);
                recog.expression_rec(0)?;

                recog.base.set_state(211);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- ruleGroupDefinition ----------------
pub type RuleGroupDefinitionContextAll<'input> = RuleGroupDefinitionContext<'input>;

pub type RuleGroupDefinitionContext<'input> =
    BaseParserRuleContext<'input, RuleGroupDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct RuleGroupDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for RuleGroupDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for RuleGroupDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ruleGroupDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_ruleGroupDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for RuleGroupDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_ruleGroupDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for RuleGroupDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_ruleGroupDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_ruleGroupDefinition }
}
antlr_rust::tid! {RuleGroupDefinitionContextExt<'a>}

impl<'input> RuleGroupDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<RuleGroupDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            RuleGroupDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait RuleGroupDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<RuleGroupDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token RULE_GROUP
    /// Returns `None` if there is no child corresponding to token RULE_GROUP
    fn RULE_GROUP(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RULE_GROUP, 0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn ruleDefinition_all(&self) -> Vec<Rc<RuleDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn ruleDefinition(&self, i: usize) -> Option<Rc<RuleDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> RuleGroupDefinitionContextAttrs<'input> for RuleGroupDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn ruleGroupDefinition(
        &mut self,
    ) -> Result<Rc<RuleGroupDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            RuleGroupDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_ruleGroupDefinition);
        let mut _localctx: Rc<RuleGroupDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(213);
                recog.base.match_token(RULE_GROUP, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(214);
                recog.id()?;

                recog.base.set_state(215);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(219);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == RULE {
                    {
                        {
                            /*InvokeRule ruleDefinition*/
                            recog.base.set_state(216);
                            recog.ruleDefinition()?;
                        }
                    }
                    recog.base.set_state(221);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(222);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(223);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- ruleDefinition ----------------
pub type RuleDefinitionContextAll<'input> = RuleDefinitionContext<'input>;

pub type RuleDefinitionContext<'input> =
    BaseParserRuleContext<'input, RuleDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct RuleDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for RuleDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for RuleDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ruleDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_ruleDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for RuleDefinitionContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_ruleDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for RuleDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_ruleDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_ruleDefinition }
}
antlr_rust::tid! {RuleDefinitionContextExt<'a>}

impl<'input> RuleDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<RuleDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            RuleDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait RuleDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<RuleDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token RULE
    /// Returns `None` if there is no child corresponding to token RULE
    fn RULE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RULE, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> RuleDefinitionContextAttrs<'input> for RuleDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn ruleDefinition(&mut self) -> Result<Rc<RuleDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            RuleDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 16, RULE_ruleDefinition);
        let mut _localctx: Rc<RuleDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(225);
                recog.base.match_token(RULE, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(226);
                recog.expression_rec(0)?;

                recog.base.set_state(227);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- subtypeDeclaration ----------------
pub type SubtypeDeclarationContextAll<'input> = SubtypeDeclarationContext<'input>;

pub type SubtypeDeclarationContext<'input> =
    BaseParserRuleContext<'input, SubtypeDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct SubtypeDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SubtypeDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SubtypeDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_subtypeDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_subtypeDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for SubtypeDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_subtypeDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for SubtypeDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_subtypeDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_subtypeDeclaration }
}
antlr_rust::tid! {SubtypeDeclarationContextExt<'a>}

impl<'input> SubtypeDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SubtypeDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SubtypeDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait SubtypeDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SubtypeDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SUBTYPE
    /// Returns `None` if there is no child corresponding to token SUBTYPE
    fn SUBTYPE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SUBTYPE, 0)
    }
    fn typeReference(&self) -> Option<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> SubtypeDeclarationContextAttrs<'input> for SubtypeDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn subtypeDeclaration(
        &mut self,
    ) -> Result<Rc<SubtypeDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SubtypeDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_subtypeDeclaration);
        let mut _localctx: Rc<SubtypeDeclarationContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(229);
                recog.base.match_token(SUBTYPE, &mut recog.err_handler)?;

                /*InvokeRule typeReference*/
                recog.base.set_state(230);
                recog.typeReference()?;

                /*InvokeRule id*/
                recog.base.set_state(231);
                recog.id()?;

                recog.base.set_state(232);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- structureDeclaration ----------------
pub type StructureDeclarationContextAll<'input> = StructureDeclarationContext<'input>;

pub type StructureDeclarationContext<'input> =
    BaseParserRuleContext<'input, StructureDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct StructureDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for StructureDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for StructureDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_structureDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_structureDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for StructureDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_structureDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for StructureDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_structureDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_structureDeclaration }
}
antlr_rust::tid! {StructureDeclarationContextExt<'a>}

impl<'input> StructureDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StructureDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StructureDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait StructureDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<StructureDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRUCTURE
    /// Returns `None` if there is no child corresponding to token STRUCTURE
    fn STRUCTURE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRUCTURE, 0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn templateParameters(&self) -> Option<Rc<TemplateParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn structureFieldDefinition_all(&self) -> Vec<Rc<StructureFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn structureFieldDefinition(
        &self,
        i: usize,
    ) -> Option<Rc<StructureFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn functionDefinition_all(&self) -> Vec<Rc<FunctionDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn functionDefinition(&self, i: usize) -> Option<Rc<FunctionDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> StructureDeclarationContextAttrs<'input> for StructureDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn structureDeclaration(
        &mut self,
    ) -> Result<Rc<StructureDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            StructureDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_structureDeclaration);
        let mut _localctx: Rc<StructureDeclarationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(234);
                recog.base.match_token(STRUCTURE, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(235);
                recog.id()?;

                recog.base.set_state(237);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == LT {
                    {
                        /*InvokeRule templateParameters*/
                        recog.base.set_state(236);
                        recog.templateParameters()?;
                    }
                }

                recog.base.set_state(240);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == LPAREN {
                    {
                        /*InvokeRule typeParameters*/
                        recog.base.set_state(239);
                        recog.typeParameters()?;
                    }
                }

                recog.base.set_state(242);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(246);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << BANG)
                            | (1usize << LPAREN)
                            | (1usize << MINUS)
                            | (1usize << PLUS)
                            | (1usize << TILDE)))
                        != 0)
                    || (((_la - 32) & !0x3f) == 0
                        && ((1usize << (_la - 32))
                            & ((1usize << (ALIGN - 32))
                                | (1usize << (BIT_FIELD - 32))
                                | (1usize << (BOOL - 32))
                                | (1usize << (EXTERN - 32))
                                | (1usize << (FLOAT16 - 32))
                                | (1usize << (FLOAT32 - 32))
                                | (1usize << (FLOAT64 - 32))
                                | (1usize << (IMPLICIT - 32))
                                | (1usize << (INDEX - 32))
                                | (1usize << (INT_FIELD - 32))
                                | (1usize << (INT16 - 32))
                                | (1usize << (INT32 - 32))
                                | (1usize << (INT64 - 32))
                                | (1usize << (INT8 - 32))
                                | (1usize << (LENGTHOF - 32))
                                | (1usize << (NUMBITS - 32))
                                | (1usize << (OPTIONAL - 32))
                                | (1usize << (PACKED - 32))))
                            != 0)
                    || (((_la - 74) & !0x3f) == 0
                        && ((1usize << (_la - 74))
                            & ((1usize << (STRING - 74))
                                | (1usize << (UINT16 - 74))
                                | (1usize << (UINT32 - 74))
                                | (1usize << (UINT64 - 74))
                                | (1usize << (UINT8 - 74))
                                | (1usize << (VALUEOF - 74))
                                | (1usize << (VARINT - 74))
                                | (1usize << (VARINT16 - 74))
                                | (1usize << (VARINT32 - 74))
                                | (1usize << (VARINT64 - 74))
                                | (1usize << (VARSIZE - 74))
                                | (1usize << (VARUINT - 74))
                                | (1usize << (VARUINT16 - 74))
                                | (1usize << (VARUINT32 - 74))
                                | (1usize << (VARUINT64 - 74))
                                | (1usize << (BOOL_LITERAL - 74))
                                | (1usize << (STRING_LITERAL - 74))
                                | (1usize << (BINARY_LITERAL - 74))
                                | (1usize << (OCTAL_LITERAL - 74))
                                | (1usize << (HEXADECIMAL_LITERAL - 74))
                                | (1usize << (DOUBLE_LITERAL - 74))))
                            != 0)
                    || (((_la - 106) & !0x3f) == 0
                        && ((1usize << (_la - 106))
                            & ((1usize << (FLOAT_LITERAL - 106))
                                | (1usize << (DECIMAL_LITERAL - 106))
                                | (1usize << (ID - 106))))
                            != 0)
                {
                    {
                        {
                            /*InvokeRule structureFieldDefinition*/
                            recog.base.set_state(243);
                            recog.structureFieldDefinition()?;
                        }
                    }
                    recog.base.set_state(248);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(252);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == FUNCTION {
                    {
                        {
                            /*InvokeRule functionDefinition*/
                            recog.base.set_state(249);
                            recog.functionDefinition()?;
                        }
                    }
                    recog.base.set_state(254);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(255);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(256);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- structureFieldDefinition ----------------
pub type StructureFieldDefinitionContextAll<'input> = StructureFieldDefinitionContext<'input>;

pub type StructureFieldDefinitionContext<'input> =
    BaseParserRuleContext<'input, StructureFieldDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct StructureFieldDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for StructureFieldDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for StructureFieldDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_structureFieldDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_structureFieldDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for StructureFieldDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_structureFieldDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for StructureFieldDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_structureFieldDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_structureFieldDefinition }
}
antlr_rust::tid! {StructureFieldDefinitionContextExt<'a>}

impl<'input> StructureFieldDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StructureFieldDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StructureFieldDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait StructureFieldDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<StructureFieldDefinitionContextExt<'input>>
{
    fn fieldTypeId(&self) -> Option<Rc<FieldTypeIdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn fieldAlignment(&self) -> Option<Rc<FieldAlignmentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn fieldOffset(&self) -> Option<Rc<FieldOffsetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPTIONAL
    /// Returns `None` if there is no child corresponding to token OPTIONAL
    fn OPTIONAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPTIONAL, 0)
    }
    fn fieldInitializer(&self) -> Option<Rc<FieldInitializerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn fieldOptionalClause(&self) -> Option<Rc<FieldOptionalClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn fieldConstraint(&self) -> Option<Rc<FieldConstraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> StructureFieldDefinitionContextAttrs<'input>
    for StructureFieldDefinitionContext<'input>
{
}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn structureFieldDefinition(
        &mut self,
    ) -> Result<Rc<StructureFieldDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            StructureFieldDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_structureFieldDefinition);
        let mut _localctx: Rc<StructureFieldDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(259);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ALIGN {
                    {
                        /*InvokeRule fieldAlignment*/
                        recog.base.set_state(258);
                        recog.fieldAlignment()?;
                    }
                }

                recog.base.set_state(262);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(15, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule fieldOffset*/
                            recog.base.set_state(261);
                            recog.fieldOffset()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(265);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OPTIONAL {
                    {
                        recog.base.set_state(264);
                        recog.base.match_token(OPTIONAL, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule fieldTypeId*/
                recog.base.set_state(267);
                recog.fieldTypeId()?;

                recog.base.set_state(269);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ASSIGN {
                    {
                        /*InvokeRule fieldInitializer*/
                        recog.base.set_state(268);
                        recog.fieldInitializer()?;
                    }
                }

                recog.base.set_state(272);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == IF {
                    {
                        /*InvokeRule fieldOptionalClause*/
                        recog.base.set_state(271);
                        recog.fieldOptionalClause()?;
                    }
                }

                recog.base.set_state(275);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COLON {
                    {
                        /*InvokeRule fieldConstraint*/
                        recog.base.set_state(274);
                        recog.fieldConstraint()?;
                    }
                }

                recog.base.set_state(277);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fieldAlignment ----------------
pub type FieldAlignmentContextAll<'input> = FieldAlignmentContext<'input>;

pub type FieldAlignmentContext<'input> =
    BaseParserRuleContext<'input, FieldAlignmentContextExt<'input>>;

#[derive(Clone)]
pub struct FieldAlignmentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FieldAlignmentContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for FieldAlignmentContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fieldAlignment(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_fieldAlignment(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for FieldAlignmentContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_fieldAlignment(self);
    }
}

impl<'input> CustomRuleContext<'input> for FieldAlignmentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fieldAlignment
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fieldAlignment }
}
antlr_rust::tid! {FieldAlignmentContextExt<'a>}

impl<'input> FieldAlignmentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldAlignmentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FieldAlignmentContextExt { ph: PhantomData },
        ))
    }
}

pub trait FieldAlignmentContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FieldAlignmentContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ALIGN
    /// Returns `None` if there is no child corresponding to token ALIGN
    fn ALIGN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ALIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
}

impl<'input> FieldAlignmentContextAttrs<'input> for FieldAlignmentContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fieldAlignment(&mut self) -> Result<Rc<FieldAlignmentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            FieldAlignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 24, RULE_fieldAlignment);
        let mut _localctx: Rc<FieldAlignmentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(279);
                recog.base.match_token(ALIGN, &mut recog.err_handler)?;

                recog.base.set_state(280);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(281);
                recog.expression_rec(0)?;

                recog.base.set_state(282);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;

                recog.base.set_state(283);
                recog.base.match_token(COLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fieldOffset ----------------
pub type FieldOffsetContextAll<'input> = FieldOffsetContext<'input>;

pub type FieldOffsetContext<'input> = BaseParserRuleContext<'input, FieldOffsetContextExt<'input>>;

#[derive(Clone)]
pub struct FieldOffsetContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FieldOffsetContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for FieldOffsetContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fieldOffset(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_fieldOffset(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for FieldOffsetContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_fieldOffset(self);
    }
}

impl<'input> CustomRuleContext<'input> for FieldOffsetContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fieldOffset
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fieldOffset }
}
antlr_rust::tid! {FieldOffsetContextExt<'a>}

impl<'input> FieldOffsetContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldOffsetContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FieldOffsetContextExt { ph: PhantomData },
        ))
    }
}

pub trait FieldOffsetContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FieldOffsetContextExt<'input>>
{
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
}

impl<'input> FieldOffsetContextAttrs<'input> for FieldOffsetContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fieldOffset(&mut self) -> Result<Rc<FieldOffsetContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FieldOffsetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 26, RULE_fieldOffset);
        let mut _localctx: Rc<FieldOffsetContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expression*/
                recog.base.set_state(285);
                recog.expression_rec(0)?;

                recog.base.set_state(286);
                recog.base.match_token(COLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fieldTypeId ----------------
pub type FieldTypeIdContextAll<'input> = FieldTypeIdContext<'input>;

pub type FieldTypeIdContext<'input> = BaseParserRuleContext<'input, FieldTypeIdContextExt<'input>>;

#[derive(Clone)]
pub struct FieldTypeIdContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FieldTypeIdContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for FieldTypeIdContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fieldTypeId(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_fieldTypeId(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for FieldTypeIdContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_fieldTypeId(self);
    }
}

impl<'input> CustomRuleContext<'input> for FieldTypeIdContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fieldTypeId
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fieldTypeId }
}
antlr_rust::tid! {FieldTypeIdContextExt<'a>}

impl<'input> FieldTypeIdContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldTypeIdContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FieldTypeIdContextExt { ph: PhantomData },
        ))
    }
}

pub trait FieldTypeIdContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FieldTypeIdContextExt<'input>>
{
    fn typeInstantiation(&self) -> Option<Rc<TypeInstantiationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PACKED
    /// Returns `None` if there is no child corresponding to token PACKED
    fn PACKED(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PACKED, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IMPLICIT
    /// Returns `None` if there is no child corresponding to token IMPLICIT
    fn IMPLICIT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IMPLICIT, 0)
    }
    fn fieldArrayRange(&self) -> Option<Rc<FieldArrayRangeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FieldTypeIdContextAttrs<'input> for FieldTypeIdContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fieldTypeId(&mut self) -> Result<Rc<FieldTypeIdContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FieldTypeIdContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 28, RULE_fieldTypeId);
        let mut _localctx: Rc<FieldTypeIdContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(289);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == PACKED {
                    {
                        recog.base.set_state(288);
                        recog.base.match_token(PACKED, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(292);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == IMPLICIT {
                    {
                        recog.base.set_state(291);
                        recog.base.match_token(IMPLICIT, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule typeInstantiation*/
                recog.base.set_state(294);
                recog.typeInstantiation()?;

                /*InvokeRule id*/
                recog.base.set_state(295);
                recog.id()?;

                recog.base.set_state(297);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == LBRACKET {
                    {
                        /*InvokeRule fieldArrayRange*/
                        recog.base.set_state(296);
                        recog.fieldArrayRange()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fieldArrayRange ----------------
pub type FieldArrayRangeContextAll<'input> = FieldArrayRangeContext<'input>;

pub type FieldArrayRangeContext<'input> =
    BaseParserRuleContext<'input, FieldArrayRangeContextExt<'input>>;

#[derive(Clone)]
pub struct FieldArrayRangeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FieldArrayRangeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for FieldArrayRangeContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fieldArrayRange(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_fieldArrayRange(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for FieldArrayRangeContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_fieldArrayRange(self);
    }
}

impl<'input> CustomRuleContext<'input> for FieldArrayRangeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fieldArrayRange
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fieldArrayRange }
}
antlr_rust::tid! {FieldArrayRangeContextExt<'a>}

impl<'input> FieldArrayRangeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldArrayRangeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FieldArrayRangeContextExt { ph: PhantomData },
        ))
    }
}

pub trait FieldArrayRangeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FieldArrayRangeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LBRACKET
    /// Returns `None` if there is no child corresponding to token LBRACKET
    fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACKET, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACKET
    /// Returns `None` if there is no child corresponding to token RBRACKET
    fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACKET, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FieldArrayRangeContextAttrs<'input> for FieldArrayRangeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fieldArrayRange(&mut self) -> Result<Rc<FieldArrayRangeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            FieldArrayRangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 30, RULE_fieldArrayRange);
        let mut _localctx: Rc<FieldArrayRangeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(299);
                recog.base.match_token(LBRACKET, &mut recog.err_handler)?;

                recog.base.set_state(301);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << BANG)
                            | (1usize << LPAREN)
                            | (1usize << MINUS)
                            | (1usize << PLUS)
                            | (1usize << TILDE)))
                        != 0)
                    || (((_la - 50) & !0x3f) == 0
                        && ((1usize << (_la - 50))
                            & ((1usize << (INDEX - 50))
                                | (1usize << (LENGTHOF - 50))
                                | (1usize << (NUMBITS - 50))))
                            != 0)
                    || (((_la - 85) & !0x3f) == 0
                        && ((1usize << (_la - 85))
                            & ((1usize << (VALUEOF - 85))
                                | (1usize << (BOOL_LITERAL - 85))
                                | (1usize << (STRING_LITERAL - 85))
                                | (1usize << (BINARY_LITERAL - 85))
                                | (1usize << (OCTAL_LITERAL - 85))
                                | (1usize << (HEXADECIMAL_LITERAL - 85))
                                | (1usize << (DOUBLE_LITERAL - 85))
                                | (1usize << (FLOAT_LITERAL - 85))
                                | (1usize << (DECIMAL_LITERAL - 85))
                                | (1usize << (ID - 85))))
                            != 0)
                {
                    {
                        /*InvokeRule expression*/
                        recog.base.set_state(300);
                        recog.expression_rec(0)?;
                    }
                }

                recog.base.set_state(303);
                recog.base.match_token(RBRACKET, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fieldInitializer ----------------
pub type FieldInitializerContextAll<'input> = FieldInitializerContext<'input>;

pub type FieldInitializerContext<'input> =
    BaseParserRuleContext<'input, FieldInitializerContextExt<'input>>;

#[derive(Clone)]
pub struct FieldInitializerContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FieldInitializerContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for FieldInitializerContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fieldInitializer(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_fieldInitializer(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for FieldInitializerContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_fieldInitializer(self);
    }
}

impl<'input> CustomRuleContext<'input> for FieldInitializerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fieldInitializer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fieldInitializer }
}
antlr_rust::tid! {FieldInitializerContextExt<'a>}

impl<'input> FieldInitializerContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldInitializerContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FieldInitializerContextExt { ph: PhantomData },
        ))
    }
}

pub trait FieldInitializerContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FieldInitializerContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FieldInitializerContextAttrs<'input> for FieldInitializerContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fieldInitializer(
        &mut self,
    ) -> Result<Rc<FieldInitializerContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            FieldInitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 32, RULE_fieldInitializer);
        let mut _localctx: Rc<FieldInitializerContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(305);
                recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(306);
                recog.expression_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fieldOptionalClause ----------------
pub type FieldOptionalClauseContextAll<'input> = FieldOptionalClauseContext<'input>;

pub type FieldOptionalClauseContext<'input> =
    BaseParserRuleContext<'input, FieldOptionalClauseContextExt<'input>>;

#[derive(Clone)]
pub struct FieldOptionalClauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FieldOptionalClauseContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for FieldOptionalClauseContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fieldOptionalClause(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_fieldOptionalClause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for FieldOptionalClauseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_fieldOptionalClause(self);
    }
}

impl<'input> CustomRuleContext<'input> for FieldOptionalClauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fieldOptionalClause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fieldOptionalClause }
}
antlr_rust::tid! {FieldOptionalClauseContextExt<'a>}

impl<'input> FieldOptionalClauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldOptionalClauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FieldOptionalClauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait FieldOptionalClauseContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FieldOptionalClauseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IF
    /// Returns `None` if there is no child corresponding to token IF
    fn IF(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IF, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FieldOptionalClauseContextAttrs<'input> for FieldOptionalClauseContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fieldOptionalClause(
        &mut self,
    ) -> Result<Rc<FieldOptionalClauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            FieldOptionalClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_fieldOptionalClause);
        let mut _localctx: Rc<FieldOptionalClauseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(308);
                recog.base.match_token(IF, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(309);
                recog.expression_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fieldConstraint ----------------
pub type FieldConstraintContextAll<'input> = FieldConstraintContext<'input>;

pub type FieldConstraintContext<'input> =
    BaseParserRuleContext<'input, FieldConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct FieldConstraintContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FieldConstraintContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for FieldConstraintContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fieldConstraint(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_fieldConstraint(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for FieldConstraintContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_fieldConstraint(self);
    }
}

impl<'input> CustomRuleContext<'input> for FieldConstraintContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fieldConstraint
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fieldConstraint }
}
antlr_rust::tid! {FieldConstraintContextExt<'a>}

impl<'input> FieldConstraintContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldConstraintContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FieldConstraintContextExt { ph: PhantomData },
        ))
    }
}

pub trait FieldConstraintContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FieldConstraintContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FieldConstraintContextAttrs<'input> for FieldConstraintContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fieldConstraint(&mut self) -> Result<Rc<FieldConstraintContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            FieldConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 36, RULE_fieldConstraint);
        let mut _localctx: Rc<FieldConstraintContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(311);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(312);
                recog.expression_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- choiceDeclaration ----------------
pub type ChoiceDeclarationContextAll<'input> = ChoiceDeclarationContext<'input>;

pub type ChoiceDeclarationContext<'input> =
    BaseParserRuleContext<'input, ChoiceDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ChoiceDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ChoiceDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ChoiceDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_choiceDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_choiceDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ChoiceDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_choiceDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for ChoiceDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_choiceDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_choiceDeclaration }
}
antlr_rust::tid! {ChoiceDeclarationContextExt<'a>}

impl<'input> ChoiceDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ChoiceDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ChoiceDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait ChoiceDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ChoiceDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CHOICE
    /// Returns `None` if there is no child corresponding to token CHOICE
    fn CHOICE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CHOICE, 0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ON
    /// Returns `None` if there is no child corresponding to token ON
    fn ON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ON, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn templateParameters(&self) -> Option<Rc<TemplateParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn choiceCases_all(&self) -> Vec<Rc<ChoiceCasesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn choiceCases(&self, i: usize) -> Option<Rc<ChoiceCasesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn choiceDefault(&self) -> Option<Rc<ChoiceDefaultContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn functionDefinition_all(&self) -> Vec<Rc<FunctionDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn functionDefinition(&self, i: usize) -> Option<Rc<FunctionDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ChoiceDeclarationContextAttrs<'input> for ChoiceDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn choiceDeclaration(
        &mut self,
    ) -> Result<Rc<ChoiceDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ChoiceDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 38, RULE_choiceDeclaration);
        let mut _localctx: Rc<ChoiceDeclarationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(314);
                recog.base.match_token(CHOICE, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(315);
                recog.id()?;

                recog.base.set_state(317);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == LT {
                    {
                        /*InvokeRule templateParameters*/
                        recog.base.set_state(316);
                        recog.templateParameters()?;
                    }
                }

                /*InvokeRule typeParameters*/
                recog.base.set_state(319);
                recog.typeParameters()?;

                recog.base.set_state(320);
                recog.base.match_token(ON, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(321);
                recog.expression_rec(0)?;

                recog.base.set_state(322);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(326);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == CASE {
                    {
                        {
                            /*InvokeRule choiceCases*/
                            recog.base.set_state(323);
                            recog.choiceCases()?;
                        }
                    }
                    recog.base.set_state(328);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(330);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == DEFAULT {
                    {
                        /*InvokeRule choiceDefault*/
                        recog.base.set_state(329);
                        recog.choiceDefault()?;
                    }
                }

                recog.base.set_state(335);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == FUNCTION {
                    {
                        {
                            /*InvokeRule functionDefinition*/
                            recog.base.set_state(332);
                            recog.functionDefinition()?;
                        }
                    }
                    recog.base.set_state(337);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(338);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(339);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- choiceCases ----------------
pub type ChoiceCasesContextAll<'input> = ChoiceCasesContext<'input>;

pub type ChoiceCasesContext<'input> = BaseParserRuleContext<'input, ChoiceCasesContextExt<'input>>;

#[derive(Clone)]
pub struct ChoiceCasesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ChoiceCasesContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for ChoiceCasesContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_choiceCases(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_choiceCases(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for ChoiceCasesContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_choiceCases(self);
    }
}

impl<'input> CustomRuleContext<'input> for ChoiceCasesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_choiceCases
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_choiceCases }
}
antlr_rust::tid! {ChoiceCasesContextExt<'a>}

impl<'input> ChoiceCasesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ChoiceCasesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ChoiceCasesContextExt { ph: PhantomData },
        ))
    }
}

pub trait ChoiceCasesContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ChoiceCasesContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn choiceCase_all(&self) -> Vec<Rc<ChoiceCaseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn choiceCase(&self, i: usize) -> Option<Rc<ChoiceCaseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn choiceFieldDefinition(&self) -> Option<Rc<ChoiceFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ChoiceCasesContextAttrs<'input> for ChoiceCasesContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn choiceCases(&mut self) -> Result<Rc<ChoiceCasesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ChoiceCasesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 40, RULE_choiceCases);
        let mut _localctx: Rc<ChoiceCasesContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(342);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule choiceCase*/
                            recog.base.set_state(341);
                            recog.choiceCase()?;
                        }
                    }
                    recog.base.set_state(344);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == CASE) {
                        break;
                    }
                }
                recog.base.set_state(347);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la - 33) & !0x3f) == 0
                    && ((1usize << (_la - 33))
                        & ((1usize << (BIT_FIELD - 33))
                            | (1usize << (BOOL - 33))
                            | (1usize << (EXTERN - 33))
                            | (1usize << (FLOAT16 - 33))
                            | (1usize << (FLOAT32 - 33))
                            | (1usize << (FLOAT64 - 33))
                            | (1usize << (IMPLICIT - 33))
                            | (1usize << (INT_FIELD - 33))
                            | (1usize << (INT16 - 33))
                            | (1usize << (INT32 - 33))
                            | (1usize << (INT64 - 33))
                            | (1usize << (INT8 - 33))
                            | (1usize << (PACKED - 33))))
                        != 0)
                    || (((_la - 74) & !0x3f) == 0
                        && ((1usize << (_la - 74))
                            & ((1usize << (STRING - 74))
                                | (1usize << (UINT16 - 74))
                                | (1usize << (UINT32 - 74))
                                | (1usize << (UINT64 - 74))
                                | (1usize << (UINT8 - 74))
                                | (1usize << (VARINT - 74))
                                | (1usize << (VARINT16 - 74))
                                | (1usize << (VARINT32 - 74))
                                | (1usize << (VARINT64 - 74))
                                | (1usize << (VARSIZE - 74))
                                | (1usize << (VARUINT - 74))
                                | (1usize << (VARUINT16 - 74))
                                | (1usize << (VARUINT32 - 74))
                                | (1usize << (VARUINT64 - 74))))
                            != 0)
                    || _la == ID
                {
                    {
                        /*InvokeRule choiceFieldDefinition*/
                        recog.base.set_state(346);
                        recog.choiceFieldDefinition()?;
                    }
                }

                recog.base.set_state(349);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- choiceCase ----------------
pub type ChoiceCaseContextAll<'input> = ChoiceCaseContext<'input>;

pub type ChoiceCaseContext<'input> = BaseParserRuleContext<'input, ChoiceCaseContextExt<'input>>;

#[derive(Clone)]
pub struct ChoiceCaseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ChoiceCaseContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for ChoiceCaseContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_choiceCase(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_choiceCase(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for ChoiceCaseContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_choiceCase(self);
    }
}

impl<'input> CustomRuleContext<'input> for ChoiceCaseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_choiceCase
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_choiceCase }
}
antlr_rust::tid! {ChoiceCaseContextExt<'a>}

impl<'input> ChoiceCaseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ChoiceCaseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ChoiceCaseContextExt { ph: PhantomData },
        ))
    }
}

pub trait ChoiceCaseContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ChoiceCaseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CASE
    /// Returns `None` if there is no child corresponding to token CASE
    fn CASE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CASE, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
}

impl<'input> ChoiceCaseContextAttrs<'input> for ChoiceCaseContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn choiceCase(&mut self) -> Result<Rc<ChoiceCaseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ChoiceCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 42, RULE_choiceCase);
        let mut _localctx: Rc<ChoiceCaseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(351);
                recog.base.match_token(CASE, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(352);
                recog.expression_rec(0)?;

                recog.base.set_state(353);
                recog.base.match_token(COLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- choiceDefault ----------------
pub type ChoiceDefaultContextAll<'input> = ChoiceDefaultContext<'input>;

pub type ChoiceDefaultContext<'input> =
    BaseParserRuleContext<'input, ChoiceDefaultContextExt<'input>>;

#[derive(Clone)]
pub struct ChoiceDefaultContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ChoiceDefaultContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ChoiceDefaultContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_choiceDefault(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_choiceDefault(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for ChoiceDefaultContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_choiceDefault(self);
    }
}

impl<'input> CustomRuleContext<'input> for ChoiceDefaultContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_choiceDefault
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_choiceDefault }
}
antlr_rust::tid! {ChoiceDefaultContextExt<'a>}

impl<'input> ChoiceDefaultContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ChoiceDefaultContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ChoiceDefaultContextExt { ph: PhantomData },
        ))
    }
}

pub trait ChoiceDefaultContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ChoiceDefaultContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DEFAULT
    /// Returns `None` if there is no child corresponding to token DEFAULT
    fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEFAULT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn choiceFieldDefinition(&self) -> Option<Rc<ChoiceFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ChoiceDefaultContextAttrs<'input> for ChoiceDefaultContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn choiceDefault(&mut self) -> Result<Rc<ChoiceDefaultContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ChoiceDefaultContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_choiceDefault);
        let mut _localctx: Rc<ChoiceDefaultContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(355);
                recog.base.match_token(DEFAULT, &mut recog.err_handler)?;

                recog.base.set_state(356);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                recog.base.set_state(358);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la - 33) & !0x3f) == 0
                    && ((1usize << (_la - 33))
                        & ((1usize << (BIT_FIELD - 33))
                            | (1usize << (BOOL - 33))
                            | (1usize << (EXTERN - 33))
                            | (1usize << (FLOAT16 - 33))
                            | (1usize << (FLOAT32 - 33))
                            | (1usize << (FLOAT64 - 33))
                            | (1usize << (IMPLICIT - 33))
                            | (1usize << (INT_FIELD - 33))
                            | (1usize << (INT16 - 33))
                            | (1usize << (INT32 - 33))
                            | (1usize << (INT64 - 33))
                            | (1usize << (INT8 - 33))
                            | (1usize << (PACKED - 33))))
                        != 0)
                    || (((_la - 74) & !0x3f) == 0
                        && ((1usize << (_la - 74))
                            & ((1usize << (STRING - 74))
                                | (1usize << (UINT16 - 74))
                                | (1usize << (UINT32 - 74))
                                | (1usize << (UINT64 - 74))
                                | (1usize << (UINT8 - 74))
                                | (1usize << (VARINT - 74))
                                | (1usize << (VARINT16 - 74))
                                | (1usize << (VARINT32 - 74))
                                | (1usize << (VARINT64 - 74))
                                | (1usize << (VARSIZE - 74))
                                | (1usize << (VARUINT - 74))
                                | (1usize << (VARUINT16 - 74))
                                | (1usize << (VARUINT32 - 74))
                                | (1usize << (VARUINT64 - 74))))
                            != 0)
                    || _la == ID
                {
                    {
                        /*InvokeRule choiceFieldDefinition*/
                        recog.base.set_state(357);
                        recog.choiceFieldDefinition()?;
                    }
                }

                recog.base.set_state(360);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- choiceFieldDefinition ----------------
pub type ChoiceFieldDefinitionContextAll<'input> = ChoiceFieldDefinitionContext<'input>;

pub type ChoiceFieldDefinitionContext<'input> =
    BaseParserRuleContext<'input, ChoiceFieldDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ChoiceFieldDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ChoiceFieldDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ChoiceFieldDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_choiceFieldDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_choiceFieldDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ChoiceFieldDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_choiceFieldDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for ChoiceFieldDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_choiceFieldDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_choiceFieldDefinition }
}
antlr_rust::tid! {ChoiceFieldDefinitionContextExt<'a>}

impl<'input> ChoiceFieldDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ChoiceFieldDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ChoiceFieldDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ChoiceFieldDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ChoiceFieldDefinitionContextExt<'input>>
{
    fn fieldTypeId(&self) -> Option<Rc<FieldTypeIdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn fieldConstraint(&self) -> Option<Rc<FieldConstraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ChoiceFieldDefinitionContextAttrs<'input> for ChoiceFieldDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn choiceFieldDefinition(
        &mut self,
    ) -> Result<Rc<ChoiceFieldDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ChoiceFieldDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 46, RULE_choiceFieldDefinition);
        let mut _localctx: Rc<ChoiceFieldDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule fieldTypeId*/
                recog.base.set_state(362);
                recog.fieldTypeId()?;

                recog.base.set_state(364);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COLON {
                    {
                        /*InvokeRule fieldConstraint*/
                        recog.base.set_state(363);
                        recog.fieldConstraint()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- unionDeclaration ----------------
pub type UnionDeclarationContextAll<'input> = UnionDeclarationContext<'input>;

pub type UnionDeclarationContext<'input> =
    BaseParserRuleContext<'input, UnionDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct UnionDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for UnionDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for UnionDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_unionDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_unionDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for UnionDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_unionDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for UnionDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unionDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unionDeclaration }
}
antlr_rust::tid! {UnionDeclarationContextExt<'a>}

impl<'input> UnionDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<UnionDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            UnionDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait UnionDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<UnionDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token UNION
    /// Returns `None` if there is no child corresponding to token UNION
    fn UNION(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNION, 0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn templateParameters(&self) -> Option<Rc<TemplateParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn unionFieldDefinition_all(&self) -> Vec<Rc<UnionFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn unionFieldDefinition(&self, i: usize) -> Option<Rc<UnionFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn functionDefinition_all(&self) -> Vec<Rc<FunctionDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn functionDefinition(&self, i: usize) -> Option<Rc<FunctionDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> UnionDeclarationContextAttrs<'input> for UnionDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn unionDeclaration(
        &mut self,
    ) -> Result<Rc<UnionDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            UnionDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 48, RULE_unionDeclaration);
        let mut _localctx: Rc<UnionDeclarationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(366);
                recog.base.match_token(UNION, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(367);
                recog.id()?;

                recog.base.set_state(369);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == LT {
                    {
                        /*InvokeRule templateParameters*/
                        recog.base.set_state(368);
                        recog.templateParameters()?;
                    }
                }

                recog.base.set_state(372);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == LPAREN {
                    {
                        /*InvokeRule typeParameters*/
                        recog.base.set_state(371);
                        recog.typeParameters()?;
                    }
                }

                recog.base.set_state(374);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(378);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 33) & !0x3f) == 0
                    && ((1usize << (_la - 33))
                        & ((1usize << (BIT_FIELD - 33))
                            | (1usize << (BOOL - 33))
                            | (1usize << (EXTERN - 33))
                            | (1usize << (FLOAT16 - 33))
                            | (1usize << (FLOAT32 - 33))
                            | (1usize << (FLOAT64 - 33))
                            | (1usize << (IMPLICIT - 33))
                            | (1usize << (INT_FIELD - 33))
                            | (1usize << (INT16 - 33))
                            | (1usize << (INT32 - 33))
                            | (1usize << (INT64 - 33))
                            | (1usize << (INT8 - 33))
                            | (1usize << (PACKED - 33))))
                        != 0)
                    || (((_la - 74) & !0x3f) == 0
                        && ((1usize << (_la - 74))
                            & ((1usize << (STRING - 74))
                                | (1usize << (UINT16 - 74))
                                | (1usize << (UINT32 - 74))
                                | (1usize << (UINT64 - 74))
                                | (1usize << (UINT8 - 74))
                                | (1usize << (VARINT - 74))
                                | (1usize << (VARINT16 - 74))
                                | (1usize << (VARINT32 - 74))
                                | (1usize << (VARINT64 - 74))
                                | (1usize << (VARSIZE - 74))
                                | (1usize << (VARUINT - 74))
                                | (1usize << (VARUINT16 - 74))
                                | (1usize << (VARUINT32 - 74))
                                | (1usize << (VARUINT64 - 74))))
                            != 0)
                    || _la == ID
                {
                    {
                        {
                            /*InvokeRule unionFieldDefinition*/
                            recog.base.set_state(375);
                            recog.unionFieldDefinition()?;
                        }
                    }
                    recog.base.set_state(380);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(384);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == FUNCTION {
                    {
                        {
                            /*InvokeRule functionDefinition*/
                            recog.base.set_state(381);
                            recog.functionDefinition()?;
                        }
                    }
                    recog.base.set_state(386);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(387);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(388);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- unionFieldDefinition ----------------
pub type UnionFieldDefinitionContextAll<'input> = UnionFieldDefinitionContext<'input>;

pub type UnionFieldDefinitionContext<'input> =
    BaseParserRuleContext<'input, UnionFieldDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct UnionFieldDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for UnionFieldDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for UnionFieldDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_unionFieldDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_unionFieldDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for UnionFieldDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_unionFieldDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for UnionFieldDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unionFieldDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unionFieldDefinition }
}
antlr_rust::tid! {UnionFieldDefinitionContextExt<'a>}

impl<'input> UnionFieldDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<UnionFieldDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            UnionFieldDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait UnionFieldDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<UnionFieldDefinitionContextExt<'input>>
{
    fn choiceFieldDefinition(&self) -> Option<Rc<ChoiceFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> UnionFieldDefinitionContextAttrs<'input> for UnionFieldDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn unionFieldDefinition(
        &mut self,
    ) -> Result<Rc<UnionFieldDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            UnionFieldDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 50, RULE_unionFieldDefinition);
        let mut _localctx: Rc<UnionFieldDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule choiceFieldDefinition*/
                recog.base.set_state(390);
                recog.choiceFieldDefinition()?;

                recog.base.set_state(391);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- enumDeclaration ----------------
pub type EnumDeclarationContextAll<'input> = EnumDeclarationContext<'input>;

pub type EnumDeclarationContext<'input> =
    BaseParserRuleContext<'input, EnumDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct EnumDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for EnumDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for EnumDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_enumDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_enumDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for EnumDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_enumDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for EnumDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_enumDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_enumDeclaration }
}
antlr_rust::tid! {EnumDeclarationContextExt<'a>}

impl<'input> EnumDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<EnumDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            EnumDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait EnumDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<EnumDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ENUM
    /// Returns `None` if there is no child corresponding to token ENUM
    fn ENUM(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ENUM, 0)
    }
    fn typeInstantiation(&self) -> Option<Rc<TypeInstantiationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    fn enumItem_all(&self) -> Vec<Rc<EnumItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn enumItem(&self, i: usize) -> Option<Rc<EnumItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(COMMA)
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> EnumDeclarationContextAttrs<'input> for EnumDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn enumDeclaration(&mut self) -> Result<Rc<EnumDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            EnumDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 52, RULE_enumDeclaration);
        let mut _localctx: Rc<EnumDeclarationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(393);
                recog.base.match_token(ENUM, &mut recog.err_handler)?;

                /*InvokeRule typeInstantiation*/
                recog.base.set_state(394);
                recog.typeInstantiation()?;

                /*InvokeRule id*/
                recog.base.set_state(395);
                recog.id()?;

                recog.base.set_state(396);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                /*InvokeRule enumItem*/
                recog.base.set_state(397);
                recog.enumItem()?;

                recog.base.set_state(402);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(36, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(398);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule enumItem*/
                                recog.base.set_state(399);
                                recog.enumItem()?;
                            }
                        }
                    }
                    recog.base.set_state(404);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(36, &mut recog.base)?;
                }
                recog.base.set_state(406);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(405);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(408);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(409);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- enumItem ----------------
pub type EnumItemContextAll<'input> = EnumItemContext<'input>;

pub type EnumItemContext<'input> = BaseParserRuleContext<'input, EnumItemContextExt<'input>>;

#[derive(Clone)]
pub struct EnumItemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for EnumItemContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for EnumItemContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_enumItem(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_enumItem(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for EnumItemContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_enumItem(self);
    }
}

impl<'input> CustomRuleContext<'input> for EnumItemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_enumItem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_enumItem }
}
antlr_rust::tid! {EnumItemContextExt<'a>}

impl<'input> EnumItemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<EnumItemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            EnumItemContextExt { ph: PhantomData },
        ))
    }
}

pub trait EnumItemContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<EnumItemContextExt<'input>>
{
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> EnumItemContextAttrs<'input> for EnumItemContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn enumItem(&mut self) -> Result<Rc<EnumItemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = EnumItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_enumItem);
        let mut _localctx: Rc<EnumItemContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule id*/
                recog.base.set_state(411);
                recog.id()?;

                recog.base.set_state(414);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ASSIGN {
                    {
                        recog.base.set_state(412);
                        recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(413);
                        recog.expression_rec(0)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- bitmaskDeclaration ----------------
pub type BitmaskDeclarationContextAll<'input> = BitmaskDeclarationContext<'input>;

pub type BitmaskDeclarationContext<'input> =
    BaseParserRuleContext<'input, BitmaskDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct BitmaskDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for BitmaskDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for BitmaskDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_bitmaskDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_bitmaskDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for BitmaskDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_bitmaskDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for BitmaskDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_bitmaskDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_bitmaskDeclaration }
}
antlr_rust::tid! {BitmaskDeclarationContextExt<'a>}

impl<'input> BitmaskDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BitmaskDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BitmaskDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait BitmaskDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<BitmaskDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BITMASK
    /// Returns `None` if there is no child corresponding to token BITMASK
    fn BITMASK(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BITMASK, 0)
    }
    fn typeInstantiation(&self) -> Option<Rc<TypeInstantiationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    fn bitmaskValue_all(&self) -> Vec<Rc<BitmaskValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn bitmaskValue(&self, i: usize) -> Option<Rc<BitmaskValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(COMMA)
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> BitmaskDeclarationContextAttrs<'input> for BitmaskDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn bitmaskDeclaration(
        &mut self,
    ) -> Result<Rc<BitmaskDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            BitmaskDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 56, RULE_bitmaskDeclaration);
        let mut _localctx: Rc<BitmaskDeclarationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(416);
                recog.base.match_token(BITMASK, &mut recog.err_handler)?;

                /*InvokeRule typeInstantiation*/
                recog.base.set_state(417);
                recog.typeInstantiation()?;

                /*InvokeRule id*/
                recog.base.set_state(418);
                recog.id()?;

                recog.base.set_state(419);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                /*InvokeRule bitmaskValue*/
                recog.base.set_state(420);
                recog.bitmaskValue()?;

                recog.base.set_state(425);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(39, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(421);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule bitmaskValue*/
                                recog.base.set_state(422);
                                recog.bitmaskValue()?;
                            }
                        }
                    }
                    recog.base.set_state(427);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(39, &mut recog.base)?;
                }
                recog.base.set_state(429);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(428);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(431);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(432);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- bitmaskValue ----------------
pub type BitmaskValueContextAll<'input> = BitmaskValueContext<'input>;

pub type BitmaskValueContext<'input> =
    BaseParserRuleContext<'input, BitmaskValueContextExt<'input>>;

#[derive(Clone)]
pub struct BitmaskValueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for BitmaskValueContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for BitmaskValueContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_bitmaskValue(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_bitmaskValue(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for BitmaskValueContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_bitmaskValue(self);
    }
}

impl<'input> CustomRuleContext<'input> for BitmaskValueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_bitmaskValue
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_bitmaskValue }
}
antlr_rust::tid! {BitmaskValueContextExt<'a>}

impl<'input> BitmaskValueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BitmaskValueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BitmaskValueContextExt { ph: PhantomData },
        ))
    }
}

pub trait BitmaskValueContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<BitmaskValueContextExt<'input>>
{
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> BitmaskValueContextAttrs<'input> for BitmaskValueContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn bitmaskValue(&mut self) -> Result<Rc<BitmaskValueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BitmaskValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 58, RULE_bitmaskValue);
        let mut _localctx: Rc<BitmaskValueContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule id*/
                recog.base.set_state(434);
                recog.id()?;

                recog.base.set_state(437);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ASSIGN {
                    {
                        recog.base.set_state(435);
                        recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(436);
                        recog.expression_rec(0)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sqlTableDeclaration ----------------
pub type SqlTableDeclarationContextAll<'input> = SqlTableDeclarationContext<'input>;

pub type SqlTableDeclarationContext<'input> =
    BaseParserRuleContext<'input, SqlTableDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct SqlTableDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SqlTableDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SqlTableDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sqlTableDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_sqlTableDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for SqlTableDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_sqlTableDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for SqlTableDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sqlTableDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sqlTableDeclaration }
}
antlr_rust::tid! {SqlTableDeclarationContextExt<'a>}

impl<'input> SqlTableDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SqlTableDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SqlTableDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait SqlTableDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SqlTableDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SQL_TABLE
    /// Returns `None` if there is no child corresponding to token SQL_TABLE
    fn SQL_TABLE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SQL_TABLE, 0)
    }
    fn id_all(&self) -> Vec<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn templateParameters(&self) -> Option<Rc<TemplateParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token USING
    /// Returns `None` if there is no child corresponding to token USING
    fn USING(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(USING, 0)
    }
    fn sqlTableFieldDefinition_all(&self) -> Vec<Rc<SqlTableFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sqlTableFieldDefinition(
        &self,
        i: usize,
    ) -> Option<Rc<SqlTableFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn sqlConstraintDefinition(&self) -> Option<Rc<SqlConstraintDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sqlWithoutRowId(&self) -> Option<Rc<SqlWithoutRowIdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> SqlTableDeclarationContextAttrs<'input> for SqlTableDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sqlTableDeclaration(
        &mut self,
    ) -> Result<Rc<SqlTableDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SqlTableDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 60, RULE_sqlTableDeclaration);
        let mut _localctx: Rc<SqlTableDeclarationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(439);
                recog.base.match_token(SQL_TABLE, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(440);
                recog.id()?;

                recog.base.set_state(442);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == LT {
                    {
                        /*InvokeRule templateParameters*/
                        recog.base.set_state(441);
                        recog.templateParameters()?;
                    }
                }

                recog.base.set_state(446);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == USING {
                    {
                        recog.base.set_state(444);
                        recog.base.match_token(USING, &mut recog.err_handler)?;

                        /*InvokeRule id*/
                        recog.base.set_state(445);
                        recog.id()?;
                    }
                }

                recog.base.set_state(448);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(452);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 33) & !0x3f) == 0
                    && ((1usize << (_la - 33))
                        & ((1usize << (BIT_FIELD - 33))
                            | (1usize << (BOOL - 33))
                            | (1usize << (EXTERN - 33))
                            | (1usize << (FLOAT16 - 33))
                            | (1usize << (FLOAT32 - 33))
                            | (1usize << (FLOAT64 - 33))
                            | (1usize << (INT_FIELD - 33))
                            | (1usize << (INT16 - 33))
                            | (1usize << (INT32 - 33))
                            | (1usize << (INT64 - 33))
                            | (1usize << (INT8 - 33))))
                        != 0)
                    || (((_la - 72) & !0x3f) == 0
                        && ((1usize << (_la - 72))
                            & ((1usize << (SQL_VIRTUAL - 72))
                                | (1usize << (STRING - 72))
                                | (1usize << (UINT16 - 72))
                                | (1usize << (UINT32 - 72))
                                | (1usize << (UINT64 - 72))
                                | (1usize << (UINT8 - 72))
                                | (1usize << (VARINT - 72))
                                | (1usize << (VARINT16 - 72))
                                | (1usize << (VARINT32 - 72))
                                | (1usize << (VARINT64 - 72))
                                | (1usize << (VARSIZE - 72))
                                | (1usize << (VARUINT - 72))
                                | (1usize << (VARUINT16 - 72))
                                | (1usize << (VARUINT32 - 72))
                                | (1usize << (VARUINT64 - 72))))
                            != 0)
                    || _la == ID
                {
                    {
                        {
                            /*InvokeRule sqlTableFieldDefinition*/
                            recog.base.set_state(449);
                            recog.sqlTableFieldDefinition()?;
                        }
                    }
                    recog.base.set_state(454);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(456);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SQL {
                    {
                        /*InvokeRule sqlConstraintDefinition*/
                        recog.base.set_state(455);
                        recog.sqlConstraintDefinition()?;
                    }
                }

                recog.base.set_state(459);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SQL_WITHOUT_ROWID {
                    {
                        /*InvokeRule sqlWithoutRowId*/
                        recog.base.set_state(458);
                        recog.sqlWithoutRowId()?;
                    }
                }

                recog.base.set_state(461);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(462);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sqlTableFieldDefinition ----------------
pub type SqlTableFieldDefinitionContextAll<'input> = SqlTableFieldDefinitionContext<'input>;

pub type SqlTableFieldDefinitionContext<'input> =
    BaseParserRuleContext<'input, SqlTableFieldDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct SqlTableFieldDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SqlTableFieldDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SqlTableFieldDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sqlTableFieldDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_sqlTableFieldDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for SqlTableFieldDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_sqlTableFieldDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for SqlTableFieldDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sqlTableFieldDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sqlTableFieldDefinition }
}
antlr_rust::tid! {SqlTableFieldDefinitionContextExt<'a>}

impl<'input> SqlTableFieldDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SqlTableFieldDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SqlTableFieldDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait SqlTableFieldDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SqlTableFieldDefinitionContextExt<'input>>
{
    fn typeInstantiation(&self) -> Option<Rc<TypeInstantiationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SQL_VIRTUAL
    /// Returns `None` if there is no child corresponding to token SQL_VIRTUAL
    fn SQL_VIRTUAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SQL_VIRTUAL, 0)
    }
    fn sqlConstraint(&self) -> Option<Rc<SqlConstraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> SqlTableFieldDefinitionContextAttrs<'input>
    for SqlTableFieldDefinitionContext<'input>
{
}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sqlTableFieldDefinition(
        &mut self,
    ) -> Result<Rc<SqlTableFieldDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SqlTableFieldDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 62, RULE_sqlTableFieldDefinition);
        let mut _localctx: Rc<SqlTableFieldDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(465);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SQL_VIRTUAL {
                    {
                        recog.base.set_state(464);
                        recog
                            .base
                            .match_token(SQL_VIRTUAL, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule typeInstantiation*/
                recog.base.set_state(467);
                recog.typeInstantiation()?;

                /*InvokeRule id*/
                recog.base.set_state(468);
                recog.id()?;

                recog.base.set_state(470);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SQL {
                    {
                        /*InvokeRule sqlConstraint*/
                        recog.base.set_state(469);
                        recog.sqlConstraint()?;
                    }
                }

                recog.base.set_state(472);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sqlConstraintDefinition ----------------
pub type SqlConstraintDefinitionContextAll<'input> = SqlConstraintDefinitionContext<'input>;

pub type SqlConstraintDefinitionContext<'input> =
    BaseParserRuleContext<'input, SqlConstraintDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct SqlConstraintDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SqlConstraintDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SqlConstraintDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sqlConstraintDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_sqlConstraintDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for SqlConstraintDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_sqlConstraintDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for SqlConstraintDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sqlConstraintDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sqlConstraintDefinition }
}
antlr_rust::tid! {SqlConstraintDefinitionContextExt<'a>}

impl<'input> SqlConstraintDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SqlConstraintDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SqlConstraintDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait SqlConstraintDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SqlConstraintDefinitionContextExt<'input>>
{
    fn sqlConstraint(&self) -> Option<Rc<SqlConstraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> SqlConstraintDefinitionContextAttrs<'input>
    for SqlConstraintDefinitionContext<'input>
{
}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sqlConstraintDefinition(
        &mut self,
    ) -> Result<Rc<SqlConstraintDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SqlConstraintDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 64, RULE_sqlConstraintDefinition);
        let mut _localctx: Rc<SqlConstraintDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule sqlConstraint*/
                recog.base.set_state(474);
                recog.sqlConstraint()?;

                recog.base.set_state(475);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sqlConstraint ----------------
pub type SqlConstraintContextAll<'input> = SqlConstraintContext<'input>;

pub type SqlConstraintContext<'input> =
    BaseParserRuleContext<'input, SqlConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct SqlConstraintContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SqlConstraintContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SqlConstraintContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sqlConstraint(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_sqlConstraint(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for SqlConstraintContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_sqlConstraint(self);
    }
}

impl<'input> CustomRuleContext<'input> for SqlConstraintContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sqlConstraint
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sqlConstraint }
}
antlr_rust::tid! {SqlConstraintContextExt<'a>}

impl<'input> SqlConstraintContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SqlConstraintContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SqlConstraintContextExt { ph: PhantomData },
        ))
    }
}

pub trait SqlConstraintContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SqlConstraintContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SQL
    /// Returns `None` if there is no child corresponding to token SQL
    fn SQL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SQL, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> SqlConstraintContextAttrs<'input> for SqlConstraintContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sqlConstraint(&mut self) -> Result<Rc<SqlConstraintContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SqlConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 66, RULE_sqlConstraint);
        let mut _localctx: Rc<SqlConstraintContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(477);
                recog.base.match_token(SQL, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(478);
                recog.expression_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sqlWithoutRowId ----------------
pub type SqlWithoutRowIdContextAll<'input> = SqlWithoutRowIdContext<'input>;

pub type SqlWithoutRowIdContext<'input> =
    BaseParserRuleContext<'input, SqlWithoutRowIdContextExt<'input>>;

#[derive(Clone)]
pub struct SqlWithoutRowIdContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SqlWithoutRowIdContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SqlWithoutRowIdContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sqlWithoutRowId(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_sqlWithoutRowId(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for SqlWithoutRowIdContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_sqlWithoutRowId(self);
    }
}

impl<'input> CustomRuleContext<'input> for SqlWithoutRowIdContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sqlWithoutRowId
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sqlWithoutRowId }
}
antlr_rust::tid! {SqlWithoutRowIdContextExt<'a>}

impl<'input> SqlWithoutRowIdContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SqlWithoutRowIdContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SqlWithoutRowIdContextExt { ph: PhantomData },
        ))
    }
}

pub trait SqlWithoutRowIdContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SqlWithoutRowIdContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SQL_WITHOUT_ROWID
    /// Returns `None` if there is no child corresponding to token SQL_WITHOUT_ROWID
    fn SQL_WITHOUT_ROWID(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SQL_WITHOUT_ROWID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> SqlWithoutRowIdContextAttrs<'input> for SqlWithoutRowIdContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sqlWithoutRowId(&mut self) -> Result<Rc<SqlWithoutRowIdContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SqlWithoutRowIdContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 68, RULE_sqlWithoutRowId);
        let mut _localctx: Rc<SqlWithoutRowIdContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(480);
                recog
                    .base
                    .match_token(SQL_WITHOUT_ROWID, &mut recog.err_handler)?;

                recog.base.set_state(481);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sqlDatabaseDefinition ----------------
pub type SqlDatabaseDefinitionContextAll<'input> = SqlDatabaseDefinitionContext<'input>;

pub type SqlDatabaseDefinitionContext<'input> =
    BaseParserRuleContext<'input, SqlDatabaseDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct SqlDatabaseDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SqlDatabaseDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SqlDatabaseDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sqlDatabaseDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_sqlDatabaseDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for SqlDatabaseDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_sqlDatabaseDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for SqlDatabaseDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sqlDatabaseDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sqlDatabaseDefinition }
}
antlr_rust::tid! {SqlDatabaseDefinitionContextExt<'a>}

impl<'input> SqlDatabaseDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SqlDatabaseDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SqlDatabaseDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait SqlDatabaseDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SqlDatabaseDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SQL_DATABASE
    /// Returns `None` if there is no child corresponding to token SQL_DATABASE
    fn SQL_DATABASE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SQL_DATABASE, 0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn sqlDatabaseFieldDefinition_all(
        &self,
    ) -> Vec<Rc<SqlDatabaseFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sqlDatabaseFieldDefinition(
        &self,
        i: usize,
    ) -> Option<Rc<SqlDatabaseFieldDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> SqlDatabaseDefinitionContextAttrs<'input> for SqlDatabaseDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sqlDatabaseDefinition(
        &mut self,
    ) -> Result<Rc<SqlDatabaseDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SqlDatabaseDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 70, RULE_sqlDatabaseDefinition);
        let mut _localctx: Rc<SqlDatabaseDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(483);
                recog
                    .base
                    .match_token(SQL_DATABASE, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(484);
                recog.id()?;

                recog.base.set_state(485);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(487);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule sqlDatabaseFieldDefinition*/
                            recog.base.set_state(486);
                            recog.sqlDatabaseFieldDefinition()?;
                        }
                    }
                    recog.base.set_state(489);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !((((_la - 33) & !0x3f) == 0
                        && ((1usize << (_la - 33))
                            & ((1usize << (BIT_FIELD - 33))
                                | (1usize << (BOOL - 33))
                                | (1usize << (EXTERN - 33))
                                | (1usize << (FLOAT16 - 33))
                                | (1usize << (FLOAT32 - 33))
                                | (1usize << (FLOAT64 - 33))
                                | (1usize << (INT_FIELD - 33))
                                | (1usize << (INT16 - 33))
                                | (1usize << (INT32 - 33))
                                | (1usize << (INT64 - 33))
                                | (1usize << (INT8 - 33))))
                            != 0)
                        || (((_la - 74) & !0x3f) == 0
                            && ((1usize << (_la - 74))
                                & ((1usize << (STRING - 74))
                                    | (1usize << (UINT16 - 74))
                                    | (1usize << (UINT32 - 74))
                                    | (1usize << (UINT64 - 74))
                                    | (1usize << (UINT8 - 74))
                                    | (1usize << (VARINT - 74))
                                    | (1usize << (VARINT16 - 74))
                                    | (1usize << (VARINT32 - 74))
                                    | (1usize << (VARINT64 - 74))
                                    | (1usize << (VARSIZE - 74))
                                    | (1usize << (VARUINT - 74))
                                    | (1usize << (VARUINT16 - 74))
                                    | (1usize << (VARUINT32 - 74))
                                    | (1usize << (VARUINT64 - 74))))
                                != 0)
                        || _la == ID)
                    {
                        break;
                    }
                }
                recog.base.set_state(491);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(492);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sqlDatabaseFieldDefinition ----------------
pub type SqlDatabaseFieldDefinitionContextAll<'input> = SqlDatabaseFieldDefinitionContext<'input>;

pub type SqlDatabaseFieldDefinitionContext<'input> =
    BaseParserRuleContext<'input, SqlDatabaseFieldDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct SqlDatabaseFieldDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for SqlDatabaseFieldDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for SqlDatabaseFieldDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sqlDatabaseFieldDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_sqlDatabaseFieldDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for SqlDatabaseFieldDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_sqlDatabaseFieldDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for SqlDatabaseFieldDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sqlDatabaseFieldDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sqlDatabaseFieldDefinition }
}
antlr_rust::tid! {SqlDatabaseFieldDefinitionContextExt<'a>}

impl<'input> SqlDatabaseFieldDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SqlDatabaseFieldDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SqlDatabaseFieldDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait SqlDatabaseFieldDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<SqlDatabaseFieldDefinitionContextExt<'input>>
{
    fn typeInstantiation(&self) -> Option<Rc<TypeInstantiationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> SqlDatabaseFieldDefinitionContextAttrs<'input>
    for SqlDatabaseFieldDefinitionContext<'input>
{
}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sqlDatabaseFieldDefinition(
        &mut self,
    ) -> Result<Rc<SqlDatabaseFieldDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SqlDatabaseFieldDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 72, RULE_sqlDatabaseFieldDefinition);
        let mut _localctx: Rc<SqlDatabaseFieldDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule typeInstantiation*/
                recog.base.set_state(494);
                recog.typeInstantiation()?;

                /*InvokeRule id*/
                recog.base.set_state(495);
                recog.id()?;

                recog.base.set_state(496);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- serviceDefinition ----------------
pub type ServiceDefinitionContextAll<'input> = ServiceDefinitionContext<'input>;

pub type ServiceDefinitionContext<'input> =
    BaseParserRuleContext<'input, ServiceDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ServiceDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ServiceDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_serviceDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_serviceDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ServiceDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_serviceDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for ServiceDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_serviceDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_serviceDefinition }
}
antlr_rust::tid! {ServiceDefinitionContextExt<'a>}

impl<'input> ServiceDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ServiceDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ServiceDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ServiceDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ServiceDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SERVICE
    /// Returns `None` if there is no child corresponding to token SERVICE
    fn SERVICE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SERVICE, 0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn serviceMethodDefinition_all(&self) -> Vec<Rc<ServiceMethodDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn serviceMethodDefinition(
        &self,
        i: usize,
    ) -> Option<Rc<ServiceMethodDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ServiceDefinitionContextAttrs<'input> for ServiceDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn serviceDefinition(
        &mut self,
    ) -> Result<Rc<ServiceDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ServiceDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 74, RULE_serviceDefinition);
        let mut _localctx: Rc<ServiceDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(498);
                recog.base.match_token(SERVICE, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(499);
                recog.id()?;

                recog.base.set_state(500);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(504);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 33) & !0x3f) == 0
                    && ((1usize << (_la - 33))
                        & ((1usize << (BIT_FIELD - 33))
                            | (1usize << (BOOL - 33))
                            | (1usize << (EXTERN - 33))
                            | (1usize << (FLOAT16 - 33))
                            | (1usize << (FLOAT32 - 33))
                            | (1usize << (FLOAT64 - 33))
                            | (1usize << (INT_FIELD - 33))
                            | (1usize << (INT16 - 33))
                            | (1usize << (INT32 - 33))
                            | (1usize << (INT64 - 33))
                            | (1usize << (INT8 - 33))))
                        != 0)
                    || (((_la - 74) & !0x3f) == 0
                        && ((1usize << (_la - 74))
                            & ((1usize << (STRING - 74))
                                | (1usize << (UINT16 - 74))
                                | (1usize << (UINT32 - 74))
                                | (1usize << (UINT64 - 74))
                                | (1usize << (UINT8 - 74))
                                | (1usize << (VARINT - 74))
                                | (1usize << (VARINT16 - 74))
                                | (1usize << (VARINT32 - 74))
                                | (1usize << (VARINT64 - 74))
                                | (1usize << (VARSIZE - 74))
                                | (1usize << (VARUINT - 74))
                                | (1usize << (VARUINT16 - 74))
                                | (1usize << (VARUINT32 - 74))
                                | (1usize << (VARUINT64 - 74))))
                            != 0)
                    || _la == ID
                {
                    {
                        {
                            /*InvokeRule serviceMethodDefinition*/
                            recog.base.set_state(501);
                            recog.serviceMethodDefinition()?;
                        }
                    }
                    recog.base.set_state(506);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(507);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(508);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- serviceMethodDefinition ----------------
pub type ServiceMethodDefinitionContextAll<'input> = ServiceMethodDefinitionContext<'input>;

pub type ServiceMethodDefinitionContext<'input> =
    BaseParserRuleContext<'input, ServiceMethodDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceMethodDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ServiceMethodDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ServiceMethodDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_serviceMethodDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_serviceMethodDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ServiceMethodDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_serviceMethodDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for ServiceMethodDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_serviceMethodDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_serviceMethodDefinition }
}
antlr_rust::tid! {ServiceMethodDefinitionContextExt<'a>}

impl<'input> ServiceMethodDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ServiceMethodDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ServiceMethodDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ServiceMethodDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ServiceMethodDefinitionContextExt<'input>>
{
    fn typeReference_all(&self) -> Vec<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn typeReference(&self, i: usize) -> Option<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> ServiceMethodDefinitionContextAttrs<'input>
    for ServiceMethodDefinitionContext<'input>
{
}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn serviceMethodDefinition(
        &mut self,
    ) -> Result<Rc<ServiceMethodDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ServiceMethodDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 76, RULE_serviceMethodDefinition);
        let mut _localctx: Rc<ServiceMethodDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule typeReference*/
                recog.base.set_state(510);
                recog.typeReference()?;

                /*InvokeRule id*/
                recog.base.set_state(511);
                recog.id()?;

                recog.base.set_state(512);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                /*InvokeRule typeReference*/
                recog.base.set_state(513);
                recog.typeReference()?;

                recog.base.set_state(514);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;

                recog.base.set_state(515);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- pubsubDefinition ----------------
pub type PubsubDefinitionContextAll<'input> = PubsubDefinitionContext<'input>;

pub type PubsubDefinitionContext<'input> =
    BaseParserRuleContext<'input, PubsubDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct PubsubDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for PubsubDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for PubsubDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pubsubDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_pubsubDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for PubsubDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_pubsubDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for PubsubDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pubsubDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pubsubDefinition }
}
antlr_rust::tid! {PubsubDefinitionContextExt<'a>}

impl<'input> PubsubDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PubsubDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PubsubDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait PubsubDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<PubsubDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PUBSUB
    /// Returns `None` if there is no child corresponding to token PUBSUB
    fn PUBSUB(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PUBSUB, 0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    fn pubsubMessageDefinition_all(&self) -> Vec<Rc<PubsubMessageDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn pubsubMessageDefinition(
        &self,
        i: usize,
    ) -> Option<Rc<PubsubMessageDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> PubsubDefinitionContextAttrs<'input> for PubsubDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pubsubDefinition(
        &mut self,
    ) -> Result<Rc<PubsubDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            PubsubDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 78, RULE_pubsubDefinition);
        let mut _localctx: Rc<PubsubDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(517);
                recog.base.match_token(PUBSUB, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(518);
                recog.id()?;

                recog.base.set_state(519);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(523);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 64) & !0x3f) == 0
                    && ((1usize << (_la - 64))
                        & ((1usize << (PUBLISH - 64))
                            | (1usize << (SUBSCRIBE - 64))
                            | (1usize << (TOPIC - 64))))
                        != 0)
                {
                    {
                        {
                            /*InvokeRule pubsubMessageDefinition*/
                            recog.base.set_state(520);
                            recog.pubsubMessageDefinition()?;
                        }
                    }
                    recog.base.set_state(525);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(526);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                recog.base.set_state(527);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- pubsubMessageDefinition ----------------
pub type PubsubMessageDefinitionContextAll<'input> = PubsubMessageDefinitionContext<'input>;

pub type PubsubMessageDefinitionContext<'input> =
    BaseParserRuleContext<'input, PubsubMessageDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct PubsubMessageDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for PubsubMessageDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for PubsubMessageDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pubsubMessageDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_pubsubMessageDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for PubsubMessageDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_pubsubMessageDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for PubsubMessageDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pubsubMessageDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pubsubMessageDefinition }
}
antlr_rust::tid! {PubsubMessageDefinitionContextExt<'a>}

impl<'input> PubsubMessageDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PubsubMessageDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PubsubMessageDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait PubsubMessageDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<PubsubMessageDefinitionContextExt<'input>>
{
    fn topicDefinition(&self) -> Option<Rc<TopicDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn typeReference(&self) -> Option<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> PubsubMessageDefinitionContextAttrs<'input>
    for PubsubMessageDefinitionContext<'input>
{
}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pubsubMessageDefinition(
        &mut self,
    ) -> Result<Rc<PubsubMessageDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            PubsubMessageDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 80, RULE_pubsubMessageDefinition);
        let mut _localctx: Rc<PubsubMessageDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule topicDefinition*/
                recog.base.set_state(529);
                recog.topicDefinition()?;

                /*InvokeRule typeReference*/
                recog.base.set_state(530);
                recog.typeReference()?;

                /*InvokeRule id*/
                recog.base.set_state(531);
                recog.id()?;

                recog.base.set_state(532);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- topicDefinition ----------------
pub type TopicDefinitionContextAll<'input> = TopicDefinitionContext<'input>;

pub type TopicDefinitionContext<'input> =
    BaseParserRuleContext<'input, TopicDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct TopicDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TopicDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TopicDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_topicDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_topicDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for TopicDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_topicDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for TopicDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_topicDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_topicDefinition }
}
antlr_rust::tid! {TopicDefinitionContextExt<'a>}

impl<'input> TopicDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TopicDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TopicDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait TopicDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TopicDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token TOPIC
    /// Returns `None` if there is no child corresponding to token TOPIC
    fn TOPIC(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TOPIC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PUBLISH
    /// Returns `None` if there is no child corresponding to token PUBLISH
    fn PUBLISH(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PUBLISH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SUBSCRIBE
    /// Returns `None` if there is no child corresponding to token SUBSCRIBE
    fn SUBSCRIBE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SUBSCRIBE, 0)
    }
}

impl<'input> TopicDefinitionContextAttrs<'input> for TopicDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn topicDefinition(&mut self) -> Result<Rc<TopicDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TopicDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 82, RULE_topicDefinition);
        let mut _localctx: Rc<TopicDefinitionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(535);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == PUBLISH || _la == SUBSCRIBE {
                    {
                        recog.base.set_state(534);
                        _la = recog.base.input.la(1);
                        if { !(_la == PUBLISH || _la == SUBSCRIBE) } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                recog.base.set_state(537);
                recog.base.match_token(TOPIC, &mut recog.err_handler)?;

                recog.base.set_state(538);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(539);
                recog.expression_rec(0)?;

                recog.base.set_state(540);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- functionDefinition ----------------
pub type FunctionDefinitionContextAll<'input> = FunctionDefinitionContext<'input>;

pub type FunctionDefinitionContext<'input> =
    BaseParserRuleContext<'input, FunctionDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FunctionDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for FunctionDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_functionDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_functionDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for FunctionDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_functionDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for FunctionDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_functionDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_functionDefinition }
}
antlr_rust::tid! {FunctionDefinitionContextExt<'a>}

impl<'input> FunctionDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FunctionDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FunctionDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait FunctionDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FunctionDefinitionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token FUNCTION
    /// Returns `None` if there is no child corresponding to token FUNCTION
    fn FUNCTION(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FUNCTION, 0)
    }
    fn functionType(&self) -> Option<Rc<FunctionTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn functionName(&self) -> Option<Rc<FunctionNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    fn functionBody(&self) -> Option<Rc<FunctionBodyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FunctionDefinitionContextAttrs<'input> for FunctionDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn functionDefinition(
        &mut self,
    ) -> Result<Rc<FunctionDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            FunctionDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 84, RULE_functionDefinition);
        let mut _localctx: Rc<FunctionDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(542);
                recog.base.match_token(FUNCTION, &mut recog.err_handler)?;

                /*InvokeRule functionType*/
                recog.base.set_state(543);
                recog.functionType()?;

                /*InvokeRule functionName*/
                recog.base.set_state(544);
                recog.functionName()?;

                recog.base.set_state(545);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                recog.base.set_state(546);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;

                /*InvokeRule functionBody*/
                recog.base.set_state(547);
                recog.functionBody()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- functionType ----------------
pub type FunctionTypeContextAll<'input> = FunctionTypeContext<'input>;

pub type FunctionTypeContext<'input> =
    BaseParserRuleContext<'input, FunctionTypeContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FunctionTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for FunctionTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_functionType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_functionType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for FunctionTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_functionType(self);
    }
}

impl<'input> CustomRuleContext<'input> for FunctionTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_functionType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_functionType }
}
antlr_rust::tid! {FunctionTypeContextExt<'a>}

impl<'input> FunctionTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FunctionTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FunctionTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait FunctionTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FunctionTypeContextExt<'input>>
{
    fn typeReference(&self) -> Option<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FunctionTypeContextAttrs<'input> for FunctionTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn functionType(&mut self) -> Result<Rc<FunctionTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FunctionTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 86, RULE_functionType);
        let mut _localctx: Rc<FunctionTypeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule typeReference*/
                recog.base.set_state(549);
                recog.typeReference()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- functionName ----------------
pub type FunctionNameContextAll<'input> = FunctionNameContext<'input>;

pub type FunctionNameContext<'input> =
    BaseParserRuleContext<'input, FunctionNameContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FunctionNameContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for FunctionNameContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_functionName(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_functionName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for FunctionNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_functionName(self);
    }
}

impl<'input> CustomRuleContext<'input> for FunctionNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_functionName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_functionName }
}
antlr_rust::tid! {FunctionNameContextExt<'a>}

impl<'input> FunctionNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FunctionNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FunctionNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait FunctionNameContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FunctionNameContextExt<'input>>
{
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FunctionNameContextAttrs<'input> for FunctionNameContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn functionName(&mut self) -> Result<Rc<FunctionNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FunctionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 88, RULE_functionName);
        let mut _localctx: Rc<FunctionNameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule id*/
                recog.base.set_state(551);
                recog.id()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- functionBody ----------------
pub type FunctionBodyContextAll<'input> = FunctionBodyContext<'input>;

pub type FunctionBodyContext<'input> =
    BaseParserRuleContext<'input, FunctionBodyContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionBodyContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FunctionBodyContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for FunctionBodyContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_functionBody(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_functionBody(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for FunctionBodyContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_functionBody(self);
    }
}

impl<'input> CustomRuleContext<'input> for FunctionBodyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_functionBody
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_functionBody }
}
antlr_rust::tid! {FunctionBodyContextExt<'a>}

impl<'input> FunctionBodyContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FunctionBodyContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FunctionBodyContextExt { ph: PhantomData },
        ))
    }
}

pub trait FunctionBodyContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FunctionBodyContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RETURN
    /// Returns `None` if there is no child corresponding to token RETURN
    fn RETURN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RETURN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
}

impl<'input> FunctionBodyContextAttrs<'input> for FunctionBodyContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn functionBody(&mut self) -> Result<Rc<FunctionBodyContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FunctionBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 90, RULE_functionBody);
        let mut _localctx: Rc<FunctionBodyContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(553);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(554);
                recog.base.match_token(RETURN, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(555);
                recog.expression_rec(0)?;

                recog.base.set_state(556);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;

                recog.base.set_state(557);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- typeParameters ----------------
pub type TypeParametersContextAll<'input> = TypeParametersContext<'input>;

pub type TypeParametersContext<'input> =
    BaseParserRuleContext<'input, TypeParametersContextExt<'input>>;

#[derive(Clone)]
pub struct TypeParametersContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TypeParametersContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TypeParametersContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_typeParameters(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_typeParameters(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for TypeParametersContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_typeParameters(self);
    }
}

impl<'input> CustomRuleContext<'input> for TypeParametersContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_typeParameters
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_typeParameters }
}
antlr_rust::tid! {TypeParametersContextExt<'a>}

impl<'input> TypeParametersContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TypeParametersContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TypeParametersContextExt { ph: PhantomData },
        ))
    }
}

pub trait TypeParametersContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TypeParametersContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn parameterDefinition_all(&self) -> Vec<Rc<ParameterDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn parameterDefinition(&self, i: usize) -> Option<Rc<ParameterDefinitionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(COMMA)
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> TypeParametersContextAttrs<'input> for TypeParametersContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn typeParameters(&mut self) -> Result<Rc<TypeParametersContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TypeParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 92, RULE_typeParameters);
        let mut _localctx: Rc<TypeParametersContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(559);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                /*InvokeRule parameterDefinition*/
                recog.base.set_state(560);
                recog.parameterDefinition()?;

                recog.base.set_state(565);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(561);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule parameterDefinition*/
                            recog.base.set_state(562);
                            recog.parameterDefinition()?;
                        }
                    }
                    recog.base.set_state(567);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(568);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- parameterDefinition ----------------
pub type ParameterDefinitionContextAll<'input> = ParameterDefinitionContext<'input>;

pub type ParameterDefinitionContext<'input> =
    BaseParserRuleContext<'input, ParameterDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterDefinitionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ParameterDefinitionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ParameterDefinitionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_parameterDefinition(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_parameterDefinition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ParameterDefinitionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_parameterDefinition(self);
    }
}

impl<'input> CustomRuleContext<'input> for ParameterDefinitionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_parameterDefinition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_parameterDefinition }
}
antlr_rust::tid! {ParameterDefinitionContextExt<'a>}

impl<'input> ParameterDefinitionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ParameterDefinitionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ParameterDefinitionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ParameterDefinitionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ParameterDefinitionContextExt<'input>>
{
    fn typeReference(&self) -> Option<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ParameterDefinitionContextAttrs<'input> for ParameterDefinitionContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn parameterDefinition(
        &mut self,
    ) -> Result<Rc<ParameterDefinitionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ParameterDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 94, RULE_parameterDefinition);
        let mut _localctx: Rc<ParameterDefinitionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule typeReference*/
                recog.base.set_state(570);
                recog.typeReference()?;

                /*InvokeRule id*/
                recog.base.set_state(571);
                recog.id()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- templateParameters ----------------
pub type TemplateParametersContextAll<'input> = TemplateParametersContext<'input>;

pub type TemplateParametersContext<'input> =
    BaseParserRuleContext<'input, TemplateParametersContextExt<'input>>;

#[derive(Clone)]
pub struct TemplateParametersContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TemplateParametersContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TemplateParametersContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_templateParameters(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_templateParameters(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for TemplateParametersContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_templateParameters(self);
    }
}

impl<'input> CustomRuleContext<'input> for TemplateParametersContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_templateParameters
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_templateParameters }
}
antlr_rust::tid! {TemplateParametersContextExt<'a>}

impl<'input> TemplateParametersContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TemplateParametersContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TemplateParametersContextExt { ph: PhantomData },
        ))
    }
}

pub trait TemplateParametersContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TemplateParametersContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT, 0)
    }
    fn id_all(&self) -> Vec<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(COMMA)
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> TemplateParametersContextAttrs<'input> for TemplateParametersContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn templateParameters(
        &mut self,
    ) -> Result<Rc<TemplateParametersContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TemplateParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 96, RULE_templateParameters);
        let mut _localctx: Rc<TemplateParametersContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(573);
                recog.base.match_token(LT, &mut recog.err_handler)?;

                /*InvokeRule id*/
                recog.base.set_state(574);
                recog.id()?;

                recog.base.set_state(579);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(575);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule id*/
                            recog.base.set_state(576);
                            recog.id()?;
                        }
                    }
                    recog.base.set_state(581);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(582);
                recog.base.match_token(GT, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- templateArguments ----------------
pub type TemplateArgumentsContextAll<'input> = TemplateArgumentsContext<'input>;

pub type TemplateArgumentsContext<'input> =
    BaseParserRuleContext<'input, TemplateArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct TemplateArgumentsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TemplateArgumentsContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TemplateArgumentsContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_templateArguments(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_templateArguments(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for TemplateArgumentsContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_templateArguments(self);
    }
}

impl<'input> CustomRuleContext<'input> for TemplateArgumentsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_templateArguments
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_templateArguments }
}
antlr_rust::tid! {TemplateArgumentsContextExt<'a>}

impl<'input> TemplateArgumentsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TemplateArgumentsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TemplateArgumentsContextExt { ph: PhantomData },
        ))
    }
}

pub trait TemplateArgumentsContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TemplateArgumentsContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT, 0)
    }
    fn templateArgument_all(&self) -> Vec<Rc<TemplateArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn templateArgument(&self, i: usize) -> Option<Rc<TemplateArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(COMMA)
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> TemplateArgumentsContextAttrs<'input> for TemplateArgumentsContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn templateArguments(
        &mut self,
    ) -> Result<Rc<TemplateArgumentsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TemplateArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 98, RULE_templateArguments);
        let mut _localctx: Rc<TemplateArgumentsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(584);
                recog.base.match_token(LT, &mut recog.err_handler)?;

                /*InvokeRule templateArgument*/
                recog.base.set_state(585);
                recog.templateArgument()?;

                recog.base.set_state(590);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(586);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule templateArgument*/
                            recog.base.set_state(587);
                            recog.templateArgument()?;
                        }
                    }
                    recog.base.set_state(592);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(593);
                recog.base.match_token(GT, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- templateArgument ----------------
pub type TemplateArgumentContextAll<'input> = TemplateArgumentContext<'input>;

pub type TemplateArgumentContext<'input> =
    BaseParserRuleContext<'input, TemplateArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct TemplateArgumentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TemplateArgumentContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TemplateArgumentContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_templateArgument(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_templateArgument(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for TemplateArgumentContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_templateArgument(self);
    }
}

impl<'input> CustomRuleContext<'input> for TemplateArgumentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_templateArgument
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_templateArgument }
}
antlr_rust::tid! {TemplateArgumentContextExt<'a>}

impl<'input> TemplateArgumentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TemplateArgumentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TemplateArgumentContextExt { ph: PhantomData },
        ))
    }
}

pub trait TemplateArgumentContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TemplateArgumentContextExt<'input>>
{
    fn typeReference(&self) -> Option<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TemplateArgumentContextAttrs<'input> for TemplateArgumentContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn templateArgument(
        &mut self,
    ) -> Result<Rc<TemplateArgumentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TemplateArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 100, RULE_templateArgument);
        let mut _localctx: Rc<TemplateArgumentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule typeReference*/
                recog.base.set_state(595);
                recog.typeReference()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- instantiateDeclaration ----------------
pub type InstantiateDeclarationContextAll<'input> = InstantiateDeclarationContext<'input>;

pub type InstantiateDeclarationContext<'input> =
    BaseParserRuleContext<'input, InstantiateDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct InstantiateDeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for InstantiateDeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for InstantiateDeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_instantiateDeclaration(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_instantiateDeclaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for InstantiateDeclarationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_instantiateDeclaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for InstantiateDeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_instantiateDeclaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_instantiateDeclaration }
}
antlr_rust::tid! {InstantiateDeclarationContextExt<'a>}

impl<'input> InstantiateDeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<InstantiateDeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            InstantiateDeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait InstantiateDeclarationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<InstantiateDeclarationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token INSTANTIATE
    /// Returns `None` if there is no child corresponding to token INSTANTIATE
    fn INSTANTIATE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INSTANTIATE, 0)
    }
    fn typeReference(&self) -> Option<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> InstantiateDeclarationContextAttrs<'input> for InstantiateDeclarationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn instantiateDeclaration(
        &mut self,
    ) -> Result<Rc<InstantiateDeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            InstantiateDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 102, RULE_instantiateDeclaration);
        let mut _localctx: Rc<InstantiateDeclarationContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(597);
                recog
                    .base
                    .match_token(INSTANTIATE, &mut recog.err_handler)?;

                /*InvokeRule typeReference*/
                recog.base.set_state(598);
                recog.typeReference()?;

                /*InvokeRule id*/
                recog.base.set_state(599);
                recog.id()?;

                recog.base.set_state(600);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expression ----------------
#[derive(Debug)]
pub enum ExpressionContextAll<'input> {
    BitwiseXorExpressionContext(BitwiseXorExpressionContext<'input>),
    DotExpressionContext(DotExpressionContext<'input>),
    ValueofExpressionContext(ValueofExpressionContext<'input>),
    ShiftExpressionContext(ShiftExpressionContext<'input>),
    ArrayExpressionContext(ArrayExpressionContext<'input>),
    NumbitsExpressionContext(NumbitsExpressionContext<'input>),
    AdditiveExpressionContext(AdditiveExpressionContext<'input>),
    RelationalExpressionContext(RelationalExpressionContext<'input>),
    LengthofExpressionContext(LengthofExpressionContext<'input>),
    IdentifierExpressionContext(IdentifierExpressionContext<'input>),
    MultiplicativeExpressionContext(MultiplicativeExpressionContext<'input>),
    LogicalOrExpressionContext(LogicalOrExpressionContext<'input>),
    BitwiseOrExpressionContext(BitwiseOrExpressionContext<'input>),
    ParenthesizedExpressionContext(ParenthesizedExpressionContext<'input>),
    BitwiseAndExpressionContext(BitwiseAndExpressionContext<'input>),
    EqualityExpressionContext(EqualityExpressionContext<'input>),
    LogicalAndExpressionContext(LogicalAndExpressionContext<'input>),
    FunctionCallExpressionContext(FunctionCallExpressionContext<'input>),
    IndexExpressionContext(IndexExpressionContext<'input>),
    UnaryExpressionContext(UnaryExpressionContext<'input>),
    LiteralExpressionContext(LiteralExpressionContext<'input>),
    TernaryExpressionContext(TernaryExpressionContext<'input>),
    Error(ExpressionContext<'input>),
}
antlr_rust::tid! {ExpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExpressionContextAll<'input> {}

impl<'input> ZserioParserContext<'input> for ExpressionContextAll<'input> {}

impl<'input> Deref for ExpressionContextAll<'input> {
    type Target = dyn ExpressionContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use ExpressionContextAll::*;
        match self {
            BitwiseXorExpressionContext(inner) => inner,
            DotExpressionContext(inner) => inner,
            ValueofExpressionContext(inner) => inner,
            ShiftExpressionContext(inner) => inner,
            ArrayExpressionContext(inner) => inner,
            NumbitsExpressionContext(inner) => inner,
            AdditiveExpressionContext(inner) => inner,
            RelationalExpressionContext(inner) => inner,
            LengthofExpressionContext(inner) => inner,
            IdentifierExpressionContext(inner) => inner,
            MultiplicativeExpressionContext(inner) => inner,
            LogicalOrExpressionContext(inner) => inner,
            BitwiseOrExpressionContext(inner) => inner,
            ParenthesizedExpressionContext(inner) => inner,
            BitwiseAndExpressionContext(inner) => inner,
            EqualityExpressionContext(inner) => inner,
            LogicalAndExpressionContext(inner) => inner,
            FunctionCallExpressionContext(inner) => inner,
            IndexExpressionContext(inner) => inner,
            UnaryExpressionContext(inner) => inner,
            LiteralExpressionContext(inner) => inner,
            TernaryExpressionContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for ExpressionContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ExpressionContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type ExpressionContext<'input> = BaseParserRuleContext<'input, ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for ExpressionContext<'input> {}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for ExpressionContext<'input> {}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid! {ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                ExpressionContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait ExpressionContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>
{
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input> {}

pub type BitwiseXorExpressionContext<'input> =
    BaseParserRuleContext<'input, BitwiseXorExpressionContextExt<'input>>;

pub trait BitwiseXorExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token XOR
    /// Returns `None` if there is no child corresponding to token XOR
    fn XOR(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(XOR, 0)
    }
}

impl<'input> BitwiseXorExpressionContextAttrs<'input> for BitwiseXorExpressionContext<'input> {}

pub struct BitwiseXorExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {BitwiseXorExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for BitwiseXorExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for BitwiseXorExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_bitwiseXorExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_bitwiseXorExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for BitwiseXorExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_bitwiseXorExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for BitwiseXorExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for BitwiseXorExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for BitwiseXorExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for BitwiseXorExpressionContext<'input> {}

impl<'input> BitwiseXorExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::BitwiseXorExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                BitwiseXorExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type DotExpressionContext<'input> =
    BaseParserRuleContext<'input, DotExpressionContextExt<'input>>;

pub trait DotExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
}

impl<'input> DotExpressionContextAttrs<'input> for DotExpressionContext<'input> {}

pub struct DotExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {DotExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for DotExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for DotExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dotExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_dotExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for DotExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_dotExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for DotExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for DotExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for DotExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for DotExpressionContext<'input> {}

impl<'input> DotExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::DotExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                DotExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ValueofExpressionContext<'input> =
    BaseParserRuleContext<'input, ValueofExpressionContextExt<'input>>;

pub trait ValueofExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VALUEOF
    /// Returns `None` if there is no child corresponding to token VALUEOF
    fn VALUEOF(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VALUEOF, 0)
    }
}

impl<'input> ValueofExpressionContextAttrs<'input> for ValueofExpressionContext<'input> {}

pub struct ValueofExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ValueofExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for ValueofExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ValueofExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueofExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_valueofExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ValueofExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_valueofExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueofExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ValueofExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ValueofExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ValueofExpressionContext<'input> {}

impl<'input> ValueofExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ValueofExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ValueofExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ShiftExpressionContext<'input> =
    BaseParserRuleContext<'input, ShiftExpressionContextExt<'input>>;

pub trait ShiftExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token GT in current rule
    fn GT_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(GT)
    }
    /// Retrieves 'i's TerminalNode corresponding to token GT, starting from 0.
    /// Returns `None` if number of children corresponding to token GT is less or equal than `i`.
    fn GT(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT, i)
    }
    /// Retrieves first TerminalNode corresponding to token LSHIFT
    /// Returns `None` if there is no child corresponding to token LSHIFT
    fn LSHIFT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LSHIFT, 0)
    }
}

impl<'input> ShiftExpressionContextAttrs<'input> for ShiftExpressionContext<'input> {}

pub struct ShiftExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ShiftExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for ShiftExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ShiftExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_shiftExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_shiftExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ShiftExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_shiftExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for ShiftExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ShiftExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ShiftExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ShiftExpressionContext<'input> {}

impl<'input> ShiftExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ShiftExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ShiftExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ArrayExpressionContext<'input> =
    BaseParserRuleContext<'input, ArrayExpressionContextExt<'input>>;

pub trait ArrayExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACKET
    /// Returns `None` if there is no child corresponding to token RBRACKET
    fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACKET, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACKET
    /// Returns `None` if there is no child corresponding to token LBRACKET
    fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACKET, 0)
    }
}

impl<'input> ArrayExpressionContextAttrs<'input> for ArrayExpressionContext<'input> {}

pub struct ArrayExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ArrayExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for ArrayExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ArrayExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_arrayExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_arrayExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ArrayExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_arrayExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for ArrayExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ArrayExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ArrayExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ArrayExpressionContext<'input> {}

impl<'input> ArrayExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ArrayExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ArrayExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NumbitsExpressionContext<'input> =
    BaseParserRuleContext<'input, NumbitsExpressionContextExt<'input>>;

pub trait NumbitsExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NUMBITS
    /// Returns `None` if there is no child corresponding to token NUMBITS
    fn NUMBITS(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBITS, 0)
    }
}

impl<'input> NumbitsExpressionContextAttrs<'input> for NumbitsExpressionContext<'input> {}

pub struct NumbitsExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NumbitsExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for NumbitsExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for NumbitsExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_numbitsExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_numbitsExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for NumbitsExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_numbitsExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for NumbitsExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for NumbitsExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for NumbitsExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for NumbitsExpressionContext<'input> {}

impl<'input> NumbitsExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::NumbitsExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NumbitsExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type AdditiveExpressionContext<'input> =
    BaseParserRuleContext<'input, AdditiveExpressionContextExt<'input>>;

pub trait AdditiveExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
}

impl<'input> AdditiveExpressionContextAttrs<'input> for AdditiveExpressionContext<'input> {}

pub struct AdditiveExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AdditiveExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for AdditiveExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for AdditiveExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_additiveExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_additiveExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for AdditiveExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_additiveExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for AdditiveExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for AdditiveExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for AdditiveExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for AdditiveExpressionContext<'input> {}

impl<'input> AdditiveExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::AdditiveExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                AdditiveExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type RelationalExpressionContext<'input> =
    BaseParserRuleContext<'input, RelationalExpressionContextExt<'input>>;

pub trait RelationalExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LE
    /// Returns `None` if there is no child corresponding to token LE
    fn LE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GE
    /// Returns `None` if there is no child corresponding to token GE
    fn GE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GE, 0)
    }
}

impl<'input> RelationalExpressionContextAttrs<'input> for RelationalExpressionContext<'input> {}

pub struct RelationalExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {RelationalExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for RelationalExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for RelationalExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_relationalExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_relationalExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for RelationalExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_relationalExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for RelationalExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for RelationalExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for RelationalExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for RelationalExpressionContext<'input> {}

impl<'input> RelationalExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::RelationalExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                RelationalExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type LengthofExpressionContext<'input> =
    BaseParserRuleContext<'input, LengthofExpressionContextExt<'input>>;

pub trait LengthofExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LENGTHOF
    /// Returns `None` if there is no child corresponding to token LENGTHOF
    fn LENGTHOF(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LENGTHOF, 0)
    }
}

impl<'input> LengthofExpressionContextAttrs<'input> for LengthofExpressionContext<'input> {}

pub struct LengthofExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {LengthofExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for LengthofExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for LengthofExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_lengthofExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_lengthofExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for LengthofExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_lengthofExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for LengthofExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for LengthofExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for LengthofExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for LengthofExpressionContext<'input> {}

impl<'input> LengthofExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::LengthofExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                LengthofExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type IdentifierExpressionContext<'input> =
    BaseParserRuleContext<'input, IdentifierExpressionContextExt<'input>>;

pub trait IdentifierExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> IdentifierExpressionContextAttrs<'input> for IdentifierExpressionContext<'input> {}

pub struct IdentifierExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {IdentifierExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for IdentifierExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for IdentifierExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_identifierExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_identifierExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for IdentifierExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_identifierExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for IdentifierExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for IdentifierExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for IdentifierExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for IdentifierExpressionContext<'input> {}

impl<'input> IdentifierExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::IdentifierExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                IdentifierExpressionContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type MultiplicativeExpressionContext<'input> =
    BaseParserRuleContext<'input, MultiplicativeExpressionContextExt<'input>>;

pub trait MultiplicativeExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token MULTIPLY
    /// Returns `None` if there is no child corresponding to token MULTIPLY
    fn MULTIPLY(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MULTIPLY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DIVIDE
    /// Returns `None` if there is no child corresponding to token DIVIDE
    fn DIVIDE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DIVIDE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MODULO
    /// Returns `None` if there is no child corresponding to token MODULO
    fn MODULO(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MODULO, 0)
    }
}

impl<'input> MultiplicativeExpressionContextAttrs<'input>
    for MultiplicativeExpressionContext<'input>
{
}

pub struct MultiplicativeExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {MultiplicativeExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for MultiplicativeExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for MultiplicativeExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_multiplicativeExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_multiplicativeExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for MultiplicativeExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_multiplicativeExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for MultiplicativeExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for MultiplicativeExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for MultiplicativeExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for MultiplicativeExpressionContext<'input> {}

impl<'input> MultiplicativeExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::MultiplicativeExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                MultiplicativeExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type LogicalOrExpressionContext<'input> =
    BaseParserRuleContext<'input, LogicalOrExpressionContextExt<'input>>;

pub trait LogicalOrExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token LOGICAL_OR
    /// Returns `None` if there is no child corresponding to token LOGICAL_OR
    fn LOGICAL_OR(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LOGICAL_OR, 0)
    }
}

impl<'input> LogicalOrExpressionContextAttrs<'input> for LogicalOrExpressionContext<'input> {}

pub struct LogicalOrExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {LogicalOrExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for LogicalOrExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for LogicalOrExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_logicalOrExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_logicalOrExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for LogicalOrExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_logicalOrExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for LogicalOrExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for LogicalOrExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for LogicalOrExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for LogicalOrExpressionContext<'input> {}

impl<'input> LogicalOrExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::LogicalOrExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                LogicalOrExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type BitwiseOrExpressionContext<'input> =
    BaseParserRuleContext<'input, BitwiseOrExpressionContextExt<'input>>;

pub trait BitwiseOrExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OR
    /// Returns `None` if there is no child corresponding to token OR
    fn OR(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OR, 0)
    }
}

impl<'input> BitwiseOrExpressionContextAttrs<'input> for BitwiseOrExpressionContext<'input> {}

pub struct BitwiseOrExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {BitwiseOrExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for BitwiseOrExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for BitwiseOrExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_bitwiseOrExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_bitwiseOrExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for BitwiseOrExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_bitwiseOrExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for BitwiseOrExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for BitwiseOrExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for BitwiseOrExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for BitwiseOrExpressionContext<'input> {}

impl<'input> BitwiseOrExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::BitwiseOrExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                BitwiseOrExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ParenthesizedExpressionContext<'input> =
    BaseParserRuleContext<'input, ParenthesizedExpressionContextExt<'input>>;

pub trait ParenthesizedExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
}

impl<'input> ParenthesizedExpressionContextAttrs<'input>
    for ParenthesizedExpressionContext<'input>
{
}

pub struct ParenthesizedExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ParenthesizedExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for ParenthesizedExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for ParenthesizedExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_parenthesizedExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_parenthesizedExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for ParenthesizedExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_parenthesizedExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for ParenthesizedExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ParenthesizedExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ParenthesizedExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ParenthesizedExpressionContext<'input> {}

impl<'input> ParenthesizedExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ParenthesizedExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ParenthesizedExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type BitwiseAndExpressionContext<'input> =
    BaseParserRuleContext<'input, BitwiseAndExpressionContextExt<'input>>;

pub trait BitwiseAndExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token AND
    /// Returns `None` if there is no child corresponding to token AND
    fn AND(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AND, 0)
    }
}

impl<'input> BitwiseAndExpressionContextAttrs<'input> for BitwiseAndExpressionContext<'input> {}

pub struct BitwiseAndExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {BitwiseAndExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for BitwiseAndExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for BitwiseAndExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_bitwiseAndExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_bitwiseAndExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for BitwiseAndExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_bitwiseAndExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for BitwiseAndExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for BitwiseAndExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for BitwiseAndExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for BitwiseAndExpressionContext<'input> {}

impl<'input> BitwiseAndExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::BitwiseAndExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                BitwiseAndExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type EqualityExpressionContext<'input> =
    BaseParserRuleContext<'input, EqualityExpressionContextExt<'input>>;

pub trait EqualityExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token EQ
    /// Returns `None` if there is no child corresponding to token EQ
    fn EQ(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NE
    /// Returns `None` if there is no child corresponding to token NE
    fn NE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NE, 0)
    }
}

impl<'input> EqualityExpressionContextAttrs<'input> for EqualityExpressionContext<'input> {}

pub struct EqualityExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {EqualityExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for EqualityExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for EqualityExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_equalityExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_equalityExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for EqualityExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_equalityExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for EqualityExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for EqualityExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for EqualityExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for EqualityExpressionContext<'input> {}

impl<'input> EqualityExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::EqualityExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                EqualityExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type LogicalAndExpressionContext<'input> =
    BaseParserRuleContext<'input, LogicalAndExpressionContextExt<'input>>;

pub trait LogicalAndExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token LOGICAL_AND
    /// Returns `None` if there is no child corresponding to token LOGICAL_AND
    fn LOGICAL_AND(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LOGICAL_AND, 0)
    }
}

impl<'input> LogicalAndExpressionContextAttrs<'input> for LogicalAndExpressionContext<'input> {}

pub struct LogicalAndExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {LogicalAndExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for LogicalAndExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for LogicalAndExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_logicalAndExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_logicalAndExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for LogicalAndExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_logicalAndExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for LogicalAndExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for LogicalAndExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for LogicalAndExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for LogicalAndExpressionContext<'input> {}

impl<'input> LogicalAndExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::LogicalAndExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                LogicalAndExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type FunctionCallExpressionContext<'input> =
    BaseParserRuleContext<'input, FunctionCallExpressionContextExt<'input>>;

pub trait FunctionCallExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
}

impl<'input> FunctionCallExpressionContextAttrs<'input> for FunctionCallExpressionContext<'input> {}

pub struct FunctionCallExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {FunctionCallExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for FunctionCallExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for FunctionCallExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_functionCallExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_functionCallExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for FunctionCallExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_functionCallExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for FunctionCallExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for FunctionCallExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for FunctionCallExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for FunctionCallExpressionContext<'input> {}

impl<'input> FunctionCallExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::FunctionCallExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                FunctionCallExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type IndexExpressionContext<'input> =
    BaseParserRuleContext<'input, IndexExpressionContextExt<'input>>;

pub trait IndexExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token INDEX
    /// Returns `None` if there is no child corresponding to token INDEX
    fn INDEX(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INDEX, 0)
    }
}

impl<'input> IndexExpressionContextAttrs<'input> for IndexExpressionContext<'input> {}

pub struct IndexExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {IndexExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for IndexExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for IndexExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_indexExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_indexExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for IndexExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_indexExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for IndexExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for IndexExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for IndexExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for IndexExpressionContext<'input> {}

impl<'input> IndexExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::IndexExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                IndexExpressionContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type UnaryExpressionContext<'input> =
    BaseParserRuleContext<'input, UnaryExpressionContextExt<'input>>;

pub trait UnaryExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BANG
    /// Returns `None` if there is no child corresponding to token BANG
    fn BANG(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BANG, 0)
    }
    /// Retrieves first TerminalNode corresponding to token TILDE
    /// Returns `None` if there is no child corresponding to token TILDE
    fn TILDE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TILDE, 0)
    }
}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExpressionContext<'input> {}

pub struct UnaryExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {UnaryExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for UnaryExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for UnaryExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_unaryExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_unaryExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for UnaryExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_unaryExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for UnaryExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for UnaryExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for UnaryExpressionContext<'input> {}

impl<'input> UnaryExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::UnaryExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                UnaryExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type LiteralExpressionContext<'input> =
    BaseParserRuleContext<'input, LiteralExpressionContextExt<'input>>;

pub trait LiteralExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> LiteralExpressionContextAttrs<'input> for LiteralExpressionContext<'input> {}

pub struct LiteralExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {LiteralExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for LiteralExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for LiteralExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_literalExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_literalExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for LiteralExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_literalExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for LiteralExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for LiteralExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for LiteralExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for LiteralExpressionContext<'input> {}

impl<'input> LiteralExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::LiteralExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                LiteralExpressionContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type TernaryExpressionContext<'input> =
    BaseParserRuleContext<'input, TernaryExpressionContextExt<'input>>;

pub trait TernaryExpressionContextAttrs<'input>: ZserioParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token QUESTIONMARK
    /// Returns `None` if there is no child corresponding to token QUESTIONMARK
    fn QUESTIONMARK(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(QUESTIONMARK, 0)
    }
}

impl<'input> TernaryExpressionContextAttrs<'input> for TernaryExpressionContext<'input> {}

pub struct TernaryExpressionContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub operator: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {TernaryExpressionContextExt<'a>}

impl<'input> ZserioParserContext<'input> for TernaryExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TernaryExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ternaryExpression(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_ternaryExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for TernaryExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_ternaryExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for TernaryExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for TernaryExpressionContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for TernaryExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for TernaryExpressionContext<'input> {}

impl<'input> TernaryExpressionContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::TernaryExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                TernaryExpressionContextExt {
                    operator: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expression(&mut self) -> Result<Rc<ExpressionContextAll<'input>>, ANTLRError> {
        self.expression_rec(0)
    }

    fn expression_rec(
        &mut self,
        _p: isize,
    ) -> Result<Rc<ExpressionContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 104, RULE_expression, _p);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 104;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(627);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    LPAREN => {
                        {
                            let mut tmp = ParenthesizedExpressionContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();

                            recog.base.set_state(603);
                            let tmp = recog.base.match_token(LPAREN, &mut recog.err_handler)?;
                            if let ExpressionContextAll::ParenthesizedExpressionContext(ctx) =
                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                            {
                                ctx.operator = Some(tmp.clone());
                            } else {
                                unreachable!("cant cast");
                            }

                            /*InvokeRule expression*/
                            recog.base.set_state(604);
                            recog.expression_rec(0)?;

                            recog.base.set_state(605);
                            recog.base.match_token(RPAREN, &mut recog.err_handler)?;
                        }
                    }

                    LENGTHOF => {
                        {
                            let mut tmp = LengthofExpressionContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(607);
                            let tmp = recog.base.match_token(LENGTHOF, &mut recog.err_handler)?;
                            if let ExpressionContextAll::LengthofExpressionContext(ctx) =
                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                            {
                                ctx.operator = Some(tmp.clone());
                            } else {
                                unreachable!("cant cast");
                            }

                            recog.base.set_state(608);
                            recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                            /*InvokeRule expression*/
                            recog.base.set_state(609);
                            recog.expression_rec(0)?;

                            recog.base.set_state(610);
                            recog.base.match_token(RPAREN, &mut recog.err_handler)?;
                        }
                    }

                    VALUEOF => {
                        {
                            let mut tmp = ValueofExpressionContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(612);
                            let tmp = recog.base.match_token(VALUEOF, &mut recog.err_handler)?;
                            if let ExpressionContextAll::ValueofExpressionContext(ctx) =
                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                            {
                                ctx.operator = Some(tmp.clone());
                            } else {
                                unreachable!("cant cast");
                            }

                            recog.base.set_state(613);
                            recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                            /*InvokeRule expression*/
                            recog.base.set_state(614);
                            recog.expression_rec(0)?;

                            recog.base.set_state(615);
                            recog.base.match_token(RPAREN, &mut recog.err_handler)?;
                        }
                    }

                    NUMBITS => {
                        {
                            let mut tmp = NumbitsExpressionContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(617);
                            let tmp = recog.base.match_token(NUMBITS, &mut recog.err_handler)?;
                            if let ExpressionContextAll::NumbitsExpressionContext(ctx) =
                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                            {
                                ctx.operator = Some(tmp.clone());
                            } else {
                                unreachable!("cant cast");
                            }

                            recog.base.set_state(618);
                            recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                            /*InvokeRule expression*/
                            recog.base.set_state(619);
                            recog.expression_rec(0)?;

                            recog.base.set_state(620);
                            recog.base.match_token(RPAREN, &mut recog.err_handler)?;
                        }
                    }

                    BANG | MINUS | PLUS | TILDE => {
                        {
                            let mut tmp = UnaryExpressionContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(622);
                            if let ExpressionContextAll::UnaryExpressionContext(ctx) =
                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                            {
                                ctx.operator = recog.base.input.lt(1).cloned();
                            } else {
                                unreachable!("cant cast");
                            }
                            _la = recog.base.input.la(1);
                            if {
                                !(((_la) & !0x3f) == 0
                                    && ((1usize << _la)
                                        & ((1usize << BANG)
                                            | (1usize << MINUS)
                                            | (1usize << PLUS)
                                            | (1usize << TILDE)))
                                        != 0)
                            } {
                                let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                                if let ExpressionContextAll::UnaryExpressionContext(ctx) =
                                    cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                {
                                    ctx.operator = Some(tmp.clone());
                                } else {
                                    unreachable!("cant cast");
                                }
                            } else {
                                if recog.base.input.la(1) == TOKEN_EOF {
                                    recog.base.matched_eof = true
                                };
                                recog.err_handler.report_match(&mut recog.base);
                                recog.base.consume(&mut recog.err_handler);
                            }
                            /*InvokeRule expression*/
                            recog.base.set_state(623);
                            recog.expression_rec(15)?;
                        }
                    }

                    BOOL_LITERAL | STRING_LITERAL | BINARY_LITERAL | OCTAL_LITERAL
                    | HEXADECIMAL_LITERAL | DOUBLE_LITERAL | FLOAT_LITERAL | DECIMAL_LITERAL => {
                        {
                            let mut tmp = LiteralExpressionContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            /*InvokeRule literal*/
                            recog.base.set_state(624);
                            recog.literal()?;
                        }
                    }

                    INDEX => {
                        let mut tmp = IndexExpressionContextExt::new(&**_localctx);
                        recog.ctx = Some(tmp.clone());
                        _localctx = tmp;
                        _prevctx = _localctx.clone();
                        recog.base.set_state(625);
                        recog.base.match_token(INDEX, &mut recog.err_handler)?;
                    }

                    ID => {
                        {
                            let mut tmp = IdentifierExpressionContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            /*InvokeRule id*/
                            recog.base.set_state(626);
                            recog.id()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(682);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(59, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(680);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(58, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = MultiplicativeExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(629);
                                        if !({ recog.precpred(None, 14) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 14)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(630);
                                        if let ExpressionContextAll::MultiplicativeExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");}
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la) & !0x3f) == 0
                                                && ((1usize << _la)
                                                    & ((1usize << DIVIDE)
                                                        | (1usize << MODULO)
                                                        | (1usize << MULTIPLY)))
                                                    != 0)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::MultiplicativeExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(631);
                                        recog.expression_rec(15)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = AdditiveExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(632);
                                        if !({ recog.precpred(None, 13) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 13)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(633);
                                        if let ExpressionContextAll::AdditiveExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if { !(_la == MINUS || _la == PLUS) } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::AdditiveExpressionContext(
                                                ctx,
                                            ) =
                                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                            {
                                                ctx.operator = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(634);
                                        recog.expression_rec(14)?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = ShiftExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(635);
                                        if !({ recog.precpred(None, 12) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 12)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(639);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        match recog.base.input.la(1) {
                                            LSHIFT => {
                                                recog.base.set_state(636);
                                                let tmp = recog
                                                    .base
                                                    .match_token(LSHIFT, &mut recog.err_handler)?;
                                                if let ExpressionContextAll::ShiftExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
									ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}
                                            }

                                            GT => {
                                                recog.base.set_state(637);
                                                let tmp = recog
                                                    .base
                                                    .match_token(GT, &mut recog.err_handler)?;
                                                if let ExpressionContextAll::ShiftExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
									ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}

                                                recog.base.set_state(638);
                                                recog
                                                    .base
                                                    .match_token(GT, &mut recog.err_handler)?;
                                            }

                                            _ => Err(ANTLRError::NoAltError(
                                                NoViableAltError::new(&mut recog.base),
                                            ))?,
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(641);
                                        recog.expression_rec(13)?;
                                    }
                                }
                                4 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = RelationalExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(642);
                                        if !({ recog.precpred(None, 11) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 11)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(643);
                                        if let ExpressionContextAll::RelationalExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la) & !0x3f) == 0
                                                && ((1usize << _la)
                                                    & ((1usize << GE)
                                                        | (1usize << GT)
                                                        | (1usize << LE)
                                                        | (1usize << LT)))
                                                    != 0)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::RelationalExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(644);
                                        recog.expression_rec(12)?;
                                    }
                                }
                                5 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = EqualityExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(645);
                                        if !({ recog.precpred(None, 10) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 10)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(646);
                                        if let ExpressionContextAll::EqualityExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if { !(_la == EQ || _la == NE) } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::EqualityExpressionContext(
                                                ctx,
                                            ) =
                                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                            {
                                                ctx.operator = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(647);
                                        recog.expression_rec(11)?;
                                    }
                                }
                                6 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BitwiseAndExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(648);
                                        if !({ recog.precpred(None, 9) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 9)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(649);
                                        let tmp =
                                            recog.base.match_token(AND, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::BitwiseAndExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expression*/
                                        recog.base.set_state(650);
                                        recog.expression_rec(10)?;
                                    }
                                }
                                7 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BitwiseXorExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(651);
                                        if !({ recog.precpred(None, 8) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 8)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(652);
                                        let tmp =
                                            recog.base.match_token(XOR, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::BitwiseXorExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expression*/
                                        recog.base.set_state(653);
                                        recog.expression_rec(9)?;
                                    }
                                }
                                8 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BitwiseOrExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(654);
                                        if !({ recog.precpred(None, 7) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 7)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(655);
                                        let tmp =
                                            recog.base.match_token(OR, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::BitwiseOrExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expression*/
                                        recog.base.set_state(656);
                                        recog.expression_rec(8)?;
                                    }
                                }
                                9 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = LogicalAndExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(657);
                                        if !({ recog.precpred(None, 6) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 6)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(658);
                                        let tmp = recog
                                            .base
                                            .match_token(LOGICAL_AND, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::LogicalAndExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expression*/
                                        recog.base.set_state(659);
                                        recog.expression_rec(7)?;
                                    }
                                }
                                10 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = LogicalOrExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(660);
                                        if !({ recog.precpred(None, 5) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 5)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(661);
                                        let tmp = recog
                                            .base
                                            .match_token(LOGICAL_OR, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::LogicalOrExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expression*/
                                        recog.base.set_state(662);
                                        recog.expression_rec(6)?;
                                    }
                                }
                                11 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = TernaryExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(663);
                                        if !({ recog.precpred(None, 4) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(664);
                                        let tmp = recog
                                            .base
                                            .match_token(QUESTIONMARK, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::TernaryExpressionContext(ctx) =
                                            cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expression*/
                                        recog.base.set_state(665);
                                        recog.expression_rec(0)?;

                                        recog.base.set_state(666);
                                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                                        /*InvokeRule expression*/
                                        recog.base.set_state(667);
                                        recog.expression_rec(4)?;
                                    }
                                }
                                12 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = FunctionCallExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(669);
                                        if !({ recog.precpred(None, 21) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 21)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(670);
                                        recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                                        recog.base.set_state(671);
                                        let tmp = recog
                                            .base
                                            .match_token(RPAREN, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::FunctionCallExpressionContext(
                                            ctx,
                                        ) = cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                    }
                                }
                                13 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = ArrayExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(672);
                                        if !({ recog.precpred(None, 20) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 20)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(673);
                                        let tmp = recog
                                            .base
                                            .match_token(LBRACKET, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::ArrayExpressionContext(ctx) =
                                            cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule expression*/
                                        recog.base.set_state(674);
                                        recog.expression_rec(0)?;

                                        recog.base.set_state(675);
                                        recog.base.match_token(RBRACKET, &mut recog.err_handler)?;
                                    }
                                }
                                14 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = DotExpressionContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(677);
                                        if !({ recog.precpred(None, 19) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 19)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(678);
                                        let tmp =
                                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                                        if let ExpressionContextAll::DotExpressionContext(ctx) =
                                            cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.operator = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }

                                        /*InvokeRule id*/
                                        recog.base.set_state(679);
                                        recog.id()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(684);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(59, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;

pub type LiteralContext<'input> = BaseParserRuleContext<'input, LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for LiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for LiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_literal(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_literal(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for LiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid! {LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<LiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            LiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait LiteralContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<LiteralContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BINARY_LITERAL
    /// Returns `None` if there is no child corresponding to token BINARY_LITERAL
    fn BINARY_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BINARY_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OCTAL_LITERAL
    /// Returns `None` if there is no child corresponding to token OCTAL_LITERAL
    fn OCTAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OCTAL_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
    /// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
    fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DECIMAL_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token HEXADECIMAL_LITERAL
    /// Returns `None` if there is no child corresponding to token HEXADECIMAL_LITERAL
    fn HEXADECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HEXADECIMAL_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BOOL_LITERAL
    /// Returns `None` if there is no child corresponding to token BOOL_LITERAL
    fn BOOL_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BOOL_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FLOAT_LITERAL
    /// Returns `None` if there is no child corresponding to token FLOAT_LITERAL
    fn FLOAT_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FLOAT_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DOUBLE_LITERAL
    /// Returns `None` if there is no child corresponding to token DOUBLE_LITERAL
    fn DOUBLE_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOUBLE_LITERAL, 0)
    }
}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn literal(&mut self) -> Result<Rc<LiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(685);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 100) & !0x3f) == 0
                        && ((1usize << (_la - 100))
                            & ((1usize << (BOOL_LITERAL - 100))
                                | (1usize << (STRING_LITERAL - 100))
                                | (1usize << (BINARY_LITERAL - 100))
                                | (1usize << (OCTAL_LITERAL - 100))
                                | (1usize << (HEXADECIMAL_LITERAL - 100))
                                | (1usize << (DOUBLE_LITERAL - 100))
                                | (1usize << (FLOAT_LITERAL - 100))
                                | (1usize << (DECIMAL_LITERAL - 100))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- id ----------------
pub type IdContextAll<'input> = IdContext<'input>;

pub type IdContext<'input> = BaseParserRuleContext<'input, IdContextExt<'input>>;

#[derive(Clone)]
pub struct IdContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for IdContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for IdContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_id(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_id(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for IdContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_id(self);
    }
}

impl<'input> CustomRuleContext<'input> for IdContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_id
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_id }
}
antlr_rust::tid! {IdContextExt<'a>}

impl<'input> IdContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<IdContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            IdContextExt { ph: PhantomData },
        ))
    }
}

pub trait IdContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<IdContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
}

impl<'input> IdContextAttrs<'input> for IdContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn id(&mut self) -> Result<Rc<IdContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = IdContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_id);
        let mut _localctx: Rc<IdContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(687);
                recog.base.match_token(ID, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- typeReference ----------------
pub type TypeReferenceContextAll<'input> = TypeReferenceContext<'input>;

pub type TypeReferenceContext<'input> =
    BaseParserRuleContext<'input, TypeReferenceContextExt<'input>>;

#[derive(Clone)]
pub struct TypeReferenceContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TypeReferenceContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TypeReferenceContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_typeReference(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_typeReference(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for TypeReferenceContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_typeReference(self);
    }
}

impl<'input> CustomRuleContext<'input> for TypeReferenceContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_typeReference
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_typeReference }
}
antlr_rust::tid! {TypeReferenceContextExt<'a>}

impl<'input> TypeReferenceContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TypeReferenceContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TypeReferenceContextExt { ph: PhantomData },
        ))
    }
}

pub trait TypeReferenceContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TypeReferenceContextExt<'input>>
{
    fn builtinType(&self) -> Option<Rc<BuiltinTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn qualifiedName(&self) -> Option<Rc<QualifiedNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn templateArguments(&self) -> Option<Rc<TemplateArgumentsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TypeReferenceContextAttrs<'input> for TypeReferenceContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn typeReference(&mut self) -> Result<Rc<TypeReferenceContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TypeReferenceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 110, RULE_typeReference);
        let mut _localctx: Rc<TypeReferenceContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(694);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                BIT_FIELD | BOOL | EXTERN | FLOAT16 | FLOAT32 | FLOAT64 | INT_FIELD | INT16
                | INT32 | INT64 | INT8 | STRING | UINT16 | UINT32 | UINT64 | UINT8 | VARINT
                | VARINT16 | VARINT32 | VARINT64 | VARSIZE | VARUINT | VARUINT16 | VARUINT32
                | VARUINT64 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule builtinType*/
                        recog.base.set_state(689);
                        recog.builtinType()?;
                    }
                }

                ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule qualifiedName*/
                        recog.base.set_state(690);
                        recog.qualifiedName()?;

                        recog.base.set_state(692);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(60, &mut recog.base)? {
                            x if x == 1 => {
                                {
                                    /*InvokeRule templateArguments*/
                                    recog.base.set_state(691);
                                    recog.templateArguments()?;
                                }
                            }

                            _ => {}
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- typeInstantiation ----------------
pub type TypeInstantiationContextAll<'input> = TypeInstantiationContext<'input>;

pub type TypeInstantiationContext<'input> =
    BaseParserRuleContext<'input, TypeInstantiationContextExt<'input>>;

#[derive(Clone)]
pub struct TypeInstantiationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TypeInstantiationContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TypeInstantiationContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_typeInstantiation(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_typeInstantiation(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for TypeInstantiationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_typeInstantiation(self);
    }
}

impl<'input> CustomRuleContext<'input> for TypeInstantiationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_typeInstantiation
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_typeInstantiation }
}
antlr_rust::tid! {TypeInstantiationContextExt<'a>}

impl<'input> TypeInstantiationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TypeInstantiationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TypeInstantiationContextExt { ph: PhantomData },
        ))
    }
}

pub trait TypeInstantiationContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TypeInstantiationContextExt<'input>>
{
    fn typeReference(&self) -> Option<Rc<TypeReferenceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn typeArguments(&self) -> Option<Rc<TypeArgumentsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn dynamicLengthArgument(&self) -> Option<Rc<DynamicLengthArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TypeInstantiationContextAttrs<'input> for TypeInstantiationContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn typeInstantiation(
        &mut self,
    ) -> Result<Rc<TypeInstantiationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TypeInstantiationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 112, RULE_typeInstantiation);
        let mut _localctx: Rc<TypeInstantiationContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule typeReference*/
                recog.base.set_state(696);
                recog.typeReference()?;

                recog.base.set_state(699);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    LPAREN => {
                        {
                            /*InvokeRule typeArguments*/
                            recog.base.set_state(697);
                            recog.typeArguments()?;
                        }
                    }

                    LT => {
                        {
                            /*InvokeRule dynamicLengthArgument*/
                            recog.base.set_state(698);
                            recog.dynamicLengthArgument()?;
                        }
                    }

                    ID => {}

                    _ => {}
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- builtinType ----------------
pub type BuiltinTypeContextAll<'input> = BuiltinTypeContext<'input>;

pub type BuiltinTypeContext<'input> = BaseParserRuleContext<'input, BuiltinTypeContextExt<'input>>;

#[derive(Clone)]
pub struct BuiltinTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for BuiltinTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for BuiltinTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_builtinType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_builtinType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for BuiltinTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_builtinType(self);
    }
}

impl<'input> CustomRuleContext<'input> for BuiltinTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_builtinType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_builtinType }
}
antlr_rust::tid! {BuiltinTypeContextExt<'a>}

impl<'input> BuiltinTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BuiltinTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BuiltinTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait BuiltinTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<BuiltinTypeContextExt<'input>>
{
    fn intType(&self) -> Option<Rc<IntTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn varintType(&self) -> Option<Rc<VarintTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn fixedBitFieldType(&self) -> Option<Rc<FixedBitFieldTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn dynamicBitFieldType(&self) -> Option<Rc<DynamicBitFieldTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn boolType(&self) -> Option<Rc<BoolTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn stringType(&self) -> Option<Rc<StringTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn floatType(&self) -> Option<Rc<FloatTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn externType(&self) -> Option<Rc<ExternTypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> BuiltinTypeContextAttrs<'input> for BuiltinTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn builtinType(&mut self) -> Result<Rc<BuiltinTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BuiltinTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 114, RULE_builtinType);
        let mut _localctx: Rc<BuiltinTypeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(709);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(63, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule intType*/
                        recog.base.set_state(701);
                        recog.intType()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule varintType*/
                        recog.base.set_state(702);
                        recog.varintType()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule fixedBitFieldType*/
                        recog.base.set_state(703);
                        recog.fixedBitFieldType()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule dynamicBitFieldType*/
                        recog.base.set_state(704);
                        recog.dynamicBitFieldType()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule boolType*/
                        recog.base.set_state(705);
                        recog.boolType()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule stringType*/
                        recog.base.set_state(706);
                        recog.stringType()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule floatType*/
                        recog.base.set_state(707);
                        recog.floatType()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule externType*/
                        recog.base.set_state(708);
                        recog.externType()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- qualifiedName ----------------
pub type QualifiedNameContextAll<'input> = QualifiedNameContext<'input>;

pub type QualifiedNameContext<'input> =
    BaseParserRuleContext<'input, QualifiedNameContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiedNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for QualifiedNameContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for QualifiedNameContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_qualifiedName(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_qualifiedName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for QualifiedNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_qualifiedName(self);
    }
}

impl<'input> CustomRuleContext<'input> for QualifiedNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_qualifiedName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedName }
}
antlr_rust::tid! {QualifiedNameContextExt<'a>}

impl<'input> QualifiedNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<QualifiedNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            QualifiedNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait QualifiedNameContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<QualifiedNameContextExt<'input>>
{
    fn id_all(&self) -> Vec<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(DOT)
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
}

impl<'input> QualifiedNameContextAttrs<'input> for QualifiedNameContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn qualifiedName(&mut self) -> Result<Rc<QualifiedNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            QualifiedNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 116, RULE_qualifiedName);
        let mut _localctx: Rc<QualifiedNameContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule id*/
                recog.base.set_state(711);
                recog.id()?;

                recog.base.set_state(716);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == DOT {
                    {
                        {
                            recog.base.set_state(712);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;

                            /*InvokeRule id*/
                            recog.base.set_state(713);
                            recog.id()?;
                        }
                    }
                    recog.base.set_state(718);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- typeArguments ----------------
pub type TypeArgumentsContextAll<'input> = TypeArgumentsContext<'input>;

pub type TypeArgumentsContext<'input> =
    BaseParserRuleContext<'input, TypeArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct TypeArgumentsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TypeArgumentsContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for TypeArgumentsContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_typeArguments(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_typeArguments(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for TypeArgumentsContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_typeArguments(self);
    }
}

impl<'input> CustomRuleContext<'input> for TypeArgumentsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_typeArguments
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_typeArguments }
}
antlr_rust::tid! {TypeArgumentsContextExt<'a>}

impl<'input> TypeArgumentsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TypeArgumentsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TypeArgumentsContextExt { ph: PhantomData },
        ))
    }
}

pub trait TypeArgumentsContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TypeArgumentsContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn typeArgument_all(&self) -> Vec<Rc<TypeArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn typeArgument(&self, i: usize) -> Option<Rc<TypeArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_tokens(COMMA)
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> TypeArgumentsContextAttrs<'input> for TypeArgumentsContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn typeArguments(&mut self) -> Result<Rc<TypeArgumentsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TypeArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 118, RULE_typeArguments);
        let mut _localctx: Rc<TypeArgumentsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(719);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                /*InvokeRule typeArgument*/
                recog.base.set_state(720);
                recog.typeArgument()?;

                recog.base.set_state(725);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(721);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule typeArgument*/
                            recog.base.set_state(722);
                            recog.typeArgument()?;
                        }
                    }
                    recog.base.set_state(727);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(728);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- typeArgument ----------------
pub type TypeArgumentContextAll<'input> = TypeArgumentContext<'input>;

pub type TypeArgumentContext<'input> =
    BaseParserRuleContext<'input, TypeArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct TypeArgumentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for TypeArgumentContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for TypeArgumentContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_typeArgument(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_typeArgument(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for TypeArgumentContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_typeArgument(self);
    }
}

impl<'input> CustomRuleContext<'input> for TypeArgumentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_typeArgument
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_typeArgument }
}
antlr_rust::tid! {TypeArgumentContextExt<'a>}

impl<'input> TypeArgumentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TypeArgumentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TypeArgumentContextExt { ph: PhantomData },
        ))
    }
}

pub trait TypeArgumentContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<TypeArgumentContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EXPLICIT
    /// Returns `None` if there is no child corresponding to token EXPLICIT
    fn EXPLICIT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXPLICIT, 0)
    }
    fn id(&self) -> Option<Rc<IdContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TypeArgumentContextAttrs<'input> for TypeArgumentContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn typeArgument(&mut self) -> Result<Rc<TypeArgumentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TypeArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 120, RULE_typeArgument);
        let mut _localctx: Rc<TypeArgumentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(733);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                EXPLICIT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(730);
                        recog.base.match_token(EXPLICIT, &mut recog.err_handler)?;

                        /*InvokeRule id*/
                        recog.base.set_state(731);
                        recog.id()?;
                    }
                }

                BANG | LPAREN | MINUS | PLUS | TILDE | INDEX | LENGTHOF | NUMBITS | VALUEOF
                | BOOL_LITERAL | STRING_LITERAL | BINARY_LITERAL | OCTAL_LITERAL
                | HEXADECIMAL_LITERAL | DOUBLE_LITERAL | FLOAT_LITERAL | DECIMAL_LITERAL | ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule expression*/
                        recog.base.set_state(732);
                        recog.expression_rec(0)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- dynamicLengthArgument ----------------
pub type DynamicLengthArgumentContextAll<'input> = DynamicLengthArgumentContext<'input>;

pub type DynamicLengthArgumentContext<'input> =
    BaseParserRuleContext<'input, DynamicLengthArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct DynamicLengthArgumentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for DynamicLengthArgumentContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for DynamicLengthArgumentContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dynamicLengthArgument(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_dynamicLengthArgument(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for DynamicLengthArgumentContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_dynamicLengthArgument(self);
    }
}

impl<'input> CustomRuleContext<'input> for DynamicLengthArgumentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dynamicLengthArgument
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dynamicLengthArgument }
}
antlr_rust::tid! {DynamicLengthArgumentContextExt<'a>}

impl<'input> DynamicLengthArgumentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DynamicLengthArgumentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DynamicLengthArgumentContextExt { ph: PhantomData },
        ))
    }
}

pub trait DynamicLengthArgumentContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<DynamicLengthArgumentContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT, 0)
    }
}

impl<'input> DynamicLengthArgumentContextAttrs<'input> for DynamicLengthArgumentContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn dynamicLengthArgument(
        &mut self,
    ) -> Result<Rc<DynamicLengthArgumentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            DynamicLengthArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 122, RULE_dynamicLengthArgument);
        let mut _localctx: Rc<DynamicLengthArgumentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(735);
                recog.base.match_token(LT, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(736);
                recog.expression_rec(0)?;

                recog.base.set_state(737);
                recog.base.match_token(GT, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- intType ----------------
pub type IntTypeContextAll<'input> = IntTypeContext<'input>;

pub type IntTypeContext<'input> = BaseParserRuleContext<'input, IntTypeContextExt<'input>>;

#[derive(Clone)]
pub struct IntTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for IntTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for IntTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_intType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_intType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for IntTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_intType(self);
    }
}

impl<'input> CustomRuleContext<'input> for IntTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_intType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_intType }
}
antlr_rust::tid! {IntTypeContextExt<'a>}

impl<'input> IntTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<IntTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            IntTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait IntTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<IntTypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token INT8
    /// Returns `None` if there is no child corresponding to token INT8
    fn INT8(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INT8, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INT16
    /// Returns `None` if there is no child corresponding to token INT16
    fn INT16(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INT16, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INT32
    /// Returns `None` if there is no child corresponding to token INT32
    fn INT32(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INT32, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INT64
    /// Returns `None` if there is no child corresponding to token INT64
    fn INT64(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INT64, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UINT8
    /// Returns `None` if there is no child corresponding to token UINT8
    fn UINT8(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UINT8, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UINT16
    /// Returns `None` if there is no child corresponding to token UINT16
    fn UINT16(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UINT16, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UINT32
    /// Returns `None` if there is no child corresponding to token UINT32
    fn UINT32(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UINT32, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UINT64
    /// Returns `None` if there is no child corresponding to token UINT64
    fn UINT64(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UINT64, 0)
    }
}

impl<'input> IntTypeContextAttrs<'input> for IntTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn intType(&mut self) -> Result<Rc<IntTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = IntTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_intType);
        let mut _localctx: Rc<IntTypeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(739);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 53) & !0x3f) == 0
                        && ((1usize << (_la - 53))
                            & ((1usize << (INT16 - 53))
                                | (1usize << (INT32 - 53))
                                | (1usize << (INT64 - 53))
                                | (1usize << (INT8 - 53))
                                | (1usize << (UINT16 - 53))
                                | (1usize << (UINT32 - 53))
                                | (1usize << (UINT64 - 53))
                                | (1usize << (UINT8 - 53))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- varintType ----------------
pub type VarintTypeContextAll<'input> = VarintTypeContext<'input>;

pub type VarintTypeContext<'input> = BaseParserRuleContext<'input, VarintTypeContextExt<'input>>;

#[derive(Clone)]
pub struct VarintTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for VarintTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for VarintTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_varintType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_varintType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for VarintTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_varintType(self);
    }
}

impl<'input> CustomRuleContext<'input> for VarintTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_varintType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_varintType }
}
antlr_rust::tid! {VarintTypeContextExt<'a>}

impl<'input> VarintTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<VarintTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            VarintTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait VarintTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<VarintTypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token VARINT
    /// Returns `None` if there is no child corresponding to token VARINT
    fn VARINT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARINT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VARINT16
    /// Returns `None` if there is no child corresponding to token VARINT16
    fn VARINT16(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARINT16, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VARINT32
    /// Returns `None` if there is no child corresponding to token VARINT32
    fn VARINT32(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARINT32, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VARINT64
    /// Returns `None` if there is no child corresponding to token VARINT64
    fn VARINT64(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARINT64, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VARSIZE
    /// Returns `None` if there is no child corresponding to token VARSIZE
    fn VARSIZE(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARSIZE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VARUINT
    /// Returns `None` if there is no child corresponding to token VARUINT
    fn VARUINT(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARUINT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VARUINT16
    /// Returns `None` if there is no child corresponding to token VARUINT16
    fn VARUINT16(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARUINT16, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VARUINT32
    /// Returns `None` if there is no child corresponding to token VARUINT32
    fn VARUINT32(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARUINT32, 0)
    }
    /// Retrieves first TerminalNode corresponding to token VARUINT64
    /// Returns `None` if there is no child corresponding to token VARUINT64
    fn VARUINT64(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VARUINT64, 0)
    }
}

impl<'input> VarintTypeContextAttrs<'input> for VarintTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn varintType(&mut self) -> Result<Rc<VarintTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = VarintTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 126, RULE_varintType);
        let mut _localctx: Rc<VarintTypeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(741);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 86) & !0x3f) == 0
                        && ((1usize << (_la - 86))
                            & ((1usize << (VARINT - 86))
                                | (1usize << (VARINT16 - 86))
                                | (1usize << (VARINT32 - 86))
                                | (1usize << (VARINT64 - 86))
                                | (1usize << (VARSIZE - 86))
                                | (1usize << (VARUINT - 86))
                                | (1usize << (VARUINT16 - 86))
                                | (1usize << (VARUINT32 - 86))
                                | (1usize << (VARUINT64 - 86))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fixedBitFieldType ----------------
pub type FixedBitFieldTypeContextAll<'input> = FixedBitFieldTypeContext<'input>;

pub type FixedBitFieldTypeContext<'input> =
    BaseParserRuleContext<'input, FixedBitFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct FixedBitFieldTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FixedBitFieldTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for FixedBitFieldTypeContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fixedBitFieldType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_fixedBitFieldType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for FixedBitFieldTypeContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_fixedBitFieldType(self);
    }
}

impl<'input> CustomRuleContext<'input> for FixedBitFieldTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fixedBitFieldType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fixedBitFieldType }
}
antlr_rust::tid! {FixedBitFieldTypeContextExt<'a>}

impl<'input> FixedBitFieldTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FixedBitFieldTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FixedBitFieldTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait FixedBitFieldTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FixedBitFieldTypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
    /// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
    fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DECIMAL_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BIT_FIELD
    /// Returns `None` if there is no child corresponding to token BIT_FIELD
    fn BIT_FIELD(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BIT_FIELD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INT_FIELD
    /// Returns `None` if there is no child corresponding to token INT_FIELD
    fn INT_FIELD(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INT_FIELD, 0)
    }
}

impl<'input> FixedBitFieldTypeContextAttrs<'input> for FixedBitFieldTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fixedBitFieldType(
        &mut self,
    ) -> Result<Rc<FixedBitFieldTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            FixedBitFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 128, RULE_fixedBitFieldType);
        let mut _localctx: Rc<FixedBitFieldTypeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(743);
                _la = recog.base.input.la(1);
                if { !(_la == BIT_FIELD || _la == INT_FIELD) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
                recog.base.set_state(744);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                recog.base.set_state(745);
                recog
                    .base
                    .match_token(DECIMAL_LITERAL, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- dynamicBitFieldType ----------------
pub type DynamicBitFieldTypeContextAll<'input> = DynamicBitFieldTypeContext<'input>;

pub type DynamicBitFieldTypeContext<'input> =
    BaseParserRuleContext<'input, DynamicBitFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct DynamicBitFieldTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for DynamicBitFieldTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a>
    for DynamicBitFieldTypeContext<'input>
{
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dynamicBitFieldType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_dynamicBitFieldType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a>
    for DynamicBitFieldTypeContext<'input>
{
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_dynamicBitFieldType(self);
    }
}

impl<'input> CustomRuleContext<'input> for DynamicBitFieldTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dynamicBitFieldType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dynamicBitFieldType }
}
antlr_rust::tid! {DynamicBitFieldTypeContextExt<'a>}

impl<'input> DynamicBitFieldTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DynamicBitFieldTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DynamicBitFieldTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait DynamicBitFieldTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<DynamicBitFieldTypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BIT_FIELD
    /// Returns `None` if there is no child corresponding to token BIT_FIELD
    fn BIT_FIELD(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BIT_FIELD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INT_FIELD
    /// Returns `None` if there is no child corresponding to token INT_FIELD
    fn INT_FIELD(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INT_FIELD, 0)
    }
}

impl<'input> DynamicBitFieldTypeContextAttrs<'input> for DynamicBitFieldTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn dynamicBitFieldType(
        &mut self,
    ) -> Result<Rc<DynamicBitFieldTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            DynamicBitFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 130, RULE_dynamicBitFieldType);
        let mut _localctx: Rc<DynamicBitFieldTypeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(747);
                _la = recog.base.input.la(1);
                if { !(_la == BIT_FIELD || _la == INT_FIELD) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- boolType ----------------
pub type BoolTypeContextAll<'input> = BoolTypeContext<'input>;

pub type BoolTypeContext<'input> = BaseParserRuleContext<'input, BoolTypeContextExt<'input>>;

#[derive(Clone)]
pub struct BoolTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for BoolTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for BoolTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_boolType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_boolType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for BoolTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_boolType(self);
    }
}

impl<'input> CustomRuleContext<'input> for BoolTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_boolType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_boolType }
}
antlr_rust::tid! {BoolTypeContextExt<'a>}

impl<'input> BoolTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BoolTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BoolTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait BoolTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<BoolTypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BOOL
    /// Returns `None` if there is no child corresponding to token BOOL
    fn BOOL(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BOOL, 0)
    }
}

impl<'input> BoolTypeContextAttrs<'input> for BoolTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn boolType(&mut self) -> Result<Rc<BoolTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BoolTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_boolType);
        let mut _localctx: Rc<BoolTypeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(749);
                recog.base.match_token(BOOL, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stringType ----------------
pub type StringTypeContextAll<'input> = StringTypeContext<'input>;

pub type StringTypeContext<'input> = BaseParserRuleContext<'input, StringTypeContextExt<'input>>;

#[derive(Clone)]
pub struct StringTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for StringTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for StringTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_stringType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_stringType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for StringTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_stringType(self);
    }
}

impl<'input> CustomRuleContext<'input> for StringTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_stringType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_stringType }
}
antlr_rust::tid! {StringTypeContextExt<'a>}

impl<'input> StringTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StringTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StringTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait StringTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<StringTypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> StringTypeContextAttrs<'input> for StringTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn stringType(&mut self) -> Result<Rc<StringTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StringTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 134, RULE_stringType);
        let mut _localctx: Rc<StringTypeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(751);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- floatType ----------------
pub type FloatTypeContextAll<'input> = FloatTypeContext<'input>;

pub type FloatTypeContext<'input> = BaseParserRuleContext<'input, FloatTypeContextExt<'input>>;

#[derive(Clone)]
pub struct FloatTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for FloatTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for FloatTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_floatType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_floatType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for FloatTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_floatType(self);
    }
}

impl<'input> CustomRuleContext<'input> for FloatTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_floatType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_floatType }
}
antlr_rust::tid! {FloatTypeContextExt<'a>}

impl<'input> FloatTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FloatTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FloatTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait FloatTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<FloatTypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token FLOAT16
    /// Returns `None` if there is no child corresponding to token FLOAT16
    fn FLOAT16(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FLOAT16, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FLOAT32
    /// Returns `None` if there is no child corresponding to token FLOAT32
    fn FLOAT32(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FLOAT32, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FLOAT64
    /// Returns `None` if there is no child corresponding to token FLOAT64
    fn FLOAT64(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FLOAT64, 0)
    }
}

impl<'input> FloatTypeContextAttrs<'input> for FloatTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn floatType(&mut self) -> Result<Rc<FloatTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FloatTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 136, RULE_floatType);
        let mut _localctx: Rc<FloatTypeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(753);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 43) & !0x3f) == 0
                        && ((1usize << (_la - 43))
                            & ((1usize << (FLOAT16 - 43))
                                | (1usize << (FLOAT32 - 43))
                                | (1usize << (FLOAT64 - 43))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- externType ----------------
pub type ExternTypeContextAll<'input> = ExternTypeContext<'input>;

pub type ExternTypeContext<'input> = BaseParserRuleContext<'input, ExternTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ExternTypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ZserioParserContext<'input> for ExternTypeContext<'input> {}

impl<'input, 'a> Listenable<dyn ZserioParserListener<'input> + 'a> for ExternTypeContext<'input> {
    fn enter(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_externType(self);
    }
    fn exit(&self, listener: &mut (dyn ZserioParserListener<'input> + 'a)) {
        listener.exit_externType(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn ZserioParserVisitor<'input> + 'a> for ExternTypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn ZserioParserVisitor<'input> + 'a)) {
        visitor.visit_externType(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExternTypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ZserioParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_externType
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_externType }
}
antlr_rust::tid! {ExternTypeContextExt<'a>}

impl<'input> ExternTypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ZserioParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExternTypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExternTypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait ExternTypeContextAttrs<'input>:
    ZserioParserContext<'input> + BorrowMut<ExternTypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EXTERN
    /// Returns `None` if there is no child corresponding to token EXTERN
    fn EXTERN(&self) -> Option<Rc<TerminalNode<'input, ZserioParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXTERN, 0)
    }
}

impl<'input> ExternTypeContextAttrs<'input> for ExternTypeContext<'input> {}

impl<'input, I, H> ZserioParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn externType(&mut self) -> Result<Rc<ExternTypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExternTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 138, RULE_externType);
        let mut _localctx: Rc<ExternTypeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(755);
                recog.base.match_token(EXTERN, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x71\u{2f8}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x03\x02\x05\x02\u{90}\
	\x0a\x02\x03\x02\x07\x02\u{93}\x0a\x02\x0c\x02\x0e\x02\u{96}\x0b\x02\x03\
	\x02\x07\x02\u{99}\x0a\x02\x0c\x02\x0e\x02\u{9c}\x0b\x02\x03\x02\x03\x02\
	\x03\x03\x03\x03\x03\x03\x03\x03\x07\x03\u{a4}\x0a\x03\x0c\x03\x0e\x03\u{a7}\
	\x0b\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\
	\x07\x04\u{b1}\x0a\x04\x0c\x04\x0e\x04\u{b4}\x0b\x04\x03\x04\x03\x04\x05\
	\x04\u{b8}\x0a\x04\x03\x04\x03\x04\x03\x05\x03\x05\x05\x05\u{be}\x0a\x05\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x05\x06\u{cb}\x0a\x06\x03\x07\x03\x07\x05\x07\u{cf}\x0a\
	\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x09\x03\
	\x09\x03\x09\x03\x09\x07\x09\u{dc}\x0a\x09\x0c\x09\x0e\x09\u{df}\x0b\x09\
	\x03\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{f0}\x0a\x0c\x03\
	\x0c\x05\x0c\u{f3}\x0a\x0c\x03\x0c\x03\x0c\x07\x0c\u{f7}\x0a\x0c\x0c\x0c\
	\x0e\x0c\u{fa}\x0b\x0c\x03\x0c\x07\x0c\u{fd}\x0a\x0c\x0c\x0c\x0e\x0c\u{100}\
	\x0b\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x05\x0d\u{106}\x0a\x0d\x03\x0d\
	\x05\x0d\u{109}\x0a\x0d\x03\x0d\x05\x0d\u{10c}\x0a\x0d\x03\x0d\x03\x0d\x05\
	\x0d\u{110}\x0a\x0d\x03\x0d\x05\x0d\u{113}\x0a\x0d\x03\x0d\x05\x0d\u{116}\
	\x0a\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0f\x03\x0f\x03\x0f\x03\x10\x05\x10\u{124}\x0a\x10\x03\x10\x05\x10\
	\u{127}\x0a\x10\x03\x10\x03\x10\x03\x10\x05\x10\u{12c}\x0a\x10\x03\x11\x03\
	\x11\x05\x11\u{130}\x0a\x11\x03\x11\x03\x11\x03\x12\x03\x12\x03\x12\x03\
	\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x05\
	\x15\u{140}\x0a\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x07\x15\u{147}\
	\x0a\x15\x0c\x15\x0e\x15\u{14a}\x0b\x15\x03\x15\x05\x15\u{14d}\x0a\x15\x03\
	\x15\x07\x15\u{150}\x0a\x15\x0c\x15\x0e\x15\u{153}\x0b\x15\x03\x15\x03\x15\
	\x03\x15\x03\x16\x06\x16\u{159}\x0a\x16\x0d\x16\x0e\x16\u{15a}\x03\x16\x05\
	\x16\u{15e}\x0a\x16\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\x03\
	\x18\x03\x18\x03\x18\x05\x18\u{169}\x0a\x18\x03\x18\x03\x18\x03\x19\x03\
	\x19\x05\x19\u{16f}\x0a\x19\x03\x1a\x03\x1a\x03\x1a\x05\x1a\u{174}\x0a\x1a\
	\x03\x1a\x05\x1a\u{177}\x0a\x1a\x03\x1a\x03\x1a\x07\x1a\u{17b}\x0a\x1a\x0c\
	\x1a\x0e\x1a\u{17e}\x0b\x1a\x03\x1a\x07\x1a\u{181}\x0a\x1a\x0c\x1a\x0e\x1a\
	\u{184}\x0b\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1c\
	\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\u{193}\x0a\x1c\
	\x0c\x1c\x0e\x1c\u{196}\x0b\x1c\x03\x1c\x05\x1c\u{199}\x0a\x1c\x03\x1c\x03\
	\x1c\x03\x1c\x03\x1d\x03\x1d\x03\x1d\x05\x1d\u{1a1}\x0a\x1d\x03\x1e\x03\
	\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x07\x1e\u{1aa}\x0a\x1e\x0c\
	\x1e\x0e\x1e\u{1ad}\x0b\x1e\x03\x1e\x05\x1e\u{1b0}\x0a\x1e\x03\x1e\x03\x1e\
	\x03\x1e\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{1b8}\x0a\x1f\x03\x20\x03\x20\
	\x03\x20\x05\x20\u{1bd}\x0a\x20\x03\x20\x03\x20\x05\x20\u{1c1}\x0a\x20\x03\
	\x20\x03\x20\x07\x20\u{1c5}\x0a\x20\x0c\x20\x0e\x20\u{1c8}\x0b\x20\x03\x20\
	\x05\x20\u{1cb}\x0a\x20\x03\x20\x05\x20\u{1ce}\x0a\x20\x03\x20\x03\x20\x03\
	\x20\x03\x21\x05\x21\u{1d4}\x0a\x21\x03\x21\x03\x21\x03\x21\x05\x21\u{1d9}\
	\x0a\x21\x03\x21\x03\x21\x03\x22\x03\x22\x03\x22\x03\x23\x03\x23\x03\x23\
	\x03\x24\x03\x24\x03\x24\x03\x25\x03\x25\x03\x25\x03\x25\x06\x25\u{1ea}\
	\x0a\x25\x0d\x25\x0e\x25\u{1eb}\x03\x25\x03\x25\x03\x25\x03\x26\x03\x26\
	\x03\x26\x03\x26\x03\x27\x03\x27\x03\x27\x03\x27\x07\x27\u{1f9}\x0a\x27\
	\x0c\x27\x0e\x27\u{1fc}\x0b\x27\x03\x27\x03\x27\x03\x27\x03\x28\x03\x28\
	\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x29\x03\x29\x03\x29\x03\x29\
	\x07\x29\u{20c}\x0a\x29\x0c\x29\x0e\x29\u{20f}\x0b\x29\x03\x29\x03\x29\x03\
	\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2b\x05\x2b\u{21a}\x0a\
	\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2c\x03\x2c\x03\x2c\x03\
	\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x03\x2e\x03\x2e\x03\x2f\x03\
	\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x30\x03\x30\x03\x30\x03\x30\x07\
	\x30\u{236}\x0a\x30\x0c\x30\x0e\x30\u{239}\x0b\x30\x03\x30\x03\x30\x03\x31\
	\x03\x31\x03\x31\x03\x32\x03\x32\x03\x32\x03\x32\x07\x32\u{244}\x0a\x32\
	\x0c\x32\x0e\x32\u{247}\x0b\x32\x03\x32\x03\x32\x03\x33\x03\x33\x03\x33\
	\x03\x33\x07\x33\u{24f}\x0a\x33\x0c\x33\x0e\x33\u{252}\x0b\x33\x03\x33\x03\
	\x33\x03\x34\x03\x34\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x36\x03\
	\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\
	\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\
	\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x05\x36\u{276}\x0a\x36\x03\
	\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\
	\x36\x05\x36\u{282}\x0a\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\
	\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\
	\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\
	\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\
	\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x07\x36\u{2ab}\x0a\
	\x36\x0c\x36\x0e\x36\u{2ae}\x0b\x36\x03\x37\x03\x37\x03\x38\x03\x38\x03\
	\x39\x03\x39\x03\x39\x05\x39\u{2b7}\x0a\x39\x05\x39\u{2b9}\x0a\x39\x03\x3a\
	\x03\x3a\x03\x3a\x05\x3a\u{2be}\x0a\x3a\x03\x3b\x03\x3b\x03\x3b\x03\x3b\
	\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x05\x3b\u{2c8}\x0a\x3b\x03\x3c\x03\x3c\
	\x03\x3c\x07\x3c\u{2cd}\x0a\x3c\x0c\x3c\x0e\x3c\u{2d0}\x0b\x3c\x03\x3d\x03\
	\x3d\x03\x3d\x03\x3d\x07\x3d\u{2d6}\x0a\x3d\x0c\x3d\x0e\x3d\u{2d9}\x0b\x3d\
	\x03\x3d\x03\x3d\x03\x3e\x03\x3e\x03\x3e\x05\x3e\u{2e0}\x0a\x3e\x03\x3f\
	\x03\x3f\x03\x3f\x03\x3f\x03\x40\x03\x40\x03\x41\x03\x41\x03\x42\x03\x42\
	\x03\x42\x03\x42\x03\x43\x03\x43\x03\x44\x03\x44\x03\x45\x03\x45\x03\x46\
	\x03\x46\x03\x47\x03\x47\x03\x47\x02\x03\x6a\x48\x02\x04\x06\x08\x0a\x0c\
	\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\
	\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\
	\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\
	\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\x02\x0d\x04\x02\x42\
	\x42\x4e\x4e\x06\x02\x05\x05\x15\x15\x1a\x1a\x20\x20\x04\x02\x08\x08\x16\
	\x17\x04\x02\x15\x15\x1a\x1a\x05\x02\x0b\x0c\x0f\x0f\x14\x14\x04\x02\x0a\
	\x0a\x18\x18\x03\x02\x66\x6d\x04\x02\x37\x3a\x51\x54\x03\x02\x58\x60\x04\
	\x02\x23\x23\x36\x36\x03\x02\x2d\x2f\x02\u{316}\x02\u{8f}\x03\x02\x02\x02\
	\x04\u{9f}\x03\x02\x02\x02\x06\u{aa}\x03\x02\x02\x02\x08\u{bd}\x03\x02\x02\
	\x02\x0a\u{ca}\x03\x02\x02\x02\x0c\u{ce}\x03\x02\x02\x02\x0e\u{d0}\x03\x02\
	\x02\x02\x10\u{d7}\x03\x02\x02\x02\x12\u{e3}\x03\x02\x02\x02\x14\u{e7}\x03\
	\x02\x02\x02\x16\u{ec}\x03\x02\x02\x02\x18\u{105}\x03\x02\x02\x02\x1a\u{119}\
	\x03\x02\x02\x02\x1c\u{11f}\x03\x02\x02\x02\x1e\u{123}\x03\x02\x02\x02\x20\
	\u{12d}\x03\x02\x02\x02\x22\u{133}\x03\x02\x02\x02\x24\u{136}\x03\x02\x02\
	\x02\x26\u{139}\x03\x02\x02\x02\x28\u{13c}\x03\x02\x02\x02\x2a\u{158}\x03\
	\x02\x02\x02\x2c\u{161}\x03\x02\x02\x02\x2e\u{165}\x03\x02\x02\x02\x30\u{16c}\
	\x03\x02\x02\x02\x32\u{170}\x03\x02\x02\x02\x34\u{188}\x03\x02\x02\x02\x36\
	\u{18b}\x03\x02\x02\x02\x38\u{19d}\x03\x02\x02\x02\x3a\u{1a2}\x03\x02\x02\
	\x02\x3c\u{1b4}\x03\x02\x02\x02\x3e\u{1b9}\x03\x02\x02\x02\x40\u{1d3}\x03\
	\x02\x02\x02\x42\u{1dc}\x03\x02\x02\x02\x44\u{1df}\x03\x02\x02\x02\x46\u{1e2}\
	\x03\x02\x02\x02\x48\u{1e5}\x03\x02\x02\x02\x4a\u{1f0}\x03\x02\x02\x02\x4c\
	\u{1f4}\x03\x02\x02\x02\x4e\u{200}\x03\x02\x02\x02\x50\u{207}\x03\x02\x02\
	\x02\x52\u{213}\x03\x02\x02\x02\x54\u{219}\x03\x02\x02\x02\x56\u{220}\x03\
	\x02\x02\x02\x58\u{227}\x03\x02\x02\x02\x5a\u{229}\x03\x02\x02\x02\x5c\u{22b}\
	\x03\x02\x02\x02\x5e\u{231}\x03\x02\x02\x02\x60\u{23c}\x03\x02\x02\x02\x62\
	\u{23f}\x03\x02\x02\x02\x64\u{24a}\x03\x02\x02\x02\x66\u{255}\x03\x02\x02\
	\x02\x68\u{257}\x03\x02\x02\x02\x6a\u{275}\x03\x02\x02\x02\x6c\u{2af}\x03\
	\x02\x02\x02\x6e\u{2b1}\x03\x02\x02\x02\x70\u{2b8}\x03\x02\x02\x02\x72\u{2ba}\
	\x03\x02\x02\x02\x74\u{2c7}\x03\x02\x02\x02\x76\u{2c9}\x03\x02\x02\x02\x78\
	\u{2d1}\x03\x02\x02\x02\x7a\u{2df}\x03\x02\x02\x02\x7c\u{2e1}\x03\x02\x02\
	\x02\x7e\u{2e5}\x03\x02\x02\x02\u{80}\u{2e7}\x03\x02\x02\x02\u{82}\u{2e9}\
	\x03\x02\x02\x02\u{84}\u{2ed}\x03\x02\x02\x02\u{86}\u{2ef}\x03\x02\x02\x02\
	\u{88}\u{2f1}\x03\x02\x02\x02\u{8a}\u{2f3}\x03\x02\x02\x02\u{8c}\u{2f5}\
	\x03\x02\x02\x02\u{8e}\u{90}\x05\x04\x03\x02\u{8f}\u{8e}\x03\x02\x02\x02\
	\u{8f}\u{90}\x03\x02\x02\x02\u{90}\u{94}\x03\x02\x02\x02\u{91}\u{93}\x05\
	\x06\x04\x02\u{92}\u{91}\x03\x02\x02\x02\u{93}\u{96}\x03\x02\x02\x02\u{94}\
	\u{92}\x03\x02\x02\x02\u{94}\u{95}\x03\x02\x02\x02\u{95}\u{9a}\x03\x02\x02\
	\x02\u{96}\u{94}\x03\x02\x02\x02\u{97}\u{99}\x05\x08\x05\x02\u{98}\u{97}\
	\x03\x02\x02\x02\u{99}\u{9c}\x03\x02\x02\x02\u{9a}\u{98}\x03\x02\x02\x02\
	\u{9a}\u{9b}\x03\x02\x02\x02\u{9b}\u{9d}\x03\x02\x02\x02\u{9c}\u{9a}\x03\
	\x02\x02\x02\u{9d}\u{9e}\x07\x02\x02\x03\u{9e}\x03\x03\x02\x02\x02\u{9f}\
	\u{a0}\x07\x3f\x02\x02\u{a0}\u{a5}\x05\x6e\x38\x02\u{a1}\u{a2}\x07\x09\x02\
	\x02\u{a2}\u{a4}\x05\x6e\x38\x02\u{a3}\u{a1}\x03\x02\x02\x02\u{a4}\u{a7}\
	\x03\x02\x02\x02\u{a5}\u{a3}\x03\x02\x02\x02\u{a5}\u{a6}\x03\x02\x02\x02\
	\u{a6}\u{a8}\x03\x02\x02\x02\u{a7}\u{a5}\x03\x02\x02\x02\u{a8}\u{a9}\x07\
	\x1f\x02\x02\u{a9}\x05\x03\x02\x02\x02\u{aa}\u{ab}\x07\x33\x02\x02\u{ab}\
	\u{ac}\x05\x6e\x38\x02\u{ac}\u{b2}\x07\x09\x02\x02\u{ad}\u{ae}\x05\x6e\x38\
	\x02\u{ae}\u{af}\x07\x09\x02\x02\u{af}\u{b1}\x03\x02\x02\x02\u{b0}\u{ad}\
	\x03\x02\x02\x02\u{b1}\u{b4}\x03\x02\x02\x02\u{b2}\u{b0}\x03\x02\x02\x02\
	\u{b2}\u{b3}\x03\x02\x02\x02\u{b3}\u{b7}\x03\x02\x02\x02\u{b4}\u{b2}\x03\
	\x02\x02\x02\u{b5}\u{b8}\x05\x6e\x38\x02\u{b6}\u{b8}\x07\x17\x02\x02\u{b7}\
	\u{b5}\x03\x02\x02\x02\u{b7}\u{b6}\x03\x02\x02\x02\u{b8}\u{b9}\x03\x02\x02\
	\x02\u{b9}\u{ba}\x07\x1f\x02\x02\u{ba}\x07\x03\x02\x02\x02\u{bb}\u{be}\x05\
	\x0c\x07\x02\u{bc}\u{be}\x05\x0a\x06\x02\u{bd}\u{bb}\x03\x02\x02\x02\u{bd}\
	\u{bc}\x03\x02\x02\x02\u{be}\x09\x03\x02\x02\x02\u{bf}\u{cb}\x05\x14\x0b\
	\x02\u{c0}\u{cb}\x05\x16\x0c\x02\u{c1}\u{cb}\x05\x28\x15\x02\u{c2}\u{cb}\
	\x05\x32\x1a\x02\u{c3}\u{cb}\x05\x36\x1c\x02\u{c4}\u{cb}\x05\x3a\x1e\x02\
	\u{c5}\u{cb}\x05\x3e\x20\x02\u{c6}\u{cb}\x05\x48\x25\x02\u{c7}\u{cb}\x05\
	\x4c\x27\x02\u{c8}\u{cb}\x05\x50\x29\x02\u{c9}\u{cb}\x05\x68\x35\x02\u{ca}\
	\u{bf}\x03\x02\x02\x02\u{ca}\u{c0}\x03\x02\x02\x02\u{ca}\u{c1}\x03\x02\x02\
	\x02\u{ca}\u{c2}\x03\x02\x02\x02\u{ca}\u{c3}\x03\x02\x02\x02\u{ca}\u{c4}\
	\x03\x02\x02\x02\u{ca}\u{c5}\x03\x02\x02\x02\u{ca}\u{c6}\x03\x02\x02\x02\
	\u{ca}\u{c7}\x03\x02\x02\x02\u{ca}\u{c8}\x03\x02\x02\x02\u{ca}\u{c9}\x03\
	\x02\x02\x02\u{cb}\x0b\x03\x02\x02\x02\u{cc}\u{cf}\x05\x0e\x08\x02\u{cd}\
	\u{cf}\x05\x10\x09\x02\u{ce}\u{cc}\x03\x02\x02\x02\u{ce}\u{cd}\x03\x02\x02\
	\x02\u{cf}\x0d\x03\x02\x02\x02\u{d0}\u{d1}\x07\x28\x02\x02\u{d1}\u{d2}\x05\
	\x72\x3a\x02\u{d2}\u{d3}\x05\x6e\x38\x02\u{d3}\u{d4}\x07\x04\x02\x02\u{d4}\
	\u{d5}\x05\x6a\x36\x02\u{d5}\u{d6}\x07\x1f\x02\x02\u{d6}\x0f\x03\x02\x02\
	\x02\u{d7}\u{d8}\x07\x45\x02\x02\u{d8}\u{d9}\x05\x6e\x38\x02\u{d9}\u{dd}\
	\x07\x0d\x02\x02\u{da}\u{dc}\x05\x12\x0a\x02\u{db}\u{da}\x03\x02\x02\x02\
	\u{dc}\u{df}\x03\x02\x02\x02\u{dd}\u{db}\x03\x02\x02\x02\u{dd}\u{de}\x03\
	\x02\x02\x02\u{de}\u{e0}\x03\x02\x02\x02\u{df}\u{dd}\x03\x02\x02\x02\u{e0}\
	\u{e1}\x07\x1c\x02\x02\u{e1}\u{e2}\x07\x1f\x02\x02\u{e2}\x11\x03\x02\x02\
	\x02\u{e3}\u{e4}\x07\x44\x02\x02\u{e4}\u{e5}\x05\x6a\x36\x02\u{e5}\u{e6}\
	\x07\x1f\x02\x02\u{e6}\x13\x03\x02\x02\x02\u{e7}\u{e8}\x07\x4f\x02\x02\u{e8}\
	\u{e9}\x05\x70\x39\x02\u{e9}\u{ea}\x05\x6e\x38\x02\u{ea}\u{eb}\x07\x1f\x02\
	\x02\u{eb}\x15\x03\x02\x02\x02\u{ec}\u{ed}\x07\x4d\x02\x02\u{ed}\u{ef}\x05\
	\x6e\x38\x02\u{ee}\u{f0}\x05\x62\x32\x02\u{ef}\u{ee}\x03\x02\x02\x02\u{ef}\
	\u{f0}\x03\x02\x02\x02\u{f0}\u{f2}\x03\x02\x02\x02\u{f1}\u{f3}\x05\x5e\x30\
	\x02\u{f2}\u{f1}\x03\x02\x02\x02\u{f2}\u{f3}\x03\x02\x02\x02\u{f3}\u{f4}\
	\x03\x02\x02\x02\u{f4}\u{f8}\x07\x0d\x02\x02\u{f5}\u{f7}\x05\x18\x0d\x02\
	\u{f6}\u{f5}\x03\x02\x02\x02\u{f7}\u{fa}\x03\x02\x02\x02\u{f8}\u{f6}\x03\
	\x02\x02\x02\u{f8}\u{f9}\x03\x02\x02\x02\u{f9}\u{fe}\x03\x02\x02\x02\u{fa}\
	\u{f8}\x03\x02\x02\x02\u{fb}\u{fd}\x05\x56\x2c\x02\u{fc}\u{fb}\x03\x02\x02\
	\x02\u{fd}\u{100}\x03\x02\x02\x02\u{fe}\u{fc}\x03\x02\x02\x02\u{fe}\u{ff}\
	\x03\x02\x02\x02\u{ff}\u{101}\x03\x02\x02\x02\u{100}\u{fe}\x03\x02\x02\x02\
	\u{101}\u{102}\x07\x1c\x02\x02\u{102}\u{103}\x07\x1f\x02\x02\u{103}\x17\
	\x03\x02\x02\x02\u{104}\u{106}\x05\x1a\x0e\x02\u{105}\u{104}\x03\x02\x02\
	\x02\u{105}\u{106}\x03\x02\x02\x02\u{106}\u{108}\x03\x02\x02\x02\u{107}\
	\u{109}\x05\x1c\x0f\x02\u{108}\u{107}\x03\x02\x02\x02\u{108}\u{109}\x03\
	\x02\x02\x02\u{109}\u{10b}\x03\x02\x02\x02\u{10a}\u{10c}\x07\x3e\x02\x02\
	\u{10b}\u{10a}\x03\x02\x02\x02\u{10b}\u{10c}\x03\x02\x02\x02\u{10c}\u{10d}\
	\x03\x02\x02\x02\u{10d}\u{10f}\x05\x1e\x10\x02\u{10e}\u{110}\x05\x22\x12\
	\x02\u{10f}\u{10e}\x03\x02\x02\x02\u{10f}\u{110}\x03\x02\x02\x02\u{110}\
	\u{112}\x03\x02\x02\x02\u{111}\u{113}\x05\x24\x13\x02\u{112}\u{111}\x03\
	\x02\x02\x02\u{112}\u{113}\x03\x02\x02\x02\u{113}\u{115}\x03\x02\x02\x02\
	\u{114}\u{116}\x05\x26\x14\x02\u{115}\u{114}\x03\x02\x02\x02\u{115}\u{116}\
	\x03\x02\x02\x02\u{116}\u{117}\x03\x02\x02\x02\u{117}\u{118}\x07\x1f\x02\
	\x02\u{118}\x19\x03\x02\x02\x02\u{119}\u{11a}\x07\x22\x02\x02\u{11a}\u{11b}\
	\x07\x12\x02\x02\u{11b}\u{11c}\x05\x6a\x36\x02\u{11c}\u{11d}\x07\x1e\x02\
	\x02\u{11d}\u{11e}\x07\x06\x02\x02\u{11e}\x1b\x03\x02\x02\x02\u{11f}\u{120}\
	\x05\x6a\x36\x02\u{120}\u{121}\x07\x06\x02\x02\u{121}\x1d\x03\x02\x02\x02\
	\u{122}\u{124}\x07\x40\x02\x02\u{123}\u{122}\x03\x02\x02\x02\u{123}\u{124}\
	\x03\x02\x02\x02\u{124}\u{126}\x03\x02\x02\x02\u{125}\u{127}\x07\x32\x02\
	\x02\u{126}\u{125}\x03\x02\x02\x02\u{126}\u{127}\x03\x02\x02\x02\u{127}\
	\u{128}\x03\x02\x02\x02\u{128}\u{129}\x05\x72\x3a\x02\u{129}\u{12b}\x05\
	\x6e\x38\x02\u{12a}\u{12c}\x05\x20\x11\x02\u{12b}\u{12a}\x03\x02\x02\x02\
	\u{12b}\u{12c}\x03\x02\x02\x02\u{12c}\x1f\x03\x02\x02\x02\u{12d}\u{12f}\
	\x07\x0e\x02\x02\u{12e}\u{130}\x05\x6a\x36\x02\u{12f}\u{12e}\x03\x02\x02\
	\x02\u{12f}\u{130}\x03\x02\x02\x02\u{130}\u{131}\x03\x02\x02\x02\u{131}\
	\u{132}\x07\x1d\x02\x02\u{132}\x21\x03\x02\x02\x02\u{133}\u{134}\x07\x04\
	\x02\x02\u{134}\u{135}\x05\x6a\x36\x02\u{135}\x23\x03\x02\x02\x02\u{136}\
	\u{137}\x07\x31\x02\x02\u{137}\u{138}\x05\x6a\x36\x02\u{138}\x25\x03\x02\
	\x02\x02\u{139}\u{13a}\x07\x06\x02\x02\u{13a}\u{13b}\x05\x6a\x36\x02\u{13b}\
	\x27\x03\x02\x02\x02\u{13c}\u{13d}\x07\x27\x02\x02\u{13d}\u{13f}\x05\x6e\
	\x38\x02\u{13e}\u{140}\x05\x62\x32\x02\u{13f}\u{13e}\x03\x02\x02\x02\u{13f}\
	\u{140}\x03\x02\x02\x02\u{140}\u{141}\x03\x02\x02\x02\u{141}\u{142}\x05\
	\x5e\x30\x02\u{142}\u{143}\x07\x3d\x02\x02\u{143}\u{144}\x05\x6a\x36\x02\
	\u{144}\u{148}\x07\x0d\x02\x02\u{145}\u{147}\x05\x2a\x16\x02\u{146}\u{145}\
	\x03\x02\x02\x02\u{147}\u{14a}\x03\x02\x02\x02\u{148}\u{146}\x03\x02\x02\
	\x02\u{148}\u{149}\x03\x02\x02\x02\u{149}\u{14c}\x03\x02\x02\x02\u{14a}\
	\u{148}\x03\x02\x02\x02\u{14b}\u{14d}\x05\x2e\x18\x02\u{14c}\u{14b}\x03\
	\x02\x02\x02\u{14c}\u{14d}\x03\x02\x02\x02\u{14d}\u{151}\x03\x02\x02\x02\
	\u{14e}\u{150}\x05\x56\x2c\x02\u{14f}\u{14e}\x03\x02\x02\x02\u{150}\u{153}\
	\x03\x02\x02\x02\u{151}\u{14f}\x03\x02\x02\x02\u{151}\u{152}\x03\x02\x02\
	\x02\u{152}\u{154}\x03\x02\x02\x02\u{153}\u{151}\x03\x02\x02\x02\u{154}\
	\u{155}\x07\x1c\x02\x02\u{155}\u{156}\x07\x1f\x02\x02\u{156}\x29\x03\x02\
	\x02\x02\u{157}\u{159}\x05\x2c\x17\x02\u{158}\u{157}\x03\x02\x02\x02\u{159}\
	\u{15a}\x03\x02\x02\x02\u{15a}\u{158}\x03\x02\x02\x02\u{15a}\u{15b}\x03\
	\x02\x02\x02\u{15b}\u{15d}\x03\x02\x02\x02\u{15c}\u{15e}\x05\x30\x19\x02\
	\u{15d}\u{15c}\x03\x02\x02\x02\u{15d}\u{15e}\x03\x02\x02\x02\u{15e}\u{15f}\
	\x03\x02\x02\x02\u{15f}\u{160}\x07\x1f\x02\x02\u{160}\x2b\x03\x02\x02\x02\
	\u{161}\u{162}\x07\x26\x02\x02\u{162}\u{163}\x05\x6a\x36\x02\u{163}\u{164}\
	\x07\x06\x02\x02\u{164}\x2d\x03\x02\x02\x02\u{165}\u{166}\x07\x29\x02\x02\
	\u{166}\u{168}\x07\x06\x02\x02\u{167}\u{169}\x05\x30\x19\x02\u{168}\u{167}\
	\x03\x02\x02\x02\u{168}\u{169}\x03\x02\x02\x02\u{169}\u{16a}\x03\x02\x02\
	\x02\u{16a}\u{16b}\x07\x1f\x02\x02\u{16b}\x2f\x03\x02\x02\x02\u{16c}\u{16e}\
	\x05\x1e\x10\x02\u{16d}\u{16f}\x05\x26\x14\x02\u{16e}\u{16d}\x03\x02\x02\
	\x02\u{16e}\u{16f}\x03\x02\x02\x02\u{16f}\x31\x03\x02\x02\x02\u{170}\u{171}\
	\x07\x55\x02\x02\u{171}\u{173}\x05\x6e\x38\x02\u{172}\u{174}\x05\x62\x32\
	\x02\u{173}\u{172}\x03\x02\x02\x02\u{173}\u{174}\x03\x02\x02\x02\u{174}\
	\u{176}\x03\x02\x02\x02\u{175}\u{177}\x05\x5e\x30\x02\u{176}\u{175}\x03\
	\x02\x02\x02\u{176}\u{177}\x03\x02\x02\x02\u{177}\u{178}\x03\x02\x02\x02\
	\u{178}\u{17c}\x07\x0d\x02\x02\u{179}\u{17b}\x05\x34\x1b\x02\u{17a}\u{179}\
	\x03\x02\x02\x02\u{17b}\u{17e}\x03\x02\x02\x02\u{17c}\u{17a}\x03\x02\x02\
	\x02\u{17c}\u{17d}\x03\x02\x02\x02\u{17d}\u{182}\x03\x02\x02\x02\u{17e}\
	\u{17c}\x03\x02\x02\x02\u{17f}\u{181}\x05\x56\x2c\x02\u{180}\u{17f}\x03\
	\x02\x02\x02\u{181}\u{184}\x03\x02\x02\x02\u{182}\u{180}\x03\x02\x02\x02\
	\u{182}\u{183}\x03\x02\x02\x02\u{183}\u{185}\x03\x02\x02\x02\u{184}\u{182}\
	\x03\x02\x02\x02\u{185}\u{186}\x07\x1c\x02\x02\u{186}\u{187}\x07\x1f\x02\
	\x02\u{187}\x33\x03\x02\x02\x02\u{188}\u{189}\x05\x30\x19\x02\u{189}\u{18a}\
	\x07\x1f\x02\x02\u{18a}\x35\x03\x02\x02\x02\u{18b}\u{18c}\x07\x2a\x02\x02\
	\u{18c}\u{18d}\x05\x72\x3a\x02\u{18d}\u{18e}\x05\x6e\x38\x02\u{18e}\u{18f}\
	\x07\x0d\x02\x02\u{18f}\u{194}\x05\x38\x1d\x02\u{190}\u{191}\x07\x07\x02\
	\x02\u{191}\u{193}\x05\x38\x1d\x02\u{192}\u{190}\x03\x02\x02\x02\u{193}\
	\u{196}\x03\x02\x02\x02\u{194}\u{192}\x03\x02\x02\x02\u{194}\u{195}\x03\
	\x02\x02\x02\u{195}\u{198}\x03\x02\x02\x02\u{196}\u{194}\x03\x02\x02\x02\
	\u{197}\u{199}\x07\x07\x02\x02\u{198}\u{197}\x03\x02\x02\x02\u{198}\u{199}\
	\x03\x02\x02\x02\u{199}\u{19a}\x03\x02\x02\x02\u{19a}\u{19b}\x07\x1c\x02\
	\x02\u{19b}\u{19c}\x07\x1f\x02\x02\u{19c}\x37\x03\x02\x02\x02\u{19d}\u{1a0}\
	\x05\x6e\x38\x02\u{19e}\u{19f}\x07\x04\x02\x02\u{19f}\u{1a1}\x05\x6a\x36\
	\x02\u{1a0}\u{19e}\x03\x02\x02\x02\u{1a0}\u{1a1}\x03\x02\x02\x02\u{1a1}\
	\x39\x03\x02\x02\x02\u{1a2}\u{1a3}\x07\x25\x02\x02\u{1a3}\u{1a4}\x05\x72\
	\x3a\x02\u{1a4}\u{1a5}\x05\x6e\x38\x02\u{1a5}\u{1a6}\x07\x0d\x02\x02\u{1a6}\
	\u{1ab}\x05\x3c\x1f\x02\u{1a7}\u{1a8}\x07\x07\x02\x02\u{1a8}\u{1aa}\x05\
	\x3c\x1f\x02\u{1a9}\u{1a7}\x03\x02\x02\x02\u{1aa}\u{1ad}\x03\x02\x02\x02\
	\u{1ab}\u{1a9}\x03\x02\x02\x02\u{1ab}\u{1ac}\x03\x02\x02\x02\u{1ac}\u{1af}\
	\x03\x02\x02\x02\u{1ad}\u{1ab}\x03\x02\x02\x02\u{1ae}\u{1b0}\x07\x07\x02\
	\x02\u{1af}\u{1ae}\x03\x02\x02\x02\u{1af}\u{1b0}\x03\x02\x02\x02\u{1b0}\
	\u{1b1}\x03\x02\x02\x02\u{1b1}\u{1b2}\x07\x1c\x02\x02\u{1b2}\u{1b3}\x07\
	\x1f\x02\x02\u{1b3}\x3b\x03\x02\x02\x02\u{1b4}\u{1b7}\x05\x6e\x38\x02\u{1b5}\
	\u{1b6}\x07\x04\x02\x02\u{1b6}\u{1b8}\x05\x6a\x36\x02\u{1b7}\u{1b5}\x03\
	\x02\x02\x02\u{1b7}\u{1b8}\x03\x02\x02\x02\u{1b8}\x3d\x03\x02\x02\x02\u{1b9}\
	\u{1ba}\x07\x49\x02\x02\u{1ba}\u{1bc}\x05\x6e\x38\x02\u{1bb}\u{1bd}\x05\
	\x62\x32\x02\u{1bc}\u{1bb}\x03\x02\x02\x02\u{1bc}\u{1bd}\x03\x02\x02\x02\
	\u{1bd}\u{1c0}\x03\x02\x02\x02\u{1be}\u{1bf}\x07\x56\x02\x02\u{1bf}\u{1c1}\
	\x05\x6e\x38\x02\u{1c0}\u{1be}\x03\x02\x02\x02\u{1c0}\u{1c1}\x03\x02\x02\
	\x02\u{1c1}\u{1c2}\x03\x02\x02\x02\u{1c2}\u{1c6}\x07\x0d\x02\x02\u{1c3}\
	\u{1c5}\x05\x40\x21\x02\u{1c4}\u{1c3}\x03\x02\x02\x02\u{1c5}\u{1c8}\x03\
	\x02\x02\x02\u{1c6}\u{1c4}\x03\x02\x02\x02\u{1c6}\u{1c7}\x03\x02\x02\x02\
	\u{1c7}\u{1ca}\x03\x02\x02\x02\u{1c8}\u{1c6}\x03\x02\x02\x02\u{1c9}\u{1cb}\
	\x05\x42\x22\x02\u{1ca}\u{1c9}\x03\x02\x02\x02\u{1ca}\u{1cb}\x03\x02\x02\
	\x02\u{1cb}\u{1cd}\x03\x02\x02\x02\u{1cc}\u{1ce}\x05\x46\x24\x02\u{1cd}\
	\u{1cc}\x03\x02\x02\x02\u{1cd}\u{1ce}\x03\x02\x02\x02\u{1ce}\u{1cf}\x03\
	\x02\x02\x02\u{1cf}\u{1d0}\x07\x1c\x02\x02\u{1d0}\u{1d1}\x07\x1f\x02\x02\
	\u{1d1}\x3f\x03\x02\x02\x02\u{1d2}\u{1d4}\x07\x4a\x02\x02\u{1d3}\u{1d2}\
	\x03\x02\x02\x02\u{1d3}\u{1d4}\x03\x02\x02\x02\u{1d4}\u{1d5}\x03\x02\x02\
	\x02\u{1d5}\u{1d6}\x05\x72\x3a\x02\u{1d6}\u{1d8}\x05\x6e\x38\x02\u{1d7}\
	\u{1d9}\x05\x44\x23\x02\u{1d8}\u{1d7}\x03\x02\x02\x02\u{1d8}\u{1d9}\x03\
	\x02\x02\x02\u{1d9}\u{1da}\x03\x02\x02\x02\u{1da}\u{1db}\x07\x1f\x02\x02\
	\u{1db}\x41\x03\x02\x02\x02\u{1dc}\u{1dd}\x05\x44\x23\x02\u{1dd}\u{1de}\
	\x07\x1f\x02\x02\u{1de}\x43\x03\x02\x02\x02\u{1df}\u{1e0}\x07\x47\x02\x02\
	\u{1e0}\u{1e1}\x05\x6a\x36\x02\u{1e1}\x45\x03\x02\x02\x02\u{1e2}\u{1e3}\
	\x07\x4b\x02\x02\u{1e3}\u{1e4}\x07\x1f\x02\x02\u{1e4}\x47\x03\x02\x02\x02\
	\u{1e5}\u{1e6}\x07\x48\x02\x02\u{1e6}\u{1e7}\x05\x6e\x38\x02\u{1e7}\u{1e9}\
	\x07\x0d\x02\x02\u{1e8}\u{1ea}\x05\x4a\x26\x02\u{1e9}\u{1e8}\x03\x02\x02\
	\x02\u{1ea}\u{1eb}\x03\x02\x02\x02\u{1eb}\u{1e9}\x03\x02\x02\x02\u{1eb}\
	\u{1ec}\x03\x02\x02\x02\u{1ec}\u{1ed}\x03\x02\x02\x02\u{1ed}\u{1ee}\x07\
	\x1c\x02\x02\u{1ee}\u{1ef}\x07\x1f\x02\x02\u{1ef}\x49\x03\x02\x02\x02\u{1f0}\
	\u{1f1}\x05\x72\x3a\x02\u{1f1}\u{1f2}\x05\x6e\x38\x02\u{1f2}\u{1f3}\x07\
	\x1f\x02\x02\u{1f3}\x4b\x03\x02\x02\x02\u{1f4}\u{1f5}\x07\x46\x02\x02\u{1f5}\
	\u{1f6}\x05\x6e\x38\x02\u{1f6}\u{1fa}\x07\x0d\x02\x02\u{1f7}\u{1f9}\x05\
	\x4e\x28\x02\u{1f8}\u{1f7}\x03\x02\x02\x02\u{1f9}\u{1fc}\x03\x02\x02\x02\
	\u{1fa}\u{1f8}\x03\x02\x02\x02\u{1fa}\u{1fb}\x03\x02\x02\x02\u{1fb}\u{1fd}\
	\x03\x02\x02\x02\u{1fc}\u{1fa}\x03\x02\x02\x02\u{1fd}\u{1fe}\x07\x1c\x02\
	\x02\u{1fe}\u{1ff}\x07\x1f\x02\x02\u{1ff}\x4d\x03\x02\x02\x02\u{200}\u{201}\
	\x05\x70\x39\x02\u{201}\u{202}\x05\x6e\x38\x02\u{202}\u{203}\x07\x12\x02\
	\x02\u{203}\u{204}\x05\x70\x39\x02\u{204}\u{205}\x07\x1e\x02\x02\u{205}\
	\u{206}\x07\x1f\x02\x02\u{206}\x4f\x03\x02\x02\x02\u{207}\u{208}\x07\x41\
	\x02\x02\u{208}\u{209}\x05\x6e\x38\x02\u{209}\u{20d}\x07\x0d\x02\x02\u{20a}\
	\u{20c}\x05\x52\x2a\x02\u{20b}\u{20a}\x03\x02\x02\x02\u{20c}\u{20f}\x03\
	\x02\x02\x02\u{20d}\u{20b}\x03\x02\x02\x02\u{20d}\u{20e}\x03\x02\x02\x02\
	\u{20e}\u{210}\x03\x02\x02\x02\u{20f}\u{20d}\x03\x02\x02\x02\u{210}\u{211}\
	\x07\x1c\x02\x02\u{211}\u{212}\x07\x1f\x02\x02\u{212}\x51\x03\x02\x02\x02\
	\u{213}\u{214}\x05\x54\x2b\x02\u{214}\u{215}\x05\x70\x39\x02\u{215}\u{216}\
	\x05\x6e\x38\x02\u{216}\u{217}\x07\x1f\x02\x02\u{217}\x53\x03\x02\x02\x02\
	\u{218}\u{21a}\x09\x02\x02\x02\u{219}\u{218}\x03\x02\x02\x02\u{219}\u{21a}\
	\x03\x02\x02\x02\u{21a}\u{21b}\x03\x02\x02\x02\u{21b}\u{21c}\x07\x50\x02\
	\x02\u{21c}\u{21d}\x07\x12\x02\x02\u{21d}\u{21e}\x05\x6a\x36\x02\u{21e}\
	\u{21f}\x07\x1e\x02\x02\u{21f}\x55\x03\x02\x02\x02\u{220}\u{221}\x07\x30\
	\x02\x02\u{221}\u{222}\x05\x58\x2d\x02\u{222}\u{223}\x05\x5a\x2e\x02\u{223}\
	\u{224}\x07\x12\x02\x02\u{224}\u{225}\x07\x1e\x02\x02\u{225}\u{226}\x05\
	\x5c\x2f\x02\u{226}\x57\x03\x02\x02\x02\u{227}\u{228}\x05\x70\x39\x02\u{228}\
	\x59\x03\x02\x02\x02\u{229}\u{22a}\x05\x6e\x38\x02\u{22a}\x5b\x03\x02\x02\
	\x02\u{22b}\u{22c}\x07\x0d\x02\x02\u{22c}\u{22d}\x07\x43\x02\x02\u{22d}\
	\u{22e}\x05\x6a\x36\x02\u{22e}\u{22f}\x07\x1f\x02\x02\u{22f}\u{230}\x07\
	\x1c\x02\x02\u{230}\x5d\x03\x02\x02\x02\u{231}\u{232}\x07\x12\x02\x02\u{232}\
	\u{237}\x05\x60\x31\x02\u{233}\u{234}\x07\x07\x02\x02\u{234}\u{236}\x05\
	\x60\x31\x02\u{235}\u{233}\x03\x02\x02\x02\u{236}\u{239}\x03\x02\x02\x02\
	\u{237}\u{235}\x03\x02\x02\x02\u{237}\u{238}\x03\x02\x02\x02\u{238}\u{23a}\
	\x03\x02\x02\x02\u{239}\u{237}\x03\x02\x02\x02\u{23a}\u{23b}\x07\x1e\x02\
	\x02\u{23b}\x5f\x03\x02\x02\x02\u{23c}\u{23d}\x05\x70\x39\x02\u{23d}\u{23e}\
	\x05\x6e\x38\x02\u{23e}\x61\x03\x02\x02\x02\u{23f}\u{240}\x07\x14\x02\x02\
	\u{240}\u{245}\x05\x6e\x38\x02\u{241}\u{242}\x07\x07\x02\x02\u{242}\u{244}\
	\x05\x6e\x38\x02\u{243}\u{241}\x03\x02\x02\x02\u{244}\u{247}\x03\x02\x02\
	\x02\u{245}\u{243}\x03\x02\x02\x02\u{245}\u{246}\x03\x02\x02\x02\u{246}\
	\u{248}\x03\x02\x02\x02\u{247}\u{245}\x03\x02\x02\x02\u{248}\u{249}\x07\
	\x0c\x02\x02\u{249}\x63\x03\x02\x02\x02\u{24a}\u{24b}\x07\x14\x02\x02\u{24b}\
	\u{250}\x05\x66\x34\x02\u{24c}\u{24d}\x07\x07\x02\x02\u{24d}\u{24f}\x05\
	\x66\x34\x02\u{24e}\u{24c}\x03\x02\x02\x02\u{24f}\u{252}\x03\x02\x02\x02\
	\u{250}\u{24e}\x03\x02\x02\x02\u{250}\u{251}\x03\x02\x02\x02\u{251}\u{253}\
	\x03\x02\x02\x02\u{252}\u{250}\x03\x02\x02\x02\u{253}\u{254}\x07\x0c\x02\
	\x02\u{254}\x65\x03\x02\x02\x02\u{255}\u{256}\x05\x70\x39\x02\u{256}\x67\
	\x03\x02\x02\x02\u{257}\u{258}\x07\x35\x02\x02\u{258}\u{259}\x05\x70\x39\
	\x02\u{259}\u{25a}\x05\x6e\x38\x02\u{25a}\u{25b}\x07\x1f\x02\x02\u{25b}\
	\x69\x03\x02\x02\x02\u{25c}\u{25d}\x08\x36\x01\x02\u{25d}\u{25e}\x07\x12\
	\x02\x02\u{25e}\u{25f}\x05\x6a\x36\x02\u{25f}\u{260}\x07\x1e\x02\x02\u{260}\
	\u{276}\x03\x02\x02\x02\u{261}\u{262}\x07\x3b\x02\x02\u{262}\u{263}\x07\
	\x12\x02\x02\u{263}\u{264}\x05\x6a\x36\x02\u{264}\u{265}\x07\x1e\x02\x02\
	\u{265}\u{276}\x03\x02\x02\x02\u{266}\u{267}\x07\x57\x02\x02\u{267}\u{268}\
	\x07\x12\x02\x02\u{268}\u{269}\x05\x6a\x36\x02\u{269}\u{26a}\x07\x1e\x02\
	\x02\u{26a}\u{276}\x03\x02\x02\x02\u{26b}\u{26c}\x07\x3c\x02\x02\u{26c}\
	\u{26d}\x07\x12\x02\x02\u{26d}\u{26e}\x05\x6a\x36\x02\u{26e}\u{26f}\x07\
	\x1e\x02\x02\u{26f}\u{276}\x03\x02\x02\x02\u{270}\u{271}\x09\x03\x02\x02\
	\u{271}\u{276}\x05\x6a\x36\x11\u{272}\u{276}\x05\x6c\x37\x02\u{273}\u{276}\
	\x07\x34\x02\x02\u{274}\u{276}\x05\x6e\x38\x02\u{275}\u{25c}\x03\x02\x02\
	\x02\u{275}\u{261}\x03\x02\x02\x02\u{275}\u{266}\x03\x02\x02\x02\u{275}\
	\u{26b}\x03\x02\x02\x02\u{275}\u{270}\x03\x02\x02\x02\u{275}\u{272}\x03\
	\x02\x02\x02\u{275}\u{273}\x03\x02\x02\x02\u{275}\u{274}\x03\x02\x02\x02\
	\u{276}\u{2ac}\x03\x02\x02\x02\u{277}\u{278}\x0c\x10\x02\x02\u{278}\u{279}\
	\x09\x04\x02\x02\u{279}\u{2ab}\x05\x6a\x36\x11\u{27a}\u{27b}\x0c\x0f\x02\
	\x02\u{27b}\u{27c}\x09\x05\x02\x02\u{27c}\u{2ab}\x05\x6a\x36\x10\u{27d}\
	\u{281}\x0c\x0e\x02\x02\u{27e}\u{282}\x07\x13\x02\x02\u{27f}\u{280}\x07\
	\x0c\x02\x02\u{280}\u{282}\x07\x0c\x02\x02\u{281}\u{27e}\x03\x02\x02\x02\
	\u{281}\u{27f}\x03\x02\x02\x02\u{282}\u{283}\x03\x02\x02\x02\u{283}\u{2ab}\
	\x05\x6a\x36\x0f\u{284}\u{285}\x0c\x0d\x02\x02\u{285}\u{286}\x09\x06\x02\
	\x02\u{286}\u{2ab}\x05\x6a\x36\x0e\u{287}\u{288}\x0c\x0c\x02\x02\u{288}\
	\u{289}\x09\x07\x02\x02\u{289}\u{2ab}\x05\x6a\x36\x0d\u{28a}\u{28b}\x0c\
	\x0b\x02\x02\u{28b}\u{28c}\x07\x03\x02\x02\u{28c}\u{2ab}\x05\x6a\x36\x0c\
	\u{28d}\u{28e}\x0c\x0a\x02\x02\u{28e}\u{28f}\x07\x21\x02\x02\u{28f}\u{2ab}\
	\x05\x6a\x36\x0b\u{290}\u{291}\x0c\x09\x02\x02\u{291}\u{292}\x07\x19\x02\
	\x02\u{292}\u{2ab}\x05\x6a\x36\x0a\u{293}\u{294}\x0c\x08\x02\x02\u{294}\
	\u{295}\x07\x10\x02\x02\u{295}\u{2ab}\x05\x6a\x36\x09\u{296}\u{297}\x0c\
	\x07\x02\x02\u{297}\u{298}\x07\x11\x02\x02\u{298}\u{2ab}\x05\x6a\x36\x08\
	\u{299}\u{29a}\x0c\x06\x02\x02\u{29a}\u{29b}\x07\x1b\x02\x02\u{29b}\u{29c}\
	\x05\x6a\x36\x02\u{29c}\u{29d}\x07\x06\x02\x02\u{29d}\u{29e}\x05\x6a\x36\
	\x06\u{29e}\u{2ab}\x03\x02\x02\x02\u{29f}\u{2a0}\x0c\x17\x02\x02\u{2a0}\
	\u{2a1}\x07\x12\x02\x02\u{2a1}\u{2ab}\x07\x1e\x02\x02\u{2a2}\u{2a3}\x0c\
	\x16\x02\x02\u{2a3}\u{2a4}\x07\x0e\x02\x02\u{2a4}\u{2a5}\x05\x6a\x36\x02\
	\u{2a5}\u{2a6}\x07\x1d\x02\x02\u{2a6}\u{2ab}\x03\x02\x02\x02\u{2a7}\u{2a8}\
	\x0c\x15\x02\x02\u{2a8}\u{2a9}\x07\x09\x02\x02\u{2a9}\u{2ab}\x05\x6e\x38\
	\x02\u{2aa}\u{277}\x03\x02\x02\x02\u{2aa}\u{27a}\x03\x02\x02\x02\u{2aa}\
	\u{27d}\x03\x02\x02\x02\u{2aa}\u{284}\x03\x02\x02\x02\u{2aa}\u{287}\x03\
	\x02\x02\x02\u{2aa}\u{28a}\x03\x02\x02\x02\u{2aa}\u{28d}\x03\x02\x02\x02\
	\u{2aa}\u{290}\x03\x02\x02\x02\u{2aa}\u{293}\x03\x02\x02\x02\u{2aa}\u{296}\
	\x03\x02\x02\x02\u{2aa}\u{299}\x03\x02\x02\x02\u{2aa}\u{29f}\x03\x02\x02\
	\x02\u{2aa}\u{2a2}\x03\x02\x02\x02\u{2aa}\u{2a7}\x03\x02\x02\x02\u{2ab}\
	\u{2ae}\x03\x02\x02\x02\u{2ac}\u{2aa}\x03\x02\x02\x02\u{2ac}\u{2ad}\x03\
	\x02\x02\x02\u{2ad}\x6b\x03\x02\x02\x02\u{2ae}\u{2ac}\x03\x02\x02\x02\u{2af}\
	\u{2b0}\x09\x08\x02\x02\u{2b0}\x6d\x03\x02\x02\x02\u{2b1}\u{2b2}\x07\x6e\
	\x02\x02\u{2b2}\x6f\x03\x02\x02\x02\u{2b3}\u{2b9}\x05\x74\x3b\x02\u{2b4}\
	\u{2b6}\x05\x76\x3c\x02\u{2b5}\u{2b7}\x05\x64\x33\x02\u{2b6}\u{2b5}\x03\
	\x02\x02\x02\u{2b6}\u{2b7}\x03\x02\x02\x02\u{2b7}\u{2b9}\x03\x02\x02\x02\
	\u{2b8}\u{2b3}\x03\x02\x02\x02\u{2b8}\u{2b4}\x03\x02\x02\x02\u{2b9}\x71\
	\x03\x02\x02\x02\u{2ba}\u{2bd}\x05\x70\x39\x02\u{2bb}\u{2be}\x05\x78\x3d\
	\x02\u{2bc}\u{2be}\x05\x7c\x3f\x02\u{2bd}\u{2bb}\x03\x02\x02\x02\u{2bd}\
	\u{2bc}\x03\x02\x02\x02\u{2bd}\u{2be}\x03\x02\x02\x02\u{2be}\x73\x03\x02\
	\x02\x02\u{2bf}\u{2c8}\x05\x7e\x40\x02\u{2c0}\u{2c8}\x05\u{80}\x41\x02\u{2c1}\
	\u{2c8}\x05\u{82}\x42\x02\u{2c2}\u{2c8}\x05\u{84}\x43\x02\u{2c3}\u{2c8}\
	\x05\u{86}\x44\x02\u{2c4}\u{2c8}\x05\u{88}\x45\x02\u{2c5}\u{2c8}\x05\u{8a}\
	\x46\x02\u{2c6}\u{2c8}\x05\u{8c}\x47\x02\u{2c7}\u{2bf}\x03\x02\x02\x02\u{2c7}\
	\u{2c0}\x03\x02\x02\x02\u{2c7}\u{2c1}\x03\x02\x02\x02\u{2c7}\u{2c2}\x03\
	\x02\x02\x02\u{2c7}\u{2c3}\x03\x02\x02\x02\u{2c7}\u{2c4}\x03\x02\x02\x02\
	\u{2c7}\u{2c5}\x03\x02\x02\x02\u{2c7}\u{2c6}\x03\x02\x02\x02\u{2c8}\x75\
	\x03\x02\x02\x02\u{2c9}\u{2ce}\x05\x6e\x38\x02\u{2ca}\u{2cb}\x07\x09\x02\
	\x02\u{2cb}\u{2cd}\x05\x6e\x38\x02\u{2cc}\u{2ca}\x03\x02\x02\x02\u{2cd}\
	\u{2d0}\x03\x02\x02\x02\u{2ce}\u{2cc}\x03\x02\x02\x02\u{2ce}\u{2cf}\x03\
	\x02\x02\x02\u{2cf}\x77\x03\x02\x02\x02\u{2d0}\u{2ce}\x03\x02\x02\x02\u{2d1}\
	\u{2d2}\x07\x12\x02\x02\u{2d2}\u{2d7}\x05\x7a\x3e\x02\u{2d3}\u{2d4}\x07\
	\x07\x02\x02\u{2d4}\u{2d6}\x05\x7a\x3e\x02\u{2d5}\u{2d3}\x03\x02\x02\x02\
	\u{2d6}\u{2d9}\x03\x02\x02\x02\u{2d7}\u{2d5}\x03\x02\x02\x02\u{2d7}\u{2d8}\
	\x03\x02\x02\x02\u{2d8}\u{2da}\x03\x02\x02\x02\u{2d9}\u{2d7}\x03\x02\x02\
	\x02\u{2da}\u{2db}\x07\x1e\x02\x02\u{2db}\x79\x03\x02\x02\x02\u{2dc}\u{2dd}\
	\x07\x2b\x02\x02\u{2dd}\u{2e0}\x05\x6e\x38\x02\u{2de}\u{2e0}\x05\x6a\x36\
	\x02\u{2df}\u{2dc}\x03\x02\x02\x02\u{2df}\u{2de}\x03\x02\x02\x02\u{2e0}\
	\x7b\x03\x02\x02\x02\u{2e1}\u{2e2}\x07\x14\x02\x02\u{2e2}\u{2e3}\x05\x6a\
	\x36\x02\u{2e3}\u{2e4}\x07\x0c\x02\x02\u{2e4}\x7d\x03\x02\x02\x02\u{2e5}\
	\u{2e6}\x09\x09\x02\x02\u{2e6}\x7f\x03\x02\x02\x02\u{2e7}\u{2e8}\x09\x0a\
	\x02\x02\u{2e8}\u{81}\x03\x02\x02\x02\u{2e9}\u{2ea}\x09\x0b\x02\x02\u{2ea}\
	\u{2eb}\x07\x06\x02\x02\u{2eb}\u{2ec}\x07\x6d\x02\x02\u{2ec}\u{83}\x03\x02\
	\x02\x02\u{2ed}\u{2ee}\x09\x0b\x02\x02\u{2ee}\u{85}\x03\x02\x02\x02\u{2ef}\
	\u{2f0}\x07\x24\x02\x02\u{2f0}\u{87}\x03\x02\x02\x02\u{2f1}\u{2f2}\x07\x4c\
	\x02\x02\u{2f2}\u{89}\x03\x02\x02\x02\u{2f3}\u{2f4}\x09\x0c\x02\x02\u{2f4}\
	\u{8b}\x03\x02\x02\x02\u{2f5}\u{2f6}\x07\x2c\x02\x02\u{2f6}\u{8d}\x03\x02\
	\x02\x02\x45\u{8f}\u{94}\u{9a}\u{a5}\u{b2}\u{b7}\u{bd}\u{ca}\u{ce}\u{dd}\
	\u{ef}\u{f2}\u{f8}\u{fe}\u{105}\u{108}\u{10b}\u{10f}\u{112}\u{115}\u{123}\
	\u{126}\u{12b}\u{12f}\u{13f}\u{148}\u{14c}\u{151}\u{15a}\u{15d}\u{168}\u{16e}\
	\u{173}\u{176}\u{17c}\u{182}\u{194}\u{198}\u{1a0}\u{1ab}\u{1af}\u{1b7}\u{1bc}\
	\u{1c0}\u{1c6}\u{1ca}\u{1cd}\u{1d3}\u{1d8}\u{1eb}\u{1fa}\u{20d}\u{219}\u{237}\
	\u{245}\u{250}\u{275}\u{281}\u{2aa}\u{2ac}\u{2b6}\u{2b8}\u{2bd}\u{2c7}\u{2ce}\
	\u{2d7}\u{2df}";
