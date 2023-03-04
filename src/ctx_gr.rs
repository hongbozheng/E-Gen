use crate::*;

/// Context Grammar Struct
/// store information about initial expression,
/// egraph, root eclass(es), skip eclass(es),
/// grammar, initial rewrite
pub struct ContextGrammar {
    init_expr: &'static str,            /* initial expression to run with egraph        */
    pub(crate) egraph: MathEGraph,      /* egraph after running rewrite rules           */
    pub(crate) root_ecls: Vec<Id>,      /* root eclasses of MathEGraph                  */
    pub(crate) init_rw: Vec<String>,    /* initial rw e.g. (* e0 e1)                    */
}

impl ContextGrammar {
    /// ## default constructor
    /// ## Arguments
    /// * `init_expr` - initial expression for rewriting
    /// ## Return
    /// * `None`
    pub fn new(init_expr: &'static str) -> Self {
        ContextGrammar {
            init_expr,
            egraph: Default::default(),
            root_ecls: vec![],
            init_rw: vec![],
        }
    }

    /// ## member function to set egraph and root_eclasses
    /// ## Argument
    /// * `self`
    /// ## Return
    /// * `None`
    pub fn set_egraph(&mut self) {
        let recexpr = self.init_expr.parse().unwrap();
        let runner = Runner::default().with_expr(&recexpr);
        self.egraph = runner.egraph;
        // pt_egraph_info(&self.egraph);
        let runner = Runner::default().with_expr(&recexpr).run(&math_rule());
        self.egraph = runner.egraph;
        // println!("\n{:?}\n", self.egraph.lookup_expr_ids(&recexpr));
        self.root_ecls = runner.roots;
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

    /// ## member function to get the initial rewrite from self
    /// ## Argument
    /// * `self`
    /// ## Return
    /// * `init_rw` - initial rewrite rule(s)
    pub fn get_init_rw(&self) -> &Vec<String> { return &self.init_rw; }
}