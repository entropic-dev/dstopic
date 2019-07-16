use resast::ref_tree::prelude::*;
use resast::ref_tree::{ProgramPart, decl, expr};
use ressa::Parser;
use std::vec::Vec;

pub fn parse_js<'a>(js: &'a str) -> Vec<&'a str> {
    let p = Parser::new(js).unwrap();
    let mut pd = Vec::<&str>::new();
    for part in p {
        if let Ok(the_part) = part {
            match the_part {
                ProgramPart::Decl(some_part) => {
                    pd = match_declaration(some_part, pd)
                },

                // Dynamic imports
                ProgramPart::Stmt(some_part) => {
                    if let Stmt::Expr(expr) = some_part {
                        if let Expr::Call(call) = expr {
                            pd = match_expr(call, pd)
                        }
                    }
                },
                the_thing => println!("Not a program part: {:?}", the_thing),
            }
        }
    }
    pd
}

fn match_declaration<'a>(declaration: Decl<'a>, mut pd: Vec<&'a str>) -> Vec<&'a str> {
    match declaration {
        Decl::Import(import) => pd = match_import(import, pd),
        Decl::Variable(_, variable_vec) => pd = match_variable(variable_vec, pd),
        the_thing => println!("log_declaration: {:?}", the_thing),
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
                the_thing => println!("log_require: {:?}", the_thing),
            }
        }
    }
    pd
}

fn match_expr<'a>(expression: expr::CallExpr<'a>, mut pd: Vec<&'a str>,) -> Vec<&'a str> {
    if let Expr::Ident(callee) = *expression.callee {
        println!("callee: {:?}", callee);
        if callee == "require" || callee == "import" {
            if let Some(argument) = expression.arguments.get(0) {
                println!("{:?}", argument);
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
        assert_eq!(&"lodash", parse_js(js).get(0).unwrap())
    }
    #[test]
    fn top_level_require_double_quotes() {
        let js = "const _ = require(\"lodash\");";
        assert_eq!(&"lodash", parse_js(js).get(0).unwrap())
    }
    #[test]
    fn top_level_default_import() {
        let js = "import _ from 'lodash';";
        assert_eq!(&"lodash", parse_js(js).get(0).unwrap())
    }
    #[test]
    fn top_level_import_alias() {
        let js = "import * as lodash from \"lodash\";";
        assert_eq!(&"lodash", parse_js(js).get(0).unwrap())
    }
    #[test]
    fn top_level_import_named() {
        let js = "import { map } from \"lodash\";";
        assert_eq!(&"lodash", parse_js(js).get(0).unwrap())
    }
    #[test]
    fn dynamic_import() {
        let js = "import('lodash');";
        assert_eq!(&"lodash", parse_js(js).get(0).unwrap())
    }
}
