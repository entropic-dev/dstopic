use ressa::Parser;
use resast;

fn main() {
    let js = "const _ = require('lodash'); import * as moment from 'moment';";
    let p = Parser::new(&js).unwrap();

    for part in p {
        if let Ok(the_part) = part {
            match the_part {
                resast::ref_tree::ProgramPart::Decl(some_part) => log_the_declaration(some_part),
                _ => println!("You lose!"),
            }
        }
    }
}

fn log_the_declaration (declaration: resast::ref_tree::prelude::Decl) {
    match declaration {
        resast::ref_tree::prelude::Decl::Import(import) => println!("{:?}", import),
        resast::ref_tree::prelude::Decl::Variable(_, variable_vec) => log_if_require(variable_vec),
        _ => println!("You Lose!")
    }

}

fn log_if_require (variable_vec: Vec<resast::ref_tree::decl::VariableDecl>) {
    for v in variable_vec {
        if let Some(variable_init) = v.init {
            match variable_init {
                resast::ref_tree::prelude::Expr::Call(call) => println!("{:?}", call),
                _ => println!("You lose!")
            }
        }
    }
}
