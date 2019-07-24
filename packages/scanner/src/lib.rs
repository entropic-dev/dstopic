use resast::ref_tree::prelude::*;
use resast::ref_tree::{ProgramPart, decl, expr};
use ressa::Parser;
use std::vec::Vec;

pub fn parse_js<'a>(js: &'a str) -> Result<Vec<&'a str>, ressa::Error> {
    let p = Parser::new(js)?;
    let mut pd = Vec::<&str>::new();
    for part in p {
        if let Ok(the_part) = part {
            match the_part {
                ProgramPart::Decl(some_part) => {
                    pd = match_declaration(some_part, pd)
                },
                ProgramPart::Stmt(some_part) => {
                    if let Stmt::Expr(expr) = some_part {
                        if let Expr::Call(call) = expr {
                            pd = match_expr(call, pd)
                        }
                    }
                },
                _ => (),
            }
        }
    }
    Ok(pd)
}

fn match_declaration<'a>(declaration: Decl<'a>, mut pd: Vec<&'a str>) -> Vec<&'a str> {
    match declaration {
        Decl::Import(import) => pd = match_import(import, pd),
        Decl::Variable(_, variable_vec) => pd = match_variable(variable_vec, pd),
        _ => (),
    }
    pd
}

fn match_import<'a>(import: Box<decl::ModImport<'a>>, mut pd: Vec<&'a str>) -> Vec<&'a str> {
    let source = import.source;
    if let expr::Literal::String(string_arg) = source {
        let quotes: &[_] = &['\'', '\"'];
        pd.push(string_arg.trim_matches(quotes));
    }
    pd
}

fn match_variable<'a>(
    variable_vec: Vec<decl::VariableDecl<'a>>,
    mut pd: Vec<&'a str>,
) -> Vec<&'a str> {
    for v in variable_vec {
        if let Some(variable_init) = v.init {
            match variable_init {
                Expr::Call(call) => pd = match_expr(call, pd),
                _ => (),
            }
        }
    }
    pd
}

fn match_expr<'a>(expression: expr::CallExpr<'a>, mut pd: Vec<&'a str>,) -> Vec<&'a str> {
    if let Expr::Ident(callee) = *expression.callee {
        if callee == "require" {
            if let Some(argument) = expression.arguments.get(0) {
                if let expr::Expr::Literal(literal) = argument {
                    if let expr::Literal::String(string_arg) = literal {
                        let quotes: &[_] = &['\'', '\"'];
                        pd.push(string_arg.trim_matches(quotes));
                    }
                }
            }
        }
    }
    pd
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn top_level_require_single_quotes() {
        let js = "const _ = require('lodash');";
        assert_eq!(&"lodash", parse_js(js).unwrap().get(0).unwrap())
    }
    #[test]
    fn top_level_require_double_quotes() {
        let js = "const _ = require(\"lodash\");";
        assert_eq!(&"lodash", parse_js(js).unwrap().get(0).unwrap())
    }
    #[test]
    fn top_level_require_statement() {
        let js = "require(\"lodash\");";
        assert_eq!(&"lodash", parse_js(js).unwrap().get(0).unwrap())
    }
    #[test]
    fn top_level_default_import() {
        let js = "import _ from 'lodash';";
        assert_eq!(&"lodash", parse_js(js).unwrap().get(0).unwrap())
    }
    #[test]
    fn top_level_import_alias() {
        let js = "import * as lodash from \"lodash\";";
        assert_eq!(&"lodash", parse_js(js).unwrap().get(0).unwrap())
    }
    #[test]
    fn top_level_import_named() {
        let js = "import { map } from \"lodash\";";
        assert_eq!(&"lodash", parse_js(js).unwrap().get(0).unwrap())
    }
}
