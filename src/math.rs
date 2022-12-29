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
        "tan" = Tan(Id),

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

fn is_const(var: &str) -> impl Fn(&mut MathEGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| egraph[subst[var]].data.is_some()
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
    vec![
        /* commutative rules */
        /* does not work with commutative rules */
        // rw!("commutative-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        // rw!("commutative-mul"; "(* ?x ?y)" => "(* ?y ?x)"),
        // rw!("commutative-add_"; "(+ ?x (+ ?y ?z))" => "(+ (+ ?x ?y) ?z)"),
        // rw!("commutative-mul_"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),
        // rw!("commutative-mul-div"; "(/ (* ?x ?y) ?z)" => "(* ?x (/ ?y ?z))"),

        /* expansion */
        /* shouldn't comment the following rewrite rule since x needs to be expanded as (* x 1) */
        rw!("mul-1-exp"; "?x" => "(* 1 ?x)"),
        /* shouldn't comment the following rewrite rule since x needs to be expanded as (pow x 1) */
        rw!("pow(1)-exp"; "?x" => "(pow ?x 1)"),

        /* simplification (working) */
        /// ### temporary work around for commutative rules
        rw!("add-0-simpl"; "(+ ?x 0)" => "?x"),
        rw!("0-add-simpl"; "(+ 0 ?x)" => "?x"),
        rw!("mul-0-simpl"; "(* ?x 0)" => "0"),
        rw!("0-mul-simpl"; "(* 0 ?x)" => "0"),
        rw!("mul-1-simpl"; "(* ?x 1)" => "?x"),
        rw!("1-mul-simpl"; "(* 1 ?x)" => "?x"),
        rw!("pow-0-simpl"; "(pow ?x 0)" => "1" if not_zero("?x")),
        rw!("pow-1-simpl"; "(pow ?x 1)" => "?x"),
        rw!("sub_cancel"; "(- ?x ?x)" => "0"),
        rw!("div_cancel"; "(/ ?x ?x)" => "1" if not_zero("?x")),
        rw!("mul-(-1)"; "(* -1 -1)" => "1"),
        rw!("recip-mul-div"; "(* ?x (/ 1 ?x))" => "1" if not_zero("?x")),

        /* distributive property & factorization */
        // rw!("distrib"; "(* ?x (+ ?y ?z))" => "(+ (* ?x ?y) (* ?x ?z))"),
        rw!("fact"; "(+ (* ?a ?x) (* ?b ?x))" => "(* (+ ?a ?b) ?x)"),

        /* power */
        // rw!("pow(0)"; "(pow ?x 0)" => "1"),
        // rw!("pow(1)"; "(pow ?x 1)" => "?x"),
        // rw!("pow-mul"; "(* (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (+ ?y ?z))"),
        // rw!("pow-div"; "(/ (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (- ?y ?z))"),

        /* derivative */
        rw!("d-power-const"; "(d ?x (pow ?x ?c))" => "(* ?c (pow ?x (- ?c 1)))"
            if is_const("?c")),

        /* derivative distributive property */
        rw!("d-const*var-distrib"; "(d ?x (* ?c ?x))" => "(* ?c (d ?x ?x))" if is_const("?c")),
        //- rw!("d-add-distrib"; "(d ?x (+ ?y ?z))" => "(+ (d ?x ?y) (d ?x ?z))"),

        /* integration */
        // rw!("i-one"; "(i 1 ?x)" => "?x"),
        // rw!("i-power-const"; "(i (pow ?x ?c) ?x)" => "(/ (pow ?x (+ ?c 1)) (+ ?c 1))"
        //     if is_const("?c")),

        /* trig */
        // rw!("sin/cos"; "(/ (sin ?x) (cos ?x))" => "(tan ?x)"),
        /* trig derivative */
        // rw!("d(sin)"; "(d ?x (sin ?x))" => "(cos ?x)"),
        // rw!("d(cos)"; "(d ?x (cos ?x))" => "(* -1 (sin ?x))"),
        /* trig integration */
        // rw!("i-sin"; "(i (sin ?x) ?x)" => "(* -1 (cos ?x))"),
        // rw!("i-cos"; "(i (cos ?x) ?x)" => "(sin ?x)"),

        /* useless */
        // rw!("add-0-exp"; "?x" => "(+ ?x 0)"),
        // rw!("mul-1-simpl"; "(* ?x 1)" => "?x"),
    ]
}