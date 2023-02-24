use crate::*;

/// Context Grammar Struct
/// store information about initial expression,
/// egraph, root eclass(es), skip eclass(es),
/// grammar, initial rewrite
pub struct ContextGrammar {
    init_expr: &'static str,                /* initial expression to run with egraph        */
    egraph: MathEGraph,                     /* egraph after running rewrite rules           */
    root_ecls: Vec<Id>,                     /* root eclasses of MathEGraph                  */
    skip_ecls: HashMap<String, f64>,        /* eclass(es) to skip during extraction         */
    grammar: HashMap<String, Vec<String>>,  /* hashmap storing the grammar from egraph      */
    init_rw: Vec<String>,                   /* initial rw e.g. (* e0 e1)                    */
}

impl ContextGrammar {
    /// ## default constructor
    /// ## Arguments
    /// * `init_expr` - initial expression for rewriting
    pub fn new(init_expr: &'static str) -> Self {
        ContextGrammar {
            init_expr,
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
    pub fn set_egraph(&mut self) {
        let recexpr = self.init_expr.parse().unwrap();
        let runner = Runner::default().with_expr(&recexpr);
        self.egraph = runner.egraph;
        pt_egraph_info(&self.egraph);
        let runner = Runner::default().with_expr(&recexpr).run(&math_rule());
        self.egraph = runner.egraph;
        println!("\n{:?}\n", self.egraph.lookup_expr_ids(&recexpr));
        self.root_ecls = runner.roots;
    }

    /// ## member function to get an reference to egraph
    /// ## Argument
    /// * `self`
    pub fn get_egraph(&self) -> &MathEGraph { return &self.egraph; }

    /// ## member function to get root_eclasses
    /// ## Argument
    /// * `self`
    pub fn get_root_ecls(&self) -> &Vec<Id> { return &self.root_ecls; }

    /// ## member function to set grammar from egraph
    /// ## Argument
    /// * `self`
    pub fn set_grammar(&mut self) {
        let eclasses = self.egraph.classes();
        for eclass in eclasses {
            let mut rewrite_rules: Vec<String> = vec![];
            let id = eclass.id;
            let ecls: String = format!("{}{}", "e", id);
            let enodes = &eclass.nodes;

            if enodes.len() == 1 {
                match enodes[0].to_string().parse::<f64>() {
                    Ok(float64) => {
                        if float64 == 1.0 || float64 == 0.0 {
                            self.skip_ecls.insert(ecls.clone(), float64);
                        }
                    },
                    Err(_) => {
                        log_error(format!("[fn set_grammar] Failed to convert {} to var type f64", enodes[0].to_string()).as_str());
                    },
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
    }

    /// ## member function to get skip_ecls from self
    /// ## Argument
    /// * `self`
    pub fn get_skip_ecls(&self) -> &HashMap<String, f64> { return &self.skip_ecls; }

    /// ## member function to get grammar from self
    /// ## Argument
    /// * `self`
    pub fn get_grammar(&self) -> &HashMap<String, Vec<String>> { return &self.grammar; }

    /// ## member function to set the initial rewrite from self
    /// ## Argument
    /// * `self`
    pub fn set_init_rw(&mut self) {
        for rc in &self.root_ecls {
            let mut root_ecls = format!("{}{}", "e", rc);
            if self.grammar.contains_key(&*root_ecls) {
                self.init_rw = self.grammar.get(&*root_ecls).unwrap().clone();
            } else {
                root_ecls = format!("{}{}", "e", self.egraph.find(*rc));
                self.init_rw = self.grammar.get(&*root_ecls).unwrap().clone();
            }
        }
        /* TODO: May still have to fix simplified to const issue here !!!!! */
        // let mut root_eclass = format!("{}{}", "e", "8");
        // self.init_rw = self.grammar.get(&*root_eclass).unwrap().clone();
    }

    /// ## member function to get the initial rewrite from self
    /// ## Argument
    /// * `self`
    pub fn get_init_rw(&self) -> &Vec<String> { return &self.init_rw; }
}