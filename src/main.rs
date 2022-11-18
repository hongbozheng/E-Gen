use egg::{RecExpr, Runner, Extractor, AstSize, Language};

mod math;

pub fn main() {
    println!("Hello World!");
    let expression: &str = "(* x y)";
    let expr: RecExpr<math::Math> = expression.parse().unwrap();
    let runner = Runner::default().with_expr(&expr).run(&math::math_rule());
    let egraph = runner.egraph;
    let iterations = runner.iterations;
    let roots = runner.roots;
    let stop_reason = runner.stop_reason;
    println!("Total Size of {}",egraph.total_size());
    println!("{} node(s)",egraph.total_number_of_nodes());
    println!("{} class(es)",egraph.number_of_classes());
    let eclasses = egraph.classes();
    println!("\nEClass");
    // equiv_expr::get_equiv_expr(eclasses);
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
    println!("Iterations");
    for iteration in &iterations {
        println!("{:?}",iteration);
    }
    print!("\nRoots ");
    for root in &roots {
        print!("{:?}",root);
    }
    println!("\nRoot Eclass ID = {}",roots[0]);
    println!("\nStop Reason {:?}",stop_reason.unwrap());
    let extractor = Extractor::new(&egraph,AstSize);
    //let find_cost = extractor.find_costs();
    let (best_cost,simplified_expr) = extractor.find_best(roots[0]);
    println!("Simplified Expression {} to {} with Cost {}",expr,simplified_expr,best_cost);
}