//! Defines input for generation of AST and `SyntaxKind`

pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (":", "COLON"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_BRACE"),
        ("}", "R_BRACE"),
        (",", "COMMA"),
        ("=", "ASSIGN"),
        ("-", "HYPHEN"),
    ],
    keywords: &[
        "and",
        "or",
        "not",
        "true",
        "false",
        "rule",
        "strings",
        "condition",
        "meta",
        "private",
        "global",
        "import",
        "include",
        "ascii",
        "wide",
        "nocase",
        "fullword",
        "xor",
        "base64",
        "base64wide",
    ],
    literals: &["STRING_LIT", "INT_LIT", "FLOAT_LIT"],
    tokens: &["IDENTIFIER", "VARIABLE", "WHITESPACE", "COMMENT", "ERROR"],
    nodes: &[
        "RULE",
        "MODIFIER",
        "TAG",
        "STRINGS",
        "META",
        "CONDITION",
        "SOURCE_FILE",
        "BLOCK_EXPR",
        "PREFIX_EXPR",
        "LITERAL",
        "EXPRESSION",
        "EXPRESSION_STMT",
        "VARIABLE_STMT",
        "IMPORT_STMT",
        "INCLUDE_STMT",
        "META_STMT",
        "PATTERN",
        "PATTERN_MOD",
        "BASE_ALPHABET",
        "XOR_RANGE",
    ],
};

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node { name: String, ty: String, cardinality: Cardinality },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) variants: Vec<String>,
}
