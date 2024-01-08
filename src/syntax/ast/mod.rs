#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommentKind {
    pub shape: CommentShape,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentShape {
    Line,
    Block,
}

impl CommentShape {
    pub fn is_line(self) -> bool {
        self == CommentShape::Line
    }

    pub fn is_block(self) -> bool {
        self == CommentShape::Block
    }
}

impl CommentKind {
    const BY_PREFIX: [(&'static str, CommentKind); 5] = [
        (
            "/**/",
            CommentKind {
                shape: CommentShape::Block,
            },
        ),
        (
            "/***",
            CommentKind {
                shape: CommentShape::Block,
            },
        ),
        (
            "////",
            CommentKind {
                shape: CommentShape::Line,
            },
        ),
        (
            "//",
            CommentKind {
                shape: CommentShape::Line,
            },
        ),
        (
            "/*",
            CommentKind {
                shape: CommentShape::Block,
            },
        ),
    ];

    pub(crate) fn from_text(text: &str) -> CommentKind {
        let &(_prefix, kind) = CommentKind::BY_PREFIX
            .iter()
            .find(|&(prefix, _kind)| text.starts_with(prefix))
            .unwrap();
        kind
    }

    pub fn prefix(&self) -> &'static str {
        let &(prefix, _) = CommentKind::BY_PREFIX
            .iter()
            .rev()
            .find(|(_, kind)| kind == self)
            .unwrap();
        prefix
    }
}
