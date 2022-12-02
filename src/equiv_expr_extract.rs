use std::ops::Index;
use crate::*;

pub struct ContextGrammar {
    egraph: MathEGraph,
    root_classes: Vec<Id>,
    grammar: HashMap<String, Vec<String>>, /* cannot use &str same reason as below */
    init_expr: String,    /* maybe use String ? idk */
    rw: Vec<String>
}

impl ContextGrammar {
    pub fn new(egraph: MathEGraph, root_classes: Vec<Id>) -> Self {
        ContextGrammar {
            egraph,
            root_classes,
            grammar: Default::default(),
            init_expr: "".to_string(),
            rw: vec![],
        }
    }

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

    pub fn get_grammar(&self) -> HashMap<String, Vec<String>>{
        return self.grammar.clone();
    }

    pub fn set_init_expr(&mut self) {
        let root_eclass_id = self.root_classes[0];
        let eclasses = self.egraph.classes();
        for eclass in eclasses {
            if eclass.id == root_eclass_id {
                let root_eclass = format!("{}{}", "e", root_eclass_id);
                self.init_expr = self.grammar.get(&*root_eclass).unwrap().index(1).clone();
            }
        }
    }

    pub fn get_init_expr(&self) -> String {
        return self.init_expr.clone();
    }

    pub fn cfg_extract(&self) {
        /* TODO: Implement Context-Free Grammar */
    }
}