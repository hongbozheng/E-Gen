// use crate::*;
//
// struct ContextGrammar {
//     egraph: EGraph<dyn Language,dyn Analysis<dyn Language, Data=()>>,
//     init_expr: &'static str,    /* maybe use String ? idk */
//     grammar: HashMap<&'static str,Vec<&'static str>>,
//     rw: Vec<&'static str>
// }
//
// impl ContextGrammar {
//     pub fn new(egraph: EGraph<dyn Language,dyn Analysis<dyn Language, Data=()>>, init_expr: &str, grammar: HashMap<&'static str,Vec<&'static str>>, rw: Vec<&'static str>) -> Self {
//         ContextGrammar {
//             egraph,
//             init_expr,
//             grammar,
//             rw
//         }
//     }
//
//     pub fn set_grammar(&self) {
//
//     }
// }