use crate::*;
// use regex::Regex;

pub struct ExpressionExtract {
    csg: bool,                              /* context-sensitive grammar flag               */
    DEBUG: bool,                            /* debug flag                                   */
    max_rw_len: u8,                         /* maximum rewrite length                       */
    ctx_gr: ContextGrammar,                 /* context grammar struct                       */
    rw: Vec<String>                         /* vec storing final rewrite                    */
}

impl ExpressionExtract {
    /// ## default constructor
    /// ## Arguments
    /// * `MathEGraph` - egraph after running rewrite rules
    /// * `init_expr`  - initial expression to run with egraph
    /// * `root_classes` - root classes of MathEGraph
    pub fn new(csg: bool, DEBUG: bool, max_rw_len: u8, ctx_gr: ContextGrammar) -> Self {
        ExpressionExtract {
            csg,
            DEBUG,
            max_rw_len,
            ctx_gr,
            rw: vec![],
        }
    }

    /// ## member function to get the final rewrites from self
    /// ## Argument
    /// * `self`
    pub fn get_rw(&self) -> &Vec<String> { return &self.rw; }

    /// ## private member function to replace distinct eclass with rewrite rule
    /// ## Argument
    /// * `self`
    /// * `op`  - operand that needs to be replaced
    /// * `rw`  - rewrite rule that is going to be replaced with
    /// * `str` - original expression
    fn distinct_replace(&self, op: &str, rw: &String, str: &mut String) {
        let matches: Vec<_> = str.match_indices(op).collect();
        for mat in matches {
            let start_idx = &mat.0;
            let end_idx = &(start_idx + op.len());
            if (*end_idx != str.len() && str.chars().nth(*end_idx).unwrap() == ' ') ||
                *end_idx == str.len() {
                str.replace_range(start_idx..end_idx,rw);
                break;
            }
        }
    }

    /// ## private member function to extract all equivalent mathematical expressions
    /// ## Context-Sensitive Grammar
    /// ## Argument
    /// * `self`
    /// * `str` - rewrite expression
    /// * `idx` - fn call idx for debugging purpose
    fn csg_extract(&mut self, mut str: String, idx: u8) {
        if self.DEBUG { println!("-----------------------------------"); }
        if self.DEBUG { println!("[DEBUG]: Function Call {}", idx); }
        let prev_str = str.clone();
        let expr: Vec<&str> = prev_str.split(" ").collect();
        if self.DEBUG {
            print!("[EXPR]: ");
            for i in 0..expr.len() {
                print!(" {:?}", expr[i]);
            }
            println!();
        }

        let mut term: bool = false;

        let grammar = self.ctx_gr.get_grammar().clone();

        for i in 0..expr.len() {
            if expr.len() == 1 {
                self.rw.push(str.clone());
                println!("[FINAL]: {}", str);
                return;
            }
            let op = expr[i];
            if !grammar.contains_key(op) { continue; }
            if self.DEBUG { println!("[ OP ]:  {}", op); }
            let rw_list = grammar.get(op).unwrap();

            for k in 0..rw_list.len() {
                let rw = &rw_list[k];
                if self.DEBUG { println!("[SSTR]:  {}", str); }
                if self.DEBUG { println!("[ RW ]:  {}", rw); }
                /**
                Regex will solve indistinct eclass match in str.replacen()
                Original Code
                ```
                str = str.replacen(op, &*rw, 1);
                ```
                Using Regex (has performance issue since it's slow)
                ```
                # use regex::Regex;
                let mat = Regex::new(format!(r"\b{}\b", op).as_str()).unwrap().find(str.as_str()).unwrap();
                str.replace_range(mat.start()..mat.end(), &rw);
                ```
                 */
                self.distinct_replace(op, rw, &mut str);
                if self.DEBUG { println!("[AFTER]: {}", str); }

                if str.len() >= self.max_rw_len as usize {
                    if self.DEBUG { println!("[DEBUG]: STR exceeds length limit, Try another RW..."); }
                    str = prev_str.clone();
                    continue;
                }
                if !str.contains('e') && k == rw_list.len()-1 {
                    self.rw.push(str.clone());
                    println!("[FINAL]: {}", str);
                    term = true;
                    break;
                } else if !str.contains('e') {
                    self.rw.push(str.clone());
                    str = prev_str.clone();
                    println!("[FINAL]: {}", str);
                } else {
                    self.csg_extract(str.clone(), idx+1);
                    if self.DEBUG { println!("[DEBUG]: Back to Function Call {}", idx); }
                    str = prev_str.clone();
                }
            }
            if term { break; }
        }
        if self.DEBUG { println!("[DEBUG]: Finish Function Call {}", idx); }
        if self.DEBUG { println!("-----------------------------------"); }
    }

    /// ## private member function to extract all equivalent mathematical expressions
    /// ## Context-Free Grammar
    /// ## Argument
    /// * `self`
    /// * `str` - rewrite expression
    /// * `idx` - fn call idx for debugging purpose
    fn cfg_extract(&mut self, mut str: String, idx: u8) {
        if self.DEBUG { println!("-----------------------------------"); }
        if self.DEBUG { println!("[DEBUG]: Function Call {}", idx); }
        let prev_str = str.clone();
        let expr: Vec<&str> = prev_str.split(" ").collect();
        if self.DEBUG {
            print!("[EXPR]: ");
            for i in 0..expr.len() {
                print!(" {:?}", expr[i]);
            }
            println!();
        }

        let mut term: bool = false;

        let grammar = self.ctx_gr.get_grammar().clone();

        for i in 0..expr.len() {
            if expr.len() == 1 {
                self.rw.push(str.clone());
                println!("[FINAL]: {}", str);
                return;
            }
            let op = expr[i];
            if !grammar.contains_key(op) { continue; }
            if self.DEBUG { println!("[ OP ]:  {}", op); }
            let rw_list = grammar.get(op).unwrap();

            for k in 0..rw_list.len() {
                let rw = &rw_list[k];
                if self.DEBUG { println!("[SSTR]:  {}", str); }
                if self.DEBUG { println!("[ RW ]:  {}", rw); }
                /**
                Regex will solve indistinct eclass match in str.replacen()
                Original Code
                ```
                str = str.replacen(op, &*rw, 1);
                ```
                Using Regex (has performance issue since it's slow)
                ```
                # use regex::Regex;
                let mat = Regex::new(format!(r"\b{}\b", op).as_str()).unwrap().find(str.as_str()).unwrap();
                str.replace_range(mat.start()..mat.end(), &rw);
                ```
                 */
                self.distinct_replace(op, rw, &mut str);
                if self.DEBUG { println!("[AFTER]: {}", str); }

                if str.len() >= self.max_rw_len as usize {
                    if self.DEBUG { println!("[DEBUG]: STR exceeds length limit, Try another RW..."); }
                    str = prev_str.clone();
                    continue;
                }
                if !str.contains('e') && k == rw_list.len()-1 {
                    self.rw.push(str.clone());
                    println!("[FINAL]: {}", str);
                    term = true;
                    break;
                } else if !str.contains('e') {
                    self.rw.push(str.clone());
                    str = prev_str.clone();
                    println!("[FINAL]: {}", str);
                } else {
                    self.cfg_extract(str.clone(), idx+1);
                    if self.DEBUG { println!("[DEBUG]: Back to Function Call {}", idx); }
                    str = prev_str.clone();
                    if k == rw_list.len()-1 {
                        term = true;
                        break;
                    }
                }
            }
            if term { break; }
        }
        if self.DEBUG { println!("[DEBUG]: Finish Function Call {}", idx); }
        if self.DEBUG { println!("-----------------------------------"); }
    }

    /// ## member function to start extraction
    /// ## Context-Free Grammar
    /// ## Argument
    /// * `self`
    pub fn extract(&mut self) {
        match self.csg {
            true => {
                println!("\n[INFO]: Start context-sensitive grammar extraction...");
                let init_rw = self.ctx_gr.get_init_rw().clone();
                for i in 0..init_rw.len() {
                    println!("\n[INFO]: Extracting with No.{} initial rewrite {}...", i+1, init_rw[i]);
                    self.csg_extract(init_rw[i].clone(), 0);
                }
                println!("\n[INFO]: Finish context-sensitive grammar extraction\n");
            },
            false => {
                println!("\n[INFO]: Start context-free grammar extraction...");
                let init_rw = self.ctx_gr.get_init_rw().clone();
                for i in 0..init_rw.len() {
                    println!("\n[INFO]: Extracting with No.{} initial rewrite {}...", i+1, init_rw[i]);
                    self.cfg_extract(init_rw[i].clone(), 0);
                }
                // self.cfg_extract("/ e3 e1".to_string().clone(), 0);
                println!("\n[INFO]: Finish context-free grammar extraction\n");
            },
        }
    }
}