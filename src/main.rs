use sigmaos_libreoffice_math::{evaluate_expr, parse_expr};
use std::collections::HashMap;

fn usage() -> ! {
    eprintln!("usage: sigmaos_libreoffice_math eval EXPR [VAR=VALUE ...]");
    eprintln!("       sigmaos_libreoffice_math repr EXPR");
    std::process::exit(2);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(cmd) = args.next() else { usage() };
    let Some(expr_src) = args.next() else { usage() };

    match cmd.as_str() {
        "eval" => {
            let mut vars = HashMap::new();
            vars.insert("pi".to_string(), std::f64::consts::PI);
            vars.insert("e".to_string(), std::f64::consts::E);
            for arg in args {
                if let Some((k, v)) = arg.split_once('=') {
                    if let Ok(n) = v.parse::<f64>() {
                        vars.insert(k.to_string(), n);
                    }
                }
            }
            match evaluate_expr(&expr_src, &vars) {
                Ok(v) => println!("{v}"),
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }
        }
        "repr" => match parse_expr(&expr_src) {
            Ok(expr) => println!("{}", expr.to_string_repr()),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        },
        _ => usage(),
    }
}
