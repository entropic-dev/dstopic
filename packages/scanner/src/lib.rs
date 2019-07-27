use resast::ref_tree::prelude::*;
use resast::ref_tree::{decl, expr, ProgramPart};
use ressa::Parser;
use std::vec::Vec;

pub fn parse_js<'a>(js: &'a str) -> Result<Vec<&'a str>, ressa::Error> {
    let p = Parser::new(js)?;
    let mut pd = Vec::<&str>::new();
    for part_result in p {
        if let Ok(part) = part_result {
            match part {
                ProgramPart::Decl(declaration) => pd = decl_to_impt_or_var(declaration, pd),
                ProgramPart::Stmt(statement) => pd = stmt_to_call(statement, pd),
                _ => (),
            }
        }
    }
    Ok(pd)
}

fn decl_to_impt_or_var<'a>(declaration: Decl<'a>, mut pd: Vec<&'a str>) -> Vec<&'a str> {
    match declaration {
        Decl::Import(import) => pd = impt_to_str(import, pd),
        Decl::Variable(_, variable_vec) => pd = var_init_to_call(variable_vec, pd),
        _ => (),
    }
    pd
}

fn stmt_to_call<'a>(statement: Stmt<'a>, mut pd: Vec<&'a str>) -> Vec<&'a str> {
    if let Stmt::Expr(expression) = statement {
        if let Expr::Call(call) = expression {
            pd = call_to_str(call, pd)
        }
    }
    pd
}

fn var_init_to_call<'a>(
    variable_vec: Vec<decl::VariableDecl<'a>>,
    mut pd: Vec<&'a str>,
) -> Vec<&'a str> {
    for v in variable_vec {
        if let Some(variable_init) = v.init {
            if let Expr::Call(call) = variable_init {
                pd = call_to_str(call, pd);
            }
        }
    }
    pd
}

fn trim_quotes<'a>(string: &'a str) -> &'a str {
    string.trim_matches(|c| c == '\'' || c == '\"')
}

fn call_to_str<'a>(expression: expr::CallExpr<'a>, mut pd: Vec<&'a str>) -> Vec<&'a str> {
    if let Expr::Ident(callee) = &*expression.callee {
        if callee == &"require" {
            if let Some(argument) = expression.arguments.get(0) {
                if let expr::Expr::Literal(literal) = argument {
                    if let expr::Literal::String(string) = literal {
                        pd.push(trim_quotes(string));
                    }
                }
            }
        }
    }
    pd
}

fn impt_to_str<'a>(import: Box<decl::ModImport<'a>>, mut pd: Vec<&'a str>) -> Vec<&'a str> {
    if let expr::Literal::String(string) = import.source {
        pd.push(trim_quotes(string));
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
    #[test]
    fn no_valid_calls() {
        let js = "const five = 5;";
        assert_eq!(None, parse_js(js).unwrap().get(0));
    }
    #[test]
    fn multi_line_one_valid_call() {
        let js = &[
            "const five = 5;",
            "const moment = require('moment');",
            "start('now');",
        ]
        .join(&"\n");
        assert_eq!(&"moment", parse_js(js).unwrap().get(0).unwrap());
    }
    #[test]
    fn multi_line_two_valid_calls() {
        let js = &[
            "const five = 5;",
            "const moment = require('moment');",
            "import _ from 'lodash';",
            "start();",
        ]
        .join(&"\n");
        let packages = parse_js(js).unwrap();
        assert_eq!(&"moment", packages.get(0).unwrap());
        assert_eq!(&"lodash", packages.get(1).unwrap());
    }
}
