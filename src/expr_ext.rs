use crate::*;
use std::collections::HashMap;

fn contain_ecls(tokens: &Vec<String>) -> bool {
    for token in tokens {
        if token.len() >= 2 && token.starts_with('e') && token.chars().nth(1).unwrap().is_ascii_digit() {
            return true;
        }
    }
    return false;
}

pub fn bfs_ext(grammar: &HashMap<String, Vec<String>>, levels: &u8, init_exprs: &Vec<String>) -> HashSet<String> {
    let mut equiv_exprs: HashSet<String> = Default::default();
    let mut tokens_prev_level: HashSet<Vec<String>> = init_exprs
        .iter()
        .map(|s| s.split_whitespace().map(String::from).collect())
        .collect();

    for _ in 1..*levels {
        let mut tokens_level: HashSet<Vec<String>> = Default::default();
        for mut tokens in &tokens_prev_level {
            if tokens.len() == 1 {
                let final_expr = tokens.join(" ");
                equiv_exprs.insert(final_expr);
            }
            log_debug(&format!("[INIT EXPR]: {:?}\n", tokens));
            for (i, op) in tokens.iter().enumerate() {
                log_debug(&format!("[OP]: {}\n", op));
                if op.len() == 1 || !op.starts_with('e') || op.starts_with("exp") ||
                    !grammar.contains_key(op) { continue; }
                let rw_vec = grammar.get(op).unwrap();

                log_debug(&format!("[RWVEC]: {:?}\n", rw_vec));
                for (k, rw) in rw_vec.iter().enumerate() {
                    log_debug(&format!("[RW]: {}\n", rw));
                    let rw_tokens: Vec<String> = rw.split_whitespace().map(String::from).collect();
                    /* context-free of context-sensitive, change here */
                    let mut new_tokens = tokens.clone();
                    new_tokens.splice(i..i+1, rw_tokens);
                    log_debug(&format!("[NEW TOKEN]: {:?}\n", new_tokens));

                    if !contain_ecls(&new_tokens) {
                        let final_expr = new_tokens.join(" ");
                        log_debug(&format!("[FINAL]: {}\n", final_expr));
                        equiv_exprs.insert(final_expr);
                    } else {
                        tokens_level.insert(new_tokens);
                    }
                }
            }
        }
        tokens_prev_level = tokens_level;
        log_debug(&format!("-----------------------"));
        log_debug(&format!("{:?}", tokens_prev_level));
    }

    return equiv_exprs;
}
