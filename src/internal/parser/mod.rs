#![allow(unknown_lints)]
#![allow(clippy)]
pub mod gen {
    pub mod zseriolexer;
    pub mod zserioparser;
    pub mod zserioparserlistener;
    pub mod zserioparservisitor; // should not be needed, but the antlr-rust doesn't work when calling
}
