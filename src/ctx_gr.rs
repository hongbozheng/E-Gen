use crate::*;

pub struct ContextGrammar {
    init_expr: &'static str,                /* initial expression to run with egraph        */
    egraph: MathEGraph,                     /* egraph after running rewrite rules           */
    root_ecls: Vec<Id>,                     /* root eclasses of MathEGraph                  */
    skip_ecls: Vec<String>,                 /* eclass(es) to skip during extraction         */
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
            skip_ecls: vec![],
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

    /// ## private member function to check if a enode is NotNan<f64>
    /// ## Argument
    /// * `enodes` - Vec of enodes
    fn ecls_skip(&self, enodes: &Vec<Math>) -> bool {
        if enodes.len() == 1 && (enodes[0].to_string().parse::<f64>().unwrap() == 1.0 ||
            enodes[0].to_string().parse::<f64>().unwrap() == 0.0) {
            return true;
        }
        return false;
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
            // if self.ecls_skip(enodes) { self.skip_ecls.push(ec.clone()); }
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

    /// ## member function to get skip_ecls from self
    /// ## Argument
    /// * `self`
    pub fn get_skip_ecls(&self) -> &Vec<String> { return &self.skip_ecls; }

    /// ## member function to get grammar from self
    /// ## Argument
    /// * `self`
    pub fn get_grammar(&self) -> &HashMap<String, Vec<String>> { return &self.grammar; }

    /// ## member function to set the initial rewrite from self
    /// ## Argument
    /// * `self`
    pub fn set_init_rw(&mut self) {
        for i in 0..self.root_ecls.len() {
            let mut root_ecls = format!("{}{}", "e", self.root_ecls[i]);
            if self.grammar.contains_key(&*root_ecls) {
                self.init_rw = self.grammar.get(&*root_ecls).unwrap().clone();
            } else {
                root_ecls = format!("{}{}", "e", self.egraph.find(self.root_ecls[i]));
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