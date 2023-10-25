use crate::*;
use std::collections::HashMap;
use std::time::Instant;

/// Context Grammar Struct
/// store information about initial expression,
/// egraph, root eclass(es), skip eclass(es),
/// grammar, initial rewrite
pub struct ContextGrammar {
    /// initial expression to run with egraph
    input_expr: String,
    /// egraph after running rewrite rules
    pub egraph: MathEGraph,
    /// root eclass(es) of MathEGraph
    pub root_eclasses: Vec<Id>,
    /// eclass(es) to skip during extract
    pub skip_eclasses: HashMap<String, f64>,
    /// grammar generated from e-graph
    pub grammar: HashMap<String, Vec<String>>,
    /// initial rw e.g. (* e0 e1)
    pub init_exprs: Vec<String>,
}

impl ContextGrammar {
    /// ### default constructor
    /// #### Arguments
    /// * `input_expr` - initial expression for rewriting
    /// #### Return
    /// * `None`
    pub fn new(input_expr: String) -> Self {
        ContextGrammar {
            input_expr,
            egraph: Default::default(),
            root_eclasses: vec![],
            skip_eclasses: Default::default(),
            grammar: Default::default(),
            init_exprs: vec![],
        }
    }

    /// ### member function to set egraph and root_eclasses
    /// #### Argument
    /// * `self`
    /// #### Return
    /// * `None`
    pub fn setup(&mut self) {
        /* parse initial expression and create initial e-graph */
        let start_time = Instant::now();
        let recexpr = self.input_expr.parse().unwrap();
        let runner = Runner::default().with_expr(&recexpr);

        /* equality saturation */
        let runner = runner.run(&math_rule());
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time).as_secs();
        log_info(&format!("E-graph Saturation {}s\n", elapsed_time));
        self.egraph = runner.egraph;
        self.root_eclasses = runner.roots;
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
                            self.skip_eclasses.insert(ecls.clone(), float64);
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
        for rc in &self.root_eclasses {
            let mut root_ecls = format!("{}{}", "e", rc);
            if self.grammar.contains_key(&*root_ecls) {
                self.init_exprs = self.grammar.get(&*root_ecls).unwrap().clone();
            } else {
                root_ecls = format!("{}{}", "e", self.egraph.find(*rc));
                self.init_exprs = self.grammar.get(&*root_ecls).unwrap().clone();
            }
        }
        log_debug_raw("\n");
        log_debug("------------- Grammar -------------\n");
        for (eclass, rewrite) in &self.grammar {
            log_debug(&format!("{} -> {:?}\n", &eclass, rewrite));
        }
        log_debug("-----------------------------------\n");

        log_info(&format!("Total # of grammars {}\n", self.grammar.len()));
        return;
    }

    /// ### member function to get an reference to egraph
    /// #### Argument
    /// * `self`
    /// #### Return
    /// * `egraph` - egraph
    pub fn get_egraph(&self) -> &MathEGraph { return &self.egraph; }

    /// ### member function to get root eclasses
    /// #### Argument
    /// * `self`
    /// #### Return
    /// * `root_eclasses` - root eclass(es) from egraph
    pub fn get_root_eclasses(&self) -> &Vec<Id> { return &self.root_eclasses; }

    /// ### member function to get skip eclasses
    /// #### Argument
    /// * `self`
    /// #### Return
    /// * `skip_eclasses` - skip eclass(es) from egraph
    pub fn get_skip_eclasses(&self) -> &HashMap<String, f64> { return &self.skip_eclasses; }

    /// ### member function to get grammars
    /// #### Argument
    /// * `self`
    /// #### Return
    /// * `grammar` - grammars created from egraph
    pub fn get_grammar(&self) -> &HashMap<String, Vec<String>> { return &self.grammar; }

    /// ### member function to get the initial rewrite from self
    /// #### Argument
    /// * `self`
    /// #### Return
    /// * `init_rw` - initial rewrite rule(s)
    pub fn get_init_rw(&self) -> &Vec<String> { return &self.init_exprs; }
}