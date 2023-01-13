use crate::*;

pub struct ContextGrammar {
    csg: bool,                              /* context-sensitive grammar flag               */
    DEBUG: bool,                            /* debug flag                                   */
    max_rw_len: u8,                         /* maximum rewrite length                       */
    init_expr: &'static str,                /* initial expression to run with egraph        */
    egraph: MathEGraph,                     /* egraph after running rewrite rules           */
    root_classes: Vec<Id>,                  /* root classes of MathEGraph                   */
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
            root_classes: vec![],
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
        self.root_classes = runner.roots;
    }

    /// ## member function to get an reference to egraph
    /// ## Argument
    /// * `self`
    pub fn get_egraph(&self) -> &MathEGraph { return &self.egraph; }

    /// ## member function to get root_eclasses
    /// ## Argument
    /// * `self`
    pub fn get_root_eclasses(&self) -> Vec<Id> {
        return self.root_classes.clone();
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
        let mut contains_class = false;
        for eclass in self.egraph.classes() {
            if self.root_classes[0] == eclass.id {
                contains_class = true;
            }
        }

        /* commutative rule will break program here; TODO: Solved */
        if contains_class {
            let root_eclass = format!("{}{}", "e", self.root_classes[0]);
            self.init_rw = self.grammar.get(&*root_eclass).unwrap().clone();
        } else {
            let root_eclass = format!("{}{}", "e", self.egraph.find(self.root_classes[0]));
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

    /// ## member function to extract all equivalent mathematical expressions
    /// ## Context-Sensitive Grammar
    /// ## Argument
    /// * `self`
    /// * `str` - rewrite expression
    /// * `idx` - fn call idx for debugging purpose
    pub fn csg_extract(&mut self, mut str: String, idx: u8) {
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
                str = str.replacen(op, &*rw, 1);
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

    /// ## member function to extract all equivalent mathematical expressions
    /// ## Context-Free Grammar
    /// ## Argument
    /// * `self`
    /// * `str` - rewrite expression
    /// * `idx` - fn call idx for debugging purpose
    pub fn cfg_extract(&mut self, mut str: String, idx: u8) {
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
                str = str.replacen(op, &*rw, 1);
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