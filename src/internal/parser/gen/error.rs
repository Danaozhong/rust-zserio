use antlr_rust::error_listener::ErrorListener;
use antlr_rust::errors::ANTLRError;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token_factory::TokenFactory;
use log::trace;

#[derive(Debug)]
pub struct LogErrorListener {}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for LogErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _e: Option<&ANTLRError>,
    ) {
        trace!("line {}:{} {}", line, column, msg);
    }
}
