use crate::{rewrite as rw, *};
use ordered_float::NotNan;

pub type MathEGraph = crate::EGraph<Math, ConstantFold>;
pub type Rewrite = crate::Rewrite<Math, ConstantFold>;

pub type Constant = NotNan<f64>;

define_language! {
    pub enum Math {
        "+"=Add([Id;2]),
        "-"=Sub([Id;2]),
        "*"=Mul([Id;2]),
        "/"=Div([Id;2]),

        "pow" =Pow([Id;2]),
        "sqrt"=Sqrt(Id),
        "ln"=Ln(Id),

        "d"=Diff([Id;2]),
        "i"=Integral([Id;2]),

        "sin" = Sin(Id),
        "cos" = Cos(Id),

        Constant(Constant),
        Symbol(Symbol),
    }
}

#[derive(Default)]
pub struct ConstantFold;
impl Analysis<Math> for ConstantFold {
    type Data = Option<(Constant, PatternAst<Math>)>;

    fn make(egraph: &MathEGraph, enode: &Math) -> Self::Data {
        let x = |i: &Id| egraph[*i].data.as_ref().map(|d| d.0);
        Some(match enode {
            Math::Constant(c) => (*c, format!("{}", c).parse().unwrap()),
            Math::Add([a, b]) => (
                x(a)? + x(b)?,
                format!("(+ {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Math::Sub([a, b]) => (
                x(a)? - x(b)?,
                format!("(- {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Math::Mul([a, b]) => (
                x(a)? * x(b)?,
                format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Math::Div([a, b]) if x(b) != Some(NotNan::new(0.0).unwrap()) => (
                x(a)? / x(b)?,
                format!("(/ {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            _ => return None,
        })
    }

    fn merge(&mut self, a: &mut Self::Data, b: Self::Data) -> DidMerge {
        match (a.as_mut(), &b) {
            (None, None) => DidMerge(false, false),
            (None, Some(_)) => {
                *a = b;
                DidMerge(true, false)
            }
            (Some(_), None) => DidMerge(false, true),
            (Some(_), Some(_)) => DidMerge(false, false),
        }
        // if a.is_none() && b.is_some() {
        //     *a = b
        // }
        // cmp
    }

    fn modify(egraph: &mut MathEGraph, id: Id) {
        let class = egraph[id].clone();
        if let Some((c, pat)) = class.data {
            if egraph.are_explanations_enabled() {
                egraph.union_instantiations(
                    &pat,
                    &format!("{}", c).parse().unwrap(),
                    &Default::default(),
                    "constant_fold".to_string(),
                );
            } else {
                let added = egraph.add(Math::Constant(c));
                egraph.union(id, added);
            }
            // to not prune, comment this out
            egraph[id].nodes.retain(|n| n.is_leaf());

            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}

fn not_zero(var: &str) -> impl Fn(&mut MathEGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        if let Some(n) = &egraph[subst[var]].data {
            *(n.0) != 0.0
        } else {
            true
        }
    }
}

/*
 * Documentation of rewrite
 * https://docs.rs/egg/0.7.1/egg/macro.rewrite.html
 */
pub fn math_rule() -> Vec<Rewrite> {
    vec![//rw!("commutative-addition"; "(+ ?x ?y)" => "(+ ?y ?x)"),
         //rw!("commutative-multiplication"; "(* ?x ?y)" => "(* ?y ?x)"),
         //rw!("associative-addition"; "(+ ?x (+ ?y ?z))" => "(+ (+ ?x ?y) ?z)"),
         //rw!("associative-multiplication"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),
         //rw!("add-0"; "(+ ?x 0)" => "?x"),
         rw!("add-0-expansion"; "?x" => "(+ ?x 0)"),
         //rw!("multiply-0"; "(* ?x 0)" => "0"),
         //rw!("multiply-1"; "(* ?x 1)" => "?x"),
         //rw!("multiply-1-expansion"; "?x" => "(* ?x 1)"),
         //rw!("distributive"; "(* ?x (+ ?y ?z))" => "(+ (* ?x ?y) (* ?x ?z))"),
         //rw!("factorization"; "(+ (* ?x ?y) (* ?x ?z))" => "(* ?x (+ ?y ?z))"),
         //rw!("subtraction_cancel"; "(- ?x ?x)" => "0"),
         //rw!("division_cancel"; "(/ ?x ?x)" => "1" if not_zero("?x")),
         //rw!("pow(1)"; "(pow ?x 1)" => "?x"),
         //rw!("pow(1)_"; "?x" => "(pow ?x 1)"),
         //rw!("pow(+)"; "(* (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (+ ?y ?z))"),
         //rw!("d(sin)"; "(d ?x (sin ?x))" => "(cos ?x)"),
         //rw!("d(cos)"; "(d ?x (cos ?x))" => "(* -1 (sin ?x))"),
         //rw!("idk"; "(/ (* ?x ?y) ?z)" => "(* ?x (/ ?y ?z))"),
         //rw!("idk_"; "(+ (- ?x ?y) ?z)" => "(+ ?x (- ?z ?y))")
    ]
}