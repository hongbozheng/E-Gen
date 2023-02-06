use crate::*;

/// ## function to print the type of a variable
/// ## Argument
/// * `_` - reference of any variable
pub fn pt_type_of<T>(_: &T) { println!("[DEBUG]: Var Type {}", std::any::type_name::<T>()); }

/// ## function to print egraph information
/// ## Argument
/// * `egraph` - egraph
pub fn pt_egraph_info(egraph: &MathEGraph) {
    println!("\n[DEBUG]: ------- EGraph Information -------");
    println!("[DEBUG]: ------------- EClass -------------");
    for eclass in egraph.classes() {
        println!("[DEBUG]: ------------ EClass {} ------------", eclass.id);
        for i in 0..eclass.nodes.len() {
            if eclass.nodes[i].to_string().parse::<f64>().is_ok() {
                print!("[DEBUG]: <f64> {:?}", eclass.nodes[i]);
            } else {
                print!("[DEBUG]: enode {}", eclass.nodes[i]);
                for k in 0..eclass.nodes[i].children().len() {
                    print!(" {}", eclass.nodes[i].children()[k]);
                }
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
    print!("[INFO]: {:?}", root_ecls);
    println!("\n[INFO]: -----------------------------------");
}

/// ## function to print eclass(es) to skip during extraction
/// ## Argument
/// * `skip_ecls` - vec<String> to skip during extraction
pub fn pt_skip_ecls(skip_ecls: &Vec<String>) {
    println!("\n[INFO]: ---------- Skip EClasses ----------");
    println!("[INFO]: {:?}", skip_ecls);
    println!("[INFO]: -----------------------------------");
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