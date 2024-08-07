#!/bin/sh

java -jar antlr-rust.jar -Dlanguage=Rust -visitor -o gen ZserioLexer.g4 ZserioParser.g4

cat <<EOF
Do not forget to udpate zserparsers.rs: you need to replace
ZserioParser::with_strategy with something that looks like this:

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        let mut base = BaseParser::new_base_parser(
            input,
            Arc::clone(&interpreter),
            ZserioParserExt {
                _pd: Default::default(),
            },
        );
        base.remove_error_listeners();
        base.add_error_listener(Box::new(super::error::LogErrorListener {}));
        Self {
            base,
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

This is required to change the error listener to something that does
not spam the user with useless output.
EOF