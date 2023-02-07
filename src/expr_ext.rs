use crate::*;
// use regex::Regex;

/// Expression Extract Struct
/// extract equivalent mathematical expressions with
/// context-sensitive grammar / context-free grammar
pub struct ExpressionExtract {
    csg: bool,                              /* context-sensitive grammar flag               */
    max_rw_len: u8,                         /* maximum rewrite length                       */
    ctx_gr: ContextGrammar,                 /* context grammar struct                       */
    freq: HashMap<String, u16>,             /* rewrite rule frequency                       */
    rw: Vec<String>                         /* vec storing final rewrite                    */
}

impl ExpressionExtract {
    /// ## default constructor
    /// ## Arguments
    /// * `MathEGraph` - egraph after running rewrite rules
    /// * `init_expr`  - initial expression to run with egraph
    /// * `root_classes` - root classes of MathEGraph
    pub fn new(csg: bool, max_rw_len: u8, ctx_gr: ContextGrammar) -> Self {
        ExpressionExtract {
            csg,
            max_rw_len,
            ctx_gr,
            freq: Default::default(),
            rw: vec![],
        }
    }

    /// ## member function to get the final rewrites from self
    /// ## Argument
    /// * `self`
    pub fn get_rw(&self) -> &Vec<String> { return &self.rw; }

    // fn skip_rw(&self, rw: &String) -> bool {
    //
    // }

    /// ## member function to update the frequency of rewrite rules
    /// ## Argument
    /// `self`
    pub fn update_freq(&mut self, rw: &String, inc: bool) -> bool {
        if inc {
            if self.freq.contains_key(rw) && self.freq.get(rw).unwrap() < &(1 as u16) {
                *self.freq.get_mut(rw).unwrap() += 1;
            } else if self.freq.contains_key(rw) && self.freq.get(rw).unwrap() == &(1 as u16) {
                return true;
            } else {
                self.freq.insert(rw.clone(), 1);
            }
        } else {
            *self.freq.get_mut(rw).unwrap() -= 1;
        }
        // println!("{:?}", self.freq);
        return false;
    }

    // pub fn update_freq(&mut self, rw: &String, inc: bool) -> bool {
    //     if self.freq.contains_key(rw) {
    //         *self.freq.get_mut(rw).unwrap() += 1;
    //     } else {
    //         self.freq.insert(rw.clone(), 1);
    //     }
    //     println!("{:?}", self.freq);
    //     return false;
    // }

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

    /// ## private member function to check if eclass is in str
    /// ## Argument
    /// `str` - current str
    fn contain_eclass(&self, str: &String) -> bool {
        let matches: Vec<_> = str.match_indices('e').collect();
        for mat in matches {
            let start_idx = &mat.0;
            if str.chars().nth(start_idx-1).unwrap() == ' ' &&
                str.chars().nth(start_idx+1).unwrap().is_ascii_digit() {
                return true;
            }
        }
        return false;
    }

    /// ## private member function to extract all equivalent mathematical expressions
    /// ## Context-Sensitive Grammar
    /// ## Argument
    /// * `self`
    /// * `str` - rewrite expression
    /// * `idx` - fn call idx for debugging purpose
    fn csg_extract(&mut self, mut str: String, idx: u8) {
        log_trace("-----------------------------------\n");
        log_trace(format!("Function Call {}\n", idx).as_str());
        let prev_str = str.clone();
        let expr: Vec<&str> = prev_str.split(" ").collect();

        let mut term: bool = false;

        let grammar = self.ctx_gr.get_grammar().clone();

        for i in 0..expr.len() {
            if expr.len() == 1 {
                self.rw.push(str.clone());
                log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                return;
            }
            let op = expr[i];
            if !grammar.contains_key(op) { continue; }
            log_trace_raw(format!("[ OP ]:  {}\n", op).as_str());
            let rw_list = grammar.get(op).unwrap();

            for k in 0..rw_list.len() {
                let rw = &rw_list[k];
                log_trace_raw(format!("[INIT]:  {}\n", str).as_str());
                log_trace_raw(format!("[ RW ]:  {}\n", rw).as_str());
                #[warn(unused_doc_comments)]
                /// ```rust
                /// /* Regex will solve indistinct eclass match in str.replacen() */
                /// /* Original Code */
                /// str = str.replacen(op, &*rw, 1);
                /// /* Using Regex (has performance issue since it's slow) */
                /// use regex::Regex;
                /// let mat = Regex::new(format!(r"\b{}\b", op).as_str()).unwrap().find(str.as_str()).unwrap();                ///
                /// str.replace_range(mat.start()..mat.end(), &rw);
                /// ```
                if rw.contains('e') {
                    if self.update_freq(rw, true) {
                        // println!("[INFO]:  Freq exceeds limit, Switching RW...");
                        continue;
                    }
                }
                self.distinct_replace(op, rw, &mut str);
                log_trace_raw(format!("[AFTER]: {}\n", str).as_str());

                if str.len() >= self.max_rw_len as usize {
                    log_trace("STR exceeds length limit, Try another RW...\n");
                    if rw.contains('e') {
                        // println!("[INFO]:  Freq exceeds limit, try another rw...");
                        self.update_freq(rw, false);
                    }
                    str = prev_str.clone();
                    continue;
                }
                if !self.contain_eclass(&str) && k == rw_list.len()-1 {
                    self.rw.push(str.clone());
                    log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                    term = true;
                    break;
                } else if !self.contain_eclass(&str) {
                    self.rw.push(str.clone());
                    str = prev_str.clone();
                    log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                } else {
                    self.csg_extract(str.clone(), idx+1);
                    log_trace(format!("Back to Function Call {}\n", idx).as_str());
                    if rw.contains('e') {
                        // println!("[INFO]:  Freq exceeds limit, try another rw...");
                        self.update_freq(rw, false);
                    }
                    str = prev_str.clone();
                }
            }
            if term { break; }
        }
        log_trace(format!("Finish Function Call {}\n", idx).as_str());
        log_trace("-----------------------------------\n");
    }

