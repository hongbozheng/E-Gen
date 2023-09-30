use crate::*;

pub fn bfs_ext(levels: &u8, init_exprs: &Vec<String>) -> HashSet<String> {
    let equiv_exprs: HashSet<String> = Default::default();
    let exprs_prev_level: HashSet<Vec<String>> = Default::default(); // prev level
    
    let mut level_0: HashSet<Vec<String>> = Default::default(); // level_0
    for expr in init_exprs.iter(){
        expr = expr.split_whitespace();
        level_0 = level_0.union(&expr);
    }
    exprs_prev_level = level_0;
    
    for level_id in 0..levels {
        let exprs_level: HashSet<Vec<String>> = Default::default(); // current level
        for expr in exprs_prev_level{
            for (i, token) in expr.iter().enumerate(){
                if token { //not in grammar
                    continue;
                }
                let rw_vec = grammar[token]; //
                for rw in rw_vec{
                    let mut new_expr = expr.clone();
                    new_expr[i] = rw_vec[rw];
                    if check_no_eclass(&new_expr) {
                        equiv_exprs = equiv_exprs.union(&new_expr.iter().collect());//convert into string and store new_expr in the equiv_exprs
                    }
                    exprs_level.union(&new_expr);
                }
            }
        }
        exprs_prev_level = exprs_level;
    }

    return equiv_exprs;
}

pub fn check_no_eclass(new_expr: &Vec<String>){//check is there eclass in new_exprs
    for token in new_expr{
        if grammar.contains(&token) {
            return false
        }
    }
    return true
}