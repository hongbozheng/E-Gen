use crate::*;

pub struct ContextGrammar {
    egraph: MathEGraph,
    init_expr: &'static str,    /* maybe use String ? idk */
    grammar: HashMap<String, Vec<String>>, /* cannot use &str same reason as below */
    rw: Vec<&'static str>
}

impl ContextGrammar {
    pub fn new(egraph: MathEGraph) -> Self {
        ContextGrammar {
            egraph,
            init_expr: "",
            grammar: Default::default(),
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
                println!("children {:?}", children);
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
}