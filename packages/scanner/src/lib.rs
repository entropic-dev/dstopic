use resast::ref_tree::prelude::*;
use ressa::Parser;
use std::vec::Vec;

pub fn parse_js<'a>(js: &'a str) -> Vec<&'a str> {
    let p = Parser::new(js).unwrap();
    let mut pd = Vec::<&str>::new();
    for part in p {
        if let Ok(the_part) = part {
            match the_part {
                resast::ref_tree::ProgramPart::Decl(some_part) => {
                    pd = match_declaration(some_part, pd)
                }
                the_thing => println!("Not a program part: {:?}", the_thing),
            }
        }
    }
    return pd;
}

fn match_declaration<'a>(declaration: Decl<'a>, mut pd: Vec<&'a str>) -> Vec<&'a str> {
    match declaration {
        Decl::Import(import) => println!("{:?}", import),
        Decl::Variable(_, variable_vec) => pd = match_variable(variable_vec, pd),
        the_thing => println!("log_declaration: {:?}", the_thing),
    }
    pd
}

fn match_variable<'a>(
    variable_vec: Vec<resast::ref_tree::decl::VariableDecl<'a>>,
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

fn match_expr<'a>(
    expression: resast::ref_tree::expr::CallExpr<'a>,
    mut pd: Vec<&'a str>,
) -> Vec<&'a str> {
    if let Expr::Ident(callee) = *expression.callee {
        if callee == "require" {
            if let Some(argument) = expression.arguments.get(0) {
                println!("{:?}", argument);
                if let resast::ref_tree::expr::Expr::Literal(literal) = argument {
                    if let resast::ref_tree::expr::Literal::String(string_arg) = literal {
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
}
