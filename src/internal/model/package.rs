use crate::internal::ast::package::ZPackage;
use crate::internal::parser::gen::zseriolexer::ZserioLexer;
use crate::internal::parser::gen::zserioparser::ZserioParser;
use crate::internal::visitor::visitor::Visitor;
use crate::internal::visitor::visitor::ZserioTreeReturnType;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::InputStream;
use std::path::Path;

use std::{fs};

/// Loads a zserio package from a file.
pub fn package_from_file(path: &Path) -> Box<ZPackage> {
    let contents = fs::read_to_string(path).expect("failed to read file");
    let lexer = ZserioLexer::new(InputStream::new(&*contents));
    let mut parser = ZserioParser::new(CommonTokenStream::new(lexer));

    let root = parser.packageDeclaration().unwrap(); //.expect("package declaration failed");
    let x = ZserioTreeReturnType::Str("".into());
    let result = Visitor(x).visit(&*root);

    match result {
        ZserioTreeReturnType::Package(p) => return p,
        _ => panic!("should not happen"),
    }
}

/*
    p.AddErrorListener(antlr.NewDiagnosticErrorListener(true))
    tree := p.PackageDeclaration()
    visitor := &visitor.Visitor{}

    switch v := visitor.Visit(tree).(type) {
    case error:
        return nil, err
    case *ast.Package:
        return v, nil
    }

    panic("unexpected result from visitor")
}
*/