    /// ## private member function to extract all equivalent mathematical expressions
    /// ## Context-Free Grammar
    /// ## Argument
    /// * `self`
    /// * `str` - rewrite expression
    /// * `idx` - fn call idx for debugging purpose
    fn cfg_extract(&mut self, mut str: String, idx: u8) {
        log_trace("-----------------------------------\n");
        log_trace(format!("Function Call {}\n", idx).as_str());
        let prev_str = str.clone();
        let expr: Vec<&str> = prev_str.split(" ").collect();

        let mut term: bool = false;

        let grammar = self.ctx_gr.get_grammar().clone();

        for i in 0..expr.len() {
            if expr.len() == 1 {
                self.rw.push(str.clone());
                log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                return;
            }
            let op = expr[i];
            if !grammar.contains_key(op) { continue; }
            log_trace_raw(format!("[ OP ]:  {}\n", op).as_str());
            let rw_list = grammar.get(op).unwrap();

            for k in 0..rw_list.len() {
                let rw = &rw_list[k];
                log_trace_raw(format!("[INIT]:  {}\n", str).as_str());
                log_trace_raw(format!("[ RW ]:  {}\n", rw).as_str());
                /// ```rust
                /// /* Regex will solve indistinct eclass match in str.replacen() */
                /// /* Original Code */
                /// str = str.replacen(op, &*rw, 1);
                /// /* Using Regex (has performance issue since it's slow) */
                /// use regex::Regex;
                /// let mat = Regex::new(format!(r"\b{}\b", op).as_str()).unwrap().find(str.as_str()).unwrap();                ///
                /// str.replace_range(mat.start()..mat.end(), &rw);
                /// ```
                self.distinct_replace(op, rw, &mut str);
                log_trace_raw(format!("[AFTER]: {}\n", str).as_str());

                if str.len() >= self.max_rw_len as usize {
                    log_trace("STR exceeds length limit, Try another RW...\n");
                    str = prev_str.clone();
                    continue;
                }
                if !str.contains('e') && k == rw_list.len()-1 {
                    self.rw.push(str.clone());
                    log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                    term = true;
                    break;
                } else if !str.contains('e') {
                    self.rw.push(str.clone());
                    str = prev_str.clone();
                    log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                } else {
                    self.cfg_extract(str.clone(), idx+1);
                    log_trace(format!("Back to Function Call {}\n", idx).as_str());
                    str = prev_str.clone();
                    if k == rw_list.len()-1 {
                        term = true;
                        break;
                    }
                }
            }
            if term { break; }
        }
        log_trace(format!("Finish Function Call {}\n", idx).as_str());
        log_trace("-----------------------------------\n");
    }

    /// ## member function to start extraction
    /// ## Context-Free Grammar
    /// ## Argument
    /// * `self`
    pub fn extract(&mut self) {
        match self.csg {
            true => {
                log_info_raw("\n");
                log_info("Start context-sensitive grammar extraction...\n");
                let init_rw = self.ctx_gr.get_init_rw().clone();
                for i in 0..init_rw.len() {
                    log_info_raw("\n");
                    log_info(format!("Extracting with No.{} initial rewrite {}...\n", i+1, init_rw[i]).as_str());
                    self.csg_extract(init_rw[i].clone(), 0);
                }
                log_info_raw("\n");
                log_info("Finish context-sensitive grammar extraction\n");
            },
            false => {
                log_info_raw("\n");
                log_info("Start context-free grammar extraction...\n");
                let init_rw = self.ctx_gr.get_init_rw().clone();
                for i in 0..init_rw.len() {
                    log_info(format!("Extracting with No.{} initial rewrite {}...\n", i+1, init_rw[i]).as_str());
                    self.cfg_extract(init_rw[i].clone(), 0);
                }
                // self.cfg_extract("/ e3 e1".to_string().clone(), 0);
                log_info_raw("\n");
                log_info("Finish context-free grammar extraction\n");
            },
        }
    }
}