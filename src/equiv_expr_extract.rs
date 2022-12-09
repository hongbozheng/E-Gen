use std::ops::Index;
use strum::IntoEnumIterator; // 0.17.1
// use enum_iterator::{sequence};
use crate::*;

pub struct ContextGrammar {
    egraph: MathEGraph,                     /* egraph after running rewrite rules           */
    init_expr: &'static str,                /* initial expression to run with egraph        */
    root_classes: Vec<Id>,                  /* root classes of MathEGraph                   */
    /* cannot use &str same reason as below */
    grammar: HashMap<String, Vec<String>>,  /* hashmap storing the grammar from egraph      */
    var: Vec<char>,                         /* vec storing variables to skip in extract fn  */
    init_rw: String,                        /* initial rw e.g. (* e0 e1)                    */
    rw: Vec<String>                         /* vec storing final rewrite                    */
}

impl ContextGrammar {
    /// ## default constructor
    ///
    /// ## Arguments
    /// * `MathEGraph` - egraph after running rewrite rules
    /// * `init_expr`  - initial expression to run with egraph
    /// * `root_classes` - root classes of MathEGraph
    pub fn new(egraph: MathEGraph, init_expr: &'static str, root_classes: Vec<Id>) -> Self {
        ContextGrammar {
            egraph,
            init_expr,
            root_classes,
            grammar: Default::default(),
            var: vec![],
            init_rw: "".to_string(),
            rw: vec![],
        }
    }

    // pub fn set_operator(&self) {
    //     for operator in Math::iter() {
    //         println!("{:?}", operator);
    //     }
    // }

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
            // println!("ec {}", ec);
            let enodes = &eclass.nodes;
            for enode in enodes {
                let mut rewrite = enode.to_string();
                let children = enode.children();
                // println!("children {:?}", children);
                // let mut rewrite = children.
                for child in children {
                    rewrite = format!("{} {}{}", rewrite, "e", child);
                }
                // println!("rw {}", rewrite);
                rewrite_rules.push(rewrite);
            }
            // for rw in rewrite_rules {
            //     println!("rw {}", rw);
            // }
            self.grammar.insert(ec, rewrite_rules);
        }

    }

    /// ## member function to get grammar from self
    /// ## Argument
    /// * `self`
    pub fn get_grammar(&self) -> HashMap<String, Vec<String>>{
        return self.grammar.clone();
    }

    /// ## member function to set variables from self
    /// ## Argument
    /// * `self`
    pub fn set_var(&mut self) {
        for char in self.init_expr.chars() {

        }
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

    /// ## member function to check if the current operand is const
    /// ## Argument
    /// * `self`
    /// TODO: may have to change to String since the reference issue
    /// * `str` - current checking operand
    pub fn is_const(&self, op: &str) -> bool {
        for char in op.chars() {
            if !char.is_numeric() {
                return false;
            }
        }
        return true;
    }

    /// ## member function to extract all equivalent mathematical expressions
    /// ## Argument
    /// * `self`
    /// * `str` - rewrite expression
    /// * `idx` - fn call idx for debugging purpose
    pub fn cfg_extract(&self, str: String, idx: u8) {
        /* TODO: Implement Context-Free Grammar */
        let expr = self.init_rw.split_whitespace();
        for op in expr {
            println!("{}", op);
            if self.is_const(op) {
                continue;
            }
        }
    }
}