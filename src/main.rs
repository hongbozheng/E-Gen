use egg::{ContextGrammar, Language, Math, math_rule, MathEGraph, RecExpr, Runner};


pub fn main() {
    let expr: &str = "(* x y)";
    println!("[INFO]: Initial expression {}", expr);
    let recexpr: RecExpr<Math> = expr.parse().unwrap();
    let runner = Runner::default().with_expr(&recexpr).run(&math_rule());
    let egraph: MathEGraph = runner.egraph;
    let iters = runner.iterations;
    let roots = runner.roots;
    let stop_reason = runner.stop_reason;
    println!("[INFO]: EGraph total size {}", egraph.total_size());
    println!("[INFO]: EGraph contains {} node(s)", egraph.total_number_of_nodes());
    println!("[INFO]: EGraph contains {} eclass(es)\n", egraph.number_of_classes());
    let eclasses = egraph.classes();
    println!("[INFO]: EClass Information");
    for eclass in eclasses {
        println!("{:?}\n",eclass);
        let id = &eclass.id;
        let enodes = &eclass.nodes;
        println!("enodes in eclass id: {}",id);
        for enode in enodes {
            println!("{}",enode);
            let children = enode.children();
            if children.is_empty() {println!("children node(s): None");}
            else {println!("children node(s): {:?}",children);}
        }
        println!("\n");
    }
    // println!("Iterations");
    // for iter in &iters {
    //     println!("{:?}",iter);
    // }

    print!("\n[INFO]: Runner Root(s)");
    for root in &roots {
        print!(" {:?}",root);
    }
    println!("\n[INFO]: Root EClass ID {}\n", roots[0]);
    // println!("\n[INFO]: Stop Reason {:?}",stop_reason.unwrap());
    // let extractor = Extractor::new(&egraph,AstSize);
    // //let find_cost = extractor.find_costs();
    // let (best_cost,simplified_expr) = extractor.find_best(roots[0]);
    // println!("Simplified Expression {} to {} with Cost {}",expr,simplified_expr,best_cost);
    //
    // println!("--------------------------------------------------\n");

    let mut ctx_g = ContextGrammar::new(egraph);
    println!("[INFO]: Creating grammar...");
    ctx_g.set_grammar();
    println!("[INFO]: Finish creating grammar\n");

    println!("[INFO]: ----- Grammar -----");
    let grammar = ctx_g.get_grammar();
    for (eclass, rewrite) in grammar {
        println!("[INFO]: {} -> {:?}", eclass, rewrite);
    }
}