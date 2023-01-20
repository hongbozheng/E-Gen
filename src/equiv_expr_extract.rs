use crate::*;
// use regex::Regex;

pub struct ContextGrammar {
    csg: bool,                              /* context-sensitive grammar flag               */
    DEBUG: bool,                            /* debug flag                                   */
    max_rw_len: u8,                         /* maximum rewrite length                       */
    init_expr: &'static str,                /* initial expression to run with egraph        */
    egraph: MathEGraph,                     /* egraph after running rewrite rules           */
    root_eclasses: Vec<Id>,                 /* root eclasses of MathEGraph                   */
    grammar: HashMap<String, Vec<String>>,  /* hashmap storing the grammar from egraph      */
    init_rw: Vec<String>,                   /* initial rw e.g. (* e0 e1)                    */
    rw: Vec<String>                         /* vec storing final rewrite                    */
}

impl ContextGrammar {
    /// ## default constructor
    /// ## Arguments
    /// * `MathEGraph` - egraph after running rewrite rules
    /* TODO: init_expr not needed i think */
    /// * `init_expr`  - initial expression to run with egraph
    /// * `root_classes` - root classes of MathEGraph
    pub fn new(csg: bool, DEBUG: bool, max_rw_len: u8, init_expr: &'static str) -> Self {
        ContextGrammar {
            csg,
            DEBUG,
            max_rw_len,
            init_expr,
            egraph: Default::default(),
            root_eclasses: vec![],
            grammar: Default::default(),
            init_rw: vec![],
            rw: vec![],
        }
    }

    /// ## member function to set egraph and root_eclasses
    /// ## Argument
    /// * `self`
    pub fn set_egraph(&mut self) {
        let recexpr = self.init_expr.parse().unwrap();
        let runner = Runner::default().with_expr(&recexpr).run(&math_rule());
        self.egraph = runner.egraph;
        self.root_eclasses = runner.roots;
    }

    /// ## member function to get an reference to egraph
    /// ## Argument
    /// * `self`
    pub fn get_egraph(&self) -> &MathEGraph { return &self.egraph; }

    /// ## member function to get root_eclasses
    /// ## Argument
    /// * `self`
    pub fn get_root_eclasses(&self) -> Vec<Id> {
        return self.root_eclasses.clone();
    }

    /// ## member function to set grammar from egraph
    /// ## Argument
    /// * `self`
    pub fn set_grammar(&mut self) {
        let eclasses = self.egraph.classes();
        for eclass in eclasses {
            let mut rewrite_rules: Vec<String> = vec![];
            let id = eclass.id;
            let ec: String = format!("{}{}", "e", id);
            let enodes = &eclass.nodes;
            for enode in enodes {
                let mut rewrite = enode.to_string();
                let children = enode.children();
                for child in children {
                    rewrite = format!("{} {}{}", rewrite, "e", child);
                }
                rewrite_rules.push(rewrite);
            }
            self.grammar.insert(ec, rewrite_rules);
        }
    }

    /// ## member function to get grammar from self
    /// ## Argument
    /// * `self`
    pub fn get_grammar(&self) -> HashMap<String, Vec<String>>{
        return self.grammar.clone();
    }

    /// ## member function to set the initial rewrite from self
    /// ## Argument
    /// * `self`
    pub fn set_init_rw(&mut self) {
        let mut root_eclass = format!("{}{}", "e", self.root_eclasses[0]);
        if self.grammar.contains_key(&*root_eclass) {
            self.init_rw = self.grammar.get(&*root_eclass).unwrap().clone()
        } else {
            root_eclass = format!("{}{}", "e", self.egraph.find(self.root_eclasses[0]));
            self.init_rw = self.grammar.get(&*root_eclass).unwrap().clone();
        }
    }

    /// ## member function to get the initial rewrite from self
    /// ## Argument
    /// * `self`
    pub fn get_init_rw(&self) -> Vec<String> { return self.init_rw.clone(); }

    /// ## member function to get the final rewrites from self
    /// ## Argument
    /// * `self`
    pub fn get_rw(&self) -> Vec<String> { return self.rw.clone(); }

    /// ## private member function to replace distinct eclass with rewrite rule
    /// ## Argument
    /// * `self`
    /// * `op`  - operand that needs to be replaced
    /// * `rw`  - rewrite rule that is going to be replaced with
    /// * `str` - original expression
    fn distinct_replace(&self, op: &str, rw: String, str: &mut String) {
        let matches: Vec<_> = str.match_indices(op).collect();
        for mat in matches {
            let start_idx = mat.0;
            let end_idx = start_idx + op.len();
            if (end_idx != str.len() && str.chars().nth(end_idx).unwrap() == ' ') ||
                end_idx == str.len() {
                str.replace_range(start_idx..end_idx, &*rw);
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
        let str_tmp = str.clone();
        let expr: Vec<&str> = str_tmp.split(" ").collect();
        if self.DEBUG {
            print!("[EXPR]: ");
            for i in 0..expr.len() {
                print!(" {:?}", expr[i]);
            }
            println!();
        }

        let mut term: bool = false;

        for i in 0..expr.len() {
            if expr.len() == 1 {
                self.rw.push(str.clone());
                println!("[FINAL]: {}", str);
                return;
            }
            let op = expr[i];
            if !self.grammar.contains_key(op) { continue; }
            if self.DEBUG { println!("[ OP ]:  {}", op); }
            let grammar = self.get_grammar();
            let rw_list = grammar.get(op).clone().unwrap();
            let prev_str = str.clone();

            for k in 0..rw_list.len() {
                let rw = rw_list[k].clone();
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
        let str_tmp = str.clone();
        let expr: Vec<&str> = str_tmp.split(" ").collect();
        if self.DEBUG {
            print!("[EXPR]: ");
            for i in 0..expr.len() {
                print!(" {:?}", expr[i]);
            }
            println!();
        }

        let mut term: bool = false;

        for i in 0..expr.len() {
            if expr.len() == 1 {
                self.rw.push(str.clone());
                println!("[FINAL]: {}", str);
                return;
            }
            let op = expr[i];
            if !self.grammar.contains_key(op) { continue; }
            if self.DEBUG { println!("[ OP ]:  {}", op); }
            let grammar = self.get_grammar();
            let rw_list = grammar.get(op).clone().unwrap();
            let prev_str = str.clone();

            for k in 0..rw_list.len() {
                let rw = rw_list[k].clone();
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

    pub fn extract(&mut self) {
        match self.csg {
            true => {
                println!("\n[INFO]: Start context-sensitive grammar extraction...");
                for i in 0..self.init_rw.len() {
                    println!("\n[INFO]: Extracting with No.{} initial rewrite {}...", i+1, self.init_rw[i]);
                    self.csg_extract(self.init_rw[i].clone(), 0);
                }
                println!("\n[INFO]: Finish context-sensitive grammar extraction\n");
            },
            false => {
                println!("\n[INFO]: Start context-free grammar extraction...");
                for i in 0..self.init_rw.len() {
                    println!("\n[INFO]: Extracting with No.{} initial rewrite {}...", i+1, self.init_rw[i]);
                    self.cfg_extract(self.init_rw[i].clone(), 0);
                }
                println!("\n[INFO]: Finish context-free grammar extraction\n");
            },
        }
    }
}