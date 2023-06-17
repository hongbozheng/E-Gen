// use std::default;
// use std::fmt::format;
// use std::fs::{File, OpenOptions};
// use std::hash::Hash;
// use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::*;

/// Context Grammar Struct
/// store information about initial expression,
/// egraph, root eclass(es), skip eclass(es),
/// grammar, initial rewrite
pub struct ContextGrammar<'a> {
    /// initial expression to run with egraph
    expr: &'a str,
    /// egraph after running rewrite rules
    pub egraph: MathEGraph,
    /// root eclasses of MathEGraph
    pub root_ecls: Vec<Id>,
    /// eclass(es) to skip during extract
    pub skip_ecls: HashMap<String, f64>,
    /// grammar generated from e-graph
    pub grammar: HashMap<String, Vec<String>>,
    /// initial rw e.g. (* e0 e1)
    pub init_rw: Vec<String>,
}

impl<'a> ContextGrammar<'a> {
    /// ## default constructor
    /// ## Arguments
    /// * `init_expr` - initial expression for rewriting
    /// ## Return
    /// * `None`
    pub fn new(expr: &'a str) -> Self {
        ContextGrammar {
            expr,
            egraph: Default::default(),
            root_ecls: vec![],
            skip_ecls: Default::default(),
            grammar: Default::default(),
            init_rw: vec![],
        }
    }

    /// ## member function to set egraph and root_eclasses
    /// ## Argument
    /// * `self`
    /// ## Return
    /// * `None`
    pub fn setup(&mut self) {
        /* parse initial expression and create initial e-graph */
        let recexpr = self.expr.parse().unwrap();
        let runner = Runner::default().with_expr(&recexpr);

        /* equality saturation */
        let runner = runner.run(&math_rule());

        self.egraph = runner.egraph;
        self.root_ecls = runner.roots;
        let eclasses = self.egraph.classes();

        /* setup member variables skip_ecls and grammar */
        for eclass in eclasses {
            let mut rewrite_rules: Vec<String> = vec![];
            let ecls: String = format!("{}{}", "e", eclass.id);
            let enodes = &eclass.nodes;
    
            if enodes.len() == 1 {
                match enodes[0].to_string().parse::<f64>() {
                    Ok(float64) => {
                        if float64 == 1.0 || float64 == 0.0 {
                            self.skip_ecls.insert(ecls.clone(), float64);
                        }
                    },
                    Err(_) => {},
                }
            }
    
            for enode in enodes {
                let mut rewrite = enode.to_string();
                let children = enode.children();
                for child in children {
                    rewrite = format!("{} {}{}", rewrite, "e", child);
                }
                rewrite_rules.push(rewrite);
            }
            self.grammar.insert(ecls, rewrite_rules);
        }

        /* setup the member variable init_rw */
        for rc in &self.root_ecls {
            let mut root_ecls = format!("{}{}", "e", rc);
            if self.grammar.contains_key(&*root_ecls) {
                self.init_rw = self.grammar.get(&*root_ecls).unwrap().clone();
            } else {
                root_ecls = format!("{}{}", "e", self.egraph.find(*rc));
                self.init_rw = self.grammar.get(&*root_ecls).unwrap().clone();
            }
        }
    }

    /// ## member function to get an reference to egraph
    /// ## Argument
    /// * `self`
    /// ## Return
    /// * `egraph` - egraph
    pub fn get_egraph(&self) -> &MathEGraph { return &self.egraph; }

    /// ## member function to get root_eclasses
    /// ## Argument
    /// * `self`
    /// ## Return
    /// * `root_ecls` - root eclass(es) from egraph
    pub fn get_root_ecls(&self) -> &Vec<Id> { return &self.root_ecls; }

    pub fn get_skip_ecls(&self) -> &HashMap<String, f64> { return &self.skip_ecls; }

    pub fn get_grammar(&self) -> &HashMap<String, Vec<String>> { return &self.grammar; }

    /// ## member function to get the initial rewrite from self
    /// ## Argument
    /// * `self`
    /// ## Return
    /// * `init_rw` - initial rewrite rule(s)
    pub fn get_init_rw(&self) -> &Vec<String> { return &self.init_rw; }
}