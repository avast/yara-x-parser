/// This library is used to create a parser for YARA language
/// It should provide also token for whitespaces
/// as we want full fidelity and error resilience.;
use crate::{
    parser::SyntaxKind,
    syntax::{
        syntax_error::SyntaxError,
        syntax_node::{SyntaxNode, SyntaxToken},
        text_token_source::TextTokenSource,
        text_tree_sink::TextTreeSink,
    },
};

pub use crate::syntax::ast::*;
pub use crate::syntax::SourceFile;

// use only for tests
#[cfg(test)]
use rowan_test::{NodeOrToken, WalkEvent};
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::io::Write;
use std::ops::Range;
#[cfg(test)]
use text_size::TextRange;

mod lexer;
mod parser;
mod syntax;

/// Just a showcase test to see how API for typed layer
/// of AST could work
#[test]
fn api_walktrough() {
    // This is a simple YARA rule
    // without errors
    let source_code = "
        rule test_rule {
            // This is a comment
            strings:
                $a = \"test\"
            condition:
                $a or not true
        }
    ";

    // SourceFile is the main entry point for any given file
    // it contains a `parse` method which returns a `Parse` struct
    // that contains AST and list of errors

    let parse_struct = SourceFile::parse(source_code);
    assert!(parse_struct.errors().is_empty());

    // To obtian the AST we can use `tree` method
    // every tree starts with `SourceFile` node which is the root
    let ast = parse_struct.tree();

    // Now we can travers the tree and obtain the other nodes
    // for example we can loop over rules in the file
    // in given example we have only one rule
    for rule in ast.rules() {
        // We can obtain also the identifier of the rule
        // and assert its name is `test_rule`
        assert_eq!(rule.identifier_token().unwrap().text(), "test_rule");

        // Each rule also have 'RULE_KW' token
        assert!(rule.rule_token().is_some());
        // Yes, just like this we can get the syntax token
        assert!(rule.rule_token().unwrap().kind() == SyntaxKind::RULE_KW);

        // Last but not least we can obtain the block of the rule
        // which is essentially a block expression
        let block = rule.body().unwrap();

        // Just to showcase, each node can also return comments
        // that belongs to that specific node
        for comment in block.comments() {
            // In this case we have only one comment
            assert_eq!(comment.text(), "This is a comment");
        }

        // This block expression consists (for now) of two parts
        // optional strings and required condition part
        // Firstly we can obtain the strings part
        let strings = block.strings().unwrap();

        // I can show again that we can obtain the tokens
        // for example the `STRINGS_KW` token
        assert!(strings.strings_token().is_some());
        assert!(strings.strings_token().unwrap().kind() == SyntaxKind::STRINGS_KW);

        // and also `COLON` token
        assert!(strings.colon_token().is_some());
        assert!(strings.colon_token().unwrap().kind() == SyntaxKind::COLON);

        // Each strings section also contains multiple
        // `VARIABLE_STMT` nodes
        for variable_stmt in strings.variable_stmts() {
            // each variable statement contains a variable token
            // an assign token and a literal token
            // now I will showm only the pattern token as an example
            let pattern = variable_stmt.pattern().unwrap();

            // For now pattern can be only a string literal
            assert!(pattern.string_lit_token().is_some());
            assert!(pattern.string_lit_token().unwrap().kind() == SyntaxKind::STRING_LIT);
        }

        // For the condition part, we can similarly get its body which is
        // an `EXPRESSION_STMT` node
        let condition = block.condition().unwrap();
        let condition_body = condition.expression_stmt().unwrap();

        // Each expression statement for now consists of either
        // `EXPRESSION`, `PREFIX_EXPR` or `LITERAL` node for binary expressions, unary expressions
        // and literals respectively, which are essentially the only 3 things we can have in the
        // condition so far. `EXPR` enum is used to group these 3 types of nodes together
        // There is Pratt parser in the background used for operators precedence
        let expr = condition_body.expr().unwrap();
        let expression = match &expr {
            Expr::Expression(e) => e,
            _ => unreachable!(),
        };

        // Now we can obtain `lhs`, `rhs` or `op` nodes for top level expression
        // in this case we have `OR` operator
        assert!(expression.op_token().is_some());
        assert!(expression.op_token().unwrap().kind() == SyntaxKind::OR_KW);

        // On the left hand side we have a LITERAL token
        // It is essentially like I mentioned `EXPR` enum
        // therefore we have to match it to obtain the `LITERAL` node
        let lhs = expression.lhs().unwrap();
        let lhs_literal = match &lhs {
            Expr::Literal(l) => l,
            _ => unreachable!(),
        };
        assert!(lhs_literal.token().kind() == SyntaxKind::VARIABLE);
        assert_eq!(lhs_literal.token().text(), "$a");

        // On the right hand side we have a `PREFIX_EXPR` node
        // which is essentially a unary expression
        let rhs = expression.rhs().unwrap();
        let rhs_prefix = match &rhs {
            Expr::PrefixExpr(p) => p,
            _ => unreachable!(),
        };

        // Prefix expression consists of an operator and an expression
        // in this case we have `NOT` operator
        assert!(rhs_prefix.op_token().is_some());
        assert!(rhs_prefix.op_token().unwrap().kind() == SyntaxKind::NOT_KW);

        // and the `LITERAL` node which is a `TRUE_KW` token
        let rhs_body = rhs_prefix.expr().unwrap();
        let rhs_literal = match &rhs_body {
            Expr::Literal(l) => l,
            _ => unreachable!(),
        };
        assert!(rhs_literal.token().kind() == SyntaxKind::TRUE_KW);
        assert_eq!(rhs_literal.token().text(), "true");

        // Last but not least, in any point we can obtain the syntax node
        // for example let's obtain the syntax node for `EXPRESSION_STMT`
        let expression_stmt_syntax = condition_body.syntax();

        assert_eq!(expression_stmt_syntax.text().to_string(), "$a or not true");

        // Syntax node have also bunch of methods
        // for example we can obtain the parent node
        let parent = expression_stmt_syntax.parent().unwrap();
        assert_eq!(parent.kind(), SyntaxKind::CONDITION);
        assert_eq!(parent.text().to_string(), "condition:\n                $a or not true");

        // We can also obtain the children
        let children = expression_stmt_syntax.first_child_or_token().unwrap();
        assert_eq!(children.kind(), SyntaxKind::EXPRESSION);

        // and also the next sibling, which in this layer can be also a whitespace
        let next_sibling = parent.next_sibling_or_token().unwrap();
        assert_eq!(next_sibling.kind(), SyntaxKind::WHITESPACE);

        // Some helpers:
        // for example get token at specific offset. This can be useful
        // to obtain the token at given Error offset, to get its text, length etc.
        let tkn = expression_stmt_syntax.token_at_offset(151.into());

        // We can have offset that is between two tokens, so we use `right_biased` method
        // to obtain the token on the right side of the offset if it is between two tokens
        // or just to get the token type
        assert!(tkn.right_biased().unwrap().kind() == SyntaxKind::OR_KW);

        // There is also a method to do a preorder traversal
        // Note that we are using those methods just for `EXPRESSION_STMT` subtree
        // but it can be also used on root tree and any other subtree
        // It works with `WalkEvent` which can be either `Enter` or `Leave`
        for (i, event) in expression_stmt_syntax.preorder_with_tokens().enumerate() {
            // Assert first couple of events
            match event {
                WalkEvent::Enter(node) => {
                    let kind = match &node {
                        NodeOrToken::Node(it) => it.kind(),
                        NodeOrToken::Token(it) => it.kind(),
                    };
                    if i == 0 {
                        assert_eq!(kind, SyntaxKind::EXPRESSION_STMT);
                    }
                    if i == 1 {
                        assert_eq!(kind, SyntaxKind::EXPRESSION);
                    }
                    if i == 2 {
                        assert_eq!(kind, SyntaxKind::LITERAL);
                    }
                    if i == 3 {
                        assert_eq!(kind, SyntaxKind::VARIABLE);
                    }
                }
                WalkEvent::Leave(node) => {
                    let kind = match &node {
                        NodeOrToken::Node(it) => it.kind(),
                        NodeOrToken::Token(it) => it.kind(),
                    };
                    if i == 4 {
                        assert_eq!(kind, SyntaxKind::VARIABLE);
                    }
                }
            }
        }

        // The last thing I want to showcase are errors
        // This is a simple YARA rule with errors
        // it has two errors, one is missing `$` before variable
        // declaration and the other one is unsupported `nor` operator
        let source_code = "
            rule test_rule {
                // This is a comment
                strings:
                    a = \"test\"
                condition:
                    $a nor not true
            }
        ";

        let parse_struct = SourceFile::parse(source_code);

        // There are some errors
        assert!(!parse_struct.errors().is_empty());
        assert!(parse_struct.errors().len() == 2);
        assert!(parse_struct.errors()[0].to_string() == "expected a variable");
        assert!(parse_struct.errors()[1].to_string() == "unsupported expression");

        // We still have the AST and we can traverse it
        let ast = parse_struct.tree();

        // We loop over rules
        for rule in ast.rules() {
            assert!(rule.identifier_token().unwrap().text() == "test_rule");
            let block = rule.body().unwrap();
            let condition = block.condition().unwrap();
            let condition_body = condition.expression_stmt().unwrap();
            let expr = condition_body.expr().unwrap();
            // The operator is wrong, therefore from binary expression we have
            // a `LITERAL` expression
            let expression = match &expr {
                Expr::Literal(e) => e,
                _ => unreachable!(),
            };
            assert!(expression.token().kind() == SyntaxKind::VARIABLE);

            // and we can obtain the error token
            let error_token = condition
                .syntax()
                .children_with_tokens()
                .find(|it| it.kind() == SyntaxKind::ERROR)
                .unwrap();

            assert!(error_token.kind() == SyntaxKind::ERROR);
            assert!(error_token.as_node().unwrap().text() == "nor");
        }
        // We can also search a token that produced the error
        // Even though it produces range, ParseErrors only supports text offsets
        assert_eq!(parse_struct.errors()[1].range(), TextRange::new(173.into(), 173.into()));

        // But luckily we can obtain the token at the offset
        // and from it we can get both its text and length
        let tkn = ast.syntax().token_at_offset(173.into()).right_biased().unwrap();

        assert_eq!(tkn.text(), "nor");
        // Error node contains also appropriate nested SyntaxKind
        assert_eq!(tkn.kind(), SyntaxKind::IDENTIFIER);
        // and also the length as TextRange for specific token
        assert_eq!(tkn.text_range(), TextRange::new(173.into(), 176.into()));
        // or
        assert_eq!(tkn.text().len(), 3);
    }
}

/// This test is used to compare the output of the parser
/// with the expected output
#[test]
fn test_parse_text() {
    let mut mint = goldenfile::Mint::new(".");

    for entry in globwalk::glob("tests/*.in").unwrap().flatten() {
        // Path to the .in.zip file.
        let path = entry.into_path();
        let display_path = path.display();

        let input = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read input file {:?}", display_path));

        let ast_struct = SourceFile::parse(&input);

        let out_path = path.with_extension("").with_extension("out");

        let mut output_file = mint.new_goldenfile(out_path).unwrap();

        write!(output_file, "{:#?}", ast_struct.tree().syntax).unwrap();

        // Check errors
        let err_path = path.with_extension("").with_extension("err");
        if err_path.exists() {
            let expected_errors = fs::read_to_string(&err_path)
                .unwrap_or_else(|_| panic!("Failed to read error file {:?}", err_path.display()));
            let actual_errors = ast_struct
                .errors()
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<_>>()
                .join("\n");
            assert_eq!(actual_errors, expected_errors);
        } else {
            assert!(ast_struct.errors().is_empty(), "Unexpected errors: {:?}", ast_struct.errors());
        }
    }
}
