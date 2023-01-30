use crate::*;

/// ## function to print egraph information
/// ## Argument
/// * `egraph` - egraph
pub fn pt_egraph_info(egraph: &MathEGraph) {
    println!("\n[DEBUG]: ------- EGraph Information -------");
    println!("[DEBUG]: ------------- EClass -------------");
    for eclass in egraph.classes() {
        println!("[DEBUG]: ------------ EClass {} ------------", eclass.id);
        for i in 0..eclass.nodes.len() {
            print!("[DEBUG]: enode {}", eclass.nodes[i]);
            for k in 0..eclass.nodes[i].children().len() {
                print!(" {}", eclass.nodes[i].children()[k]);
            }
            println!();
        }
        print!("[DEBUG]: parents");
        for k in 0..eclass.parents().len() {
            print!(" {:?}", eclass.parents().nth(k).unwrap());
        }
        println!("\n[DEBUG]: data  {:?}", eclass.data);
    }
    println!("[DEBUG]: ----------------------------------");
}

/// ## function to print root eclasses
/// ## Argument
/// * `root_eclasses` - root eclass vec<Id>
pub fn pt_root_ecls_info(root_ecls: &Vec<Id>) {
    println!("\n[INFO]: ---------- Root EClasses ----------");
    print!("[INFO]:");
    for i in 0..root_ecls.len() {
        print!(" {}", root_ecls[i]);
    }
    println!("\n[INFO]: -----------------------------------");
}

/// ## function to print grammar
/// ## Argument
/// * `grammar` - grammar HashMap
pub fn pt_grammar(grammar: &HashMap<String, Vec<String>>){
    println!("\n[INFO]: ------------- Grammar -------------");
    for (eclass, rewrite) in grammar {
        println!("[INFO]: {} -> {:?}", eclass, rewrite);
    }
    println!("[INFO]: -----------------------------------");
}

/// ## function to print initial rewrites
/// ## Argument
/// * `init_rw` - init_rw Vec<String>
pub fn pt_init_rw(init_rw: &Vec<String>) {
    println!("\n[INFO]: --------- Initial Rewrite ---------");
    println!("[INFO]: {:?}", init_rw);
    println!("[INFO]: -----------------------------------");
}