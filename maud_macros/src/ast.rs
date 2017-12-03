use proc_macro::{Span, TokenStream};

#[derive(Debug)]
pub enum Markup {
    Block(Block),
    Literal {
        content: String,
        span: Span,
    },
    Splice {
        expr: TokenStream,
    },
    Element {
        name: TokenStream,
        attrs: Attrs,
        body: Option<Box<Markup>>,
    },
    Let {
        tokens: TokenStream,
    },
    If {
        segments: Vec<Special>,
    },
    Special(Special),
    Match {
        head: TokenStream,
        arms: Vec<Special>,
        arms_span: Span,
    }
}

#[derive(Debug)]
pub struct Attrs {
    pub classes_static: Vec<ClassOrId>,
    pub classes_toggled: Vec<(ClassOrId, Toggler)>,
    pub ids: Vec<ClassOrId>,
    pub attrs: Vec<Attribute>,
}

type ClassOrId = TokenStream;

#[derive(Debug)]
pub struct Block {
    pub markups: Vec<Markup>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Special {
    pub head: TokenStream,
    pub body: Block,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: TokenStream,
    pub attr_type: AttrType,
}

#[derive(Debug)]
pub enum AttrType {
    Normal {
        value: Markup,
    },
    Empty {
        toggler: Option<Toggler>,
    },
}

#[derive(Debug)]
pub struct Toggler {
    pub cond: TokenStream,
    pub cond_span: Span,
}