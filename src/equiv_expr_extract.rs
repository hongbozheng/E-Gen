use std::ops::Index;
use strum::IntoEnumIterator; // 0.17.1
// use enum_iterator::{sequence};
use crate::*;

pub struct ContextGrammar {
    egraph: MathEGraph,                     /* egraph after running rewrite rules           */
    /* TODO: init_expr not needed i think */
    init_expr: &'static str,                /* initial expression to run with egraph        */
    root_classes: Vec<Id>,                  /* root classes of MathEGraph                   */
    DEBUG: bool,                            /* debug flag                                   */
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
    pub fn new(egraph: MathEGraph, init_expr: &'static str, root_classes: Vec<Id>, DEBUG: bool) -> Self {
        ContextGrammar {
            egraph,
            init_expr,
            root_classes,
            DEBUG,
            grammar: Default::default(),
            init_rw: "".to_string(),
            rw: vec![],
        }
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
        for eclass in eclasses {
            if eclass.id == root_eclass_id {
                let root_eclass = format!("{}{}", "e", root_eclass_id);
                self.init_rw = self.grammar.get(&*root_eclass).unwrap().index(1).clone();
            }
        }
    }

    /// ## member function to get the initial rewrite from self
    /// ## Argument
    /// * `self`
    pub fn get_init_rw(&self) -> String {
        return self.init_rw.clone();
    }

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