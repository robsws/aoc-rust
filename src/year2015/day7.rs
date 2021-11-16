use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let conn_strs = read_lines(input_file_path);
    let mut conns = Vec::with_capacity(conn_strs.len());
    for conn_str in conn_strs {
        conns.push(parse_connection(conn_str));
    }
    let resolved_signals = resolve_connections(&conns);
    println!("{}", resolved_signals["a"]);
}

fn resolve_connections(conns: &Vec<Connection>) -> HashMap<String, u16> {
    let mut resolved_signals = HashMap::new();
    let mut modified_this_pass = true;
    while modified_this_pass {
        modified_this_pass = false;
        for conn in conns {
            match &conn.source {
                LogicExpression::Base(e) => {
                    match resolve_expression(e, &resolved_signals) {
                        Some(v) => {
                            if !resolved_signals.contains_key(&conn.target) {
                                resolved_signals.insert(conn.target.clone(), v);
                                modified_this_pass = true;
                            }
                        },
                        _ => {}
                    }
                },
                LogicExpression::NotGate(e) => {
                    match resolve_expression(e, &resolved_signals) {
                        Some(v) => {
                            if !resolved_signals.contains_key(&conn.target) {
                                resolved_signals.insert(conn.target.clone(), !v);
                                modified_this_pass = true;
                            }
                        },
                        _ => {}
                    }
                },
                LogicExpression::AndGate(e1, e2) => {
                    let result1 = resolve_expression(e1, &resolved_signals);
                    let result2 = resolve_expression(e2, &resolved_signals);
                    match (result1, result2) {
                        (Some(v1), Some(v2)) => {
                            if !resolved_signals.contains_key(&conn.target) {
                                resolved_signals.insert(conn.target.clone(), v1 & v2);
                                modified_this_pass = true;
                            }
                        },
                        _ => {}
                    }
                },
                LogicExpression::OrGate(e1, e2) => {
                    let result1 = resolve_expression(e1, &resolved_signals);
                    let result2 = resolve_expression(e2, &resolved_signals);
                    match (result1, result2) {
                        (Some(v1), Some(v2)) => {
                            if !resolved_signals.contains_key(&conn.target) {
                                resolved_signals.insert(conn.target.clone(), v1 | v2);
                                modified_this_pass = true;
                            }
                        },
                        _ => {}
                    }
                },
                LogicExpression::LShift(e1, e2) => {
                    let result1 = resolve_expression(e1, &resolved_signals);
                    let result2 = resolve_expression(e2, &resolved_signals);
                    match (result1, result2) {
                        (Some(v1), Some(v2)) => {
                            if !resolved_signals.contains_key(&conn.target) {
                                resolved_signals.insert(conn.target.clone(), v1 << v2);
                                modified_this_pass = true;
                            }
                        },
                        _ => {}
                    }
                },
                LogicExpression::RShift(e1, e2) => {
                    let result1 = resolve_expression(e1, &resolved_signals);
                    let result2 = resolve_expression(e2, &resolved_signals);
                    match (result1, result2) {
                        (Some(v1), Some(v2)) => {
                            if !resolved_signals.contains_key(&conn.target) {
                                resolved_signals.insert(conn.target.clone(), v1 >> v2);
                                modified_this_pass = true;
                            }
                        },
                        _ => {}
                    }
                },
            }
        }
    }
    resolved_signals
}

fn resolve_expression(expr: &BaseExpression, resolved_signals: &HashMap<String, u16>) -> Option<u16> {
    match expr {
        BaseExpression::Constant(c) => Some(c.to_owned()),
        BaseExpression::Reference(r) => {
            if resolved_signals.contains_key(r) {
                Some(resolved_signals[r])
            } else {
                None
            }
        }
    }
}

enum LogicExpression {
    Base(BaseExpression),
    NotGate(BaseExpression),
    AndGate(BaseExpression, BaseExpression),
    OrGate(BaseExpression, BaseExpression),
    LShift(BaseExpression, BaseExpression),
    RShift(BaseExpression, BaseExpression)
}

enum BaseExpression {
    Constant(u16),
    Reference(String),
}

struct Connection {
    source: LogicExpression,
    target: String,
}

lazy_static! {
    static ref CONNECTION_RE: Regex =
        Regex::new(
            "^(?P<not>NOT )?(?:(?P<lconst>[0-9]+)|(?P<lref>[a-z]+))(?: (?P<binop>AND|OR|NOT|LSHIFT|RSHIFT) (?:(?P<rconst>[0-9]+)|(?P<rref>[a-z]+)))? -> (?P<dest>[a-z]+)$"
        ).unwrap();
}

fn parse_connection(connection_str: String) -> Connection {
    let caps = CONNECTION_RE.captures(&connection_str)
        .expect(&format!("Input line did not match expected pattern. {}", connection_str));
    let not;
    // Check if NOT included
    match caps.name("not") {
        Some(_) => not = true,
        _ => not = false
    }
    // Parse first constant/reference term
    let lexpr = parse_base_expression(caps.name("lconst"), caps.name("lref"));
    // Check for a binary operation
    let binop = match caps.name("binop") {
        Some(m) => {
            // Don't allow NOT in combination with a binary operator.
            if not {
                panic!("NOT not supported for complex expressions.");
            }
            Some(m.as_str())
        },
        None => None
    };
    let source = match binop {
        Some(op) => {
            // If there is a binary op, parse the second constant/reference term
            // and build the appropriate binary logic expression from the first and
            // second terms.
            let rexpr = parse_base_expression(caps.name("rconst"), caps.name("rref"));
            match op {
                "AND" => LogicExpression::AndGate(lexpr, rexpr),
                "OR" => LogicExpression::OrGate(lexpr, rexpr),
                "LSHIFT" => LogicExpression::LShift(lexpr, rexpr),
                "RSHIFT" => LogicExpression::RShift(lexpr, rexpr),
                _ => panic!("Unsupported binary operation in line.")
            }
        },
        None => {
            // If there's no binary op, build a NOT logic expression
            // if NOT was found, or just use the left expression directly
            // if there wasn't.
            if not {
                LogicExpression::NotGate(lexpr)
            } else {
                LogicExpression::Base(lexpr)
            }
        }
    };
    // The target of the connection is a basic reference.
    let target = caps.name("dest").expect("No destination found in line.").as_str().to_owned();
    // Build the connection from the parsed components.
    Connection {source, target}
}

fn parse_base_expression(
    const_match: Option<regex::Match>,
    ref_match: Option<regex::Match>
) -> BaseExpression {
    match const_match {
        Some(m) => {
            let value = m.as_str().parse::<u16>().expect("Constant is not 16 bit unsigned integer.");
            BaseExpression::Constant(value)
        },
        None => {
            match ref_match {
                Some(m) => {
                    let reference = m.as_str().to_owned();
                    BaseExpression::Reference(reference)
                },
                None => panic!("Unhandled expression type.")
            }
        }
    }
}