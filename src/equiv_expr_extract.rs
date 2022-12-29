use std::fmt::format;
use crate::*;

pub struct ContextGrammar {
    init_expr: &'static str,                /* initial expression to run with egraph        */
    DEBUG: bool,                            /* debug flag                                   */
    egraph: MathEGraph,                     /* egraph after running rewrite rules           */
    root_classes: Vec<Id>,                  /* root classes of MathEGraph                   */
    grammar: HashMap<String, Vec<String>>,  /* hashmap storing the grammar from egraph      */
    init_rw: String,                        /* initial rw e.g. (* e0 e1)                    */
    rw: Vec<String>                         /* vec storing final rewrite                    */
}

impl ContextGrammar {
    /// ## default constructor
    /// ## Arguments
    /// * `MathEGraph` - egraph after running rewrite rules
    /* TODO: init_expr not needed i think */
    /// * `init_expr`  - initial expression to run with egraph
    /// * `root_classes` - root classes of MathEGraph
    pub fn new(init_expr: &'static str, DEBUG: bool) -> Self {
        ContextGrammar {
            init_expr,
            DEBUG,
            egraph: Default::default(),
            root_classes: vec![],
            grammar: Default::default(),
            init_rw: "".to_string(),
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
        // for eclass in self.egraph.classes() {
        //     println!("[INFO]: {:?}",eclass);
        //     let id = &eclass.id;
        //     let enodes = &eclass.nodes;
        //     println!("enodes in eclass id: {}",id);
        //     for enode in enodes {
        //         println!("{}",enode);
        //         let children = enode.children();
        //         if children.is_empty() {println!("children node(s): None");}
        //         else {println!("children node(s): {:?}",children);}
        //     }
        //     println!("\n");
        // }
        // print!("\n[INFO]: Runner Root(s)");
        // for root in &self.root_classes {
        //     print!(" {:?}",root);
        // }
        // println!("\n[INFO]: Root EClass ID {}\n", &self.root_classes[0]);
        // let extractor = Extractor::new(&self.egraph, AstSize);
        // let (best_cost, simpl_expr) = extractor.find_best(self.root_classes[0]);
        // println!("Simplified Expression to {} with Cost {}",simpl_expr,best_cost);
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
            /* cannot use &str, since it may reference rewrite that has already been deallocated */
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
        let root_eclass_id = self.root_classes[0];
        let eclasses = self.egraph.classes();
        // for eclass in eclasses {
        //     let mut tmp = 0;
        //     let eclass_key = format!("{}{}", "e", eclass.id);
        //     let rw_vec = self.grammar.get(&*eclass_key).unwrap();
        //     if rw_vec.len() > tmp {
        //         self.init_rw = rw_vec[0].clone();
        //     }
        // }
        for eclass in eclasses {
            if eclass.id == root_eclass_id {
                let root_eclass = format!("{}{}", "e", root_eclass_id);
                self.init_rw = self.grammar.get(&*root_eclass).unwrap()[0].clone();
            }
        }
    }

    /// ## member function to get the initial rewrite from self
    /// ## Argument
    /// * `self`
    pub fn get_init_rw(&self) -> String { return self.init_rw.clone(); }

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

                if str.len() >= 20 {
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
    /// ## ## Context-Free Grammar
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

                if str.len() >= 25 {
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
}