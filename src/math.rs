use crate::{rewrite as rw, *};
use ordered_float::NotNan;

/// mathematical expression egraph
pub type MathEGraph = EGraph<Math, ConstantFold>;
pub type Rewrite = crate::Rewrite<Math, ConstantFold>;

pub type Constant = NotNan<f64>;

/* math operators */
define_language! {
    pub enum Math {
        "+"=Add([Id;2]),
        "-"=Sub([Id;2]),
        "*"=Mul([Id;2]),
        "/"=Div([Id;2]),

        "pow"=Pow([Id;2]),
        "sqrt"=Sqrt(Id),

        "d"=Diff([Id;2]),
        "i"=Integral([Id;2]),

        /* log & exponential */
        "exp"=Exp(Id),
        "ln"=Ln(Id),

        /* trig operator */
        "sin"=Sin(Id),
        "cos"=Cos(Id),
        "tan"=Tan(Id),
        "csc"=Csc(Id),
        "sec"=Sec(Id),
        "cot"=Cot(Id),
        "asin"=ASin(Id),
        "acos"=ACos(Id),
        "atan"=ATan(Id),
        "acsc"=ACsc(Id),
        "asec"=ASec(Id),
        "acot"=ACot(Id),

        /* hyperbolic */
        "sinh"=Sinh(Id),
        "cosh"=Cosh(Id),
        "tanh"=Tanh(Id),
        "csch"=Csch(Id),
        "sech"=Sech(Id),
        "coth"=Coth(Id),
        "asinh"=ASinh(Id),
        "acosh"=ACosh(Id),
        "atanh"=ATanh(Id),
        "acsch"=ACsch(Id),
        "asech"=ASech(Id),
        "acoth"=ACoth(Id),

        "abs"=Abs(Id),

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

/// mathematical rules including:
/// 1. basic arithmetic
/// 2. simplification
/// 3. expansion
/// 4. exponential
/// 5. trigonometry
/// 6. hyperbolic
/// 7. derivative
/// 8. integration
pub fn math_rule() -> Vec<Rewrite> {
    vec![
        /* commutative rules */
        // rw!("x+y->y+x"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        // rw!("x*y->y*x"; "(* ?x ?y)" => "(* ?y ?x)"),
        // rw!("comm-add-3var"; "(+ ?x (+ ?y ?z))" => "(+ (+ ?x ?y) ?z)"),
        // rw!("comm-mul-3var"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),
        // rw!("comm-mul-div"; "(/ (* ?x ?y) ?z)" => "(* ?x (/ ?y ?z))"),
        // rw!("x(y/z)->(xy)/z"; "(* ?x (/ ?y ?z))" => "(/ (* ?x ?y) ?z)"),

        // /* expansion */
        // rw!("x->x*1"; "?x" => "(* 1 ?x)"),
        rw!("x->x^1"; "?x" => "(pow ?x 1)"),

        // /* basic arithmetic simplification */
        // rw!("x+0->x"; "(+ ?x 0)" => "?x"),
        // rw!("x*0->0"; "(* ?x 0)" => "0"),
        // rw!("x*1->x"; "(* ?x 1)" => "?x"),
        // rw!("x/1->x"; "(/ ?x 1)" => "?x"),
        // rw!("x-x->0"; "(- ?x ?x)" => "0"),
        // rw!("x/x->1"; "(/ ?x ?x)" => "1" if not_zero("?x")),
        // rw!("(-1)*(-1)->1"; "(* -1 -1)" => "1"),
        // rw!("x*(1/x)->1"; "(* ?x (/ 1 ?x))" => "1" if not_zero("?x")),

        // /* distributive property & factorization */
        // rw!("distrib"; "(* ?x (+ ?y ?z))" => "(+ (* ?x ?y) (* ?x ?z))"),
        // rw!("fact"; "(+ (* ?a ?x) (* ?b ?x))" => "(* (+ ?a ?b) ?x)"),

        // /* multiplication <-> division identity */
        // rw!("x/(y/z)->x(z/y)"; "(/ ?x (/ ?y ?z))" => "(* ?x (/ ?z ?y))"),

        /* exponent rules */
        /* simplification */
        rw!("pow(0)"; "(pow ?x 0)" => "1"),
        rw!("pow(1)"; "(pow ?x 1)" => "?x"),
        /* basic rule */
        rw!("pow-of-prod"; "(* (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (+ ?y ?z))"),
        rw!("pow-of-quotient"; "(/ (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (- ?y ?z))"),
        rw!("pow-of-pow"; "(pow (pow ?x ?y) ?z)" => "(pow ?x (* ?y ?z))"),
        rw!("pow-of-(-1)"; "(pow ?x -1)" => "(/ 1 ?x)" if not_zero("?x")),
        /* ln */
        rw!("ln(x^y)->yln(x)"; "(ln (pow ?x ?y))" => "(* ?y (ln ?x))"),
        /* exponent derivative */
        rw!("d(lnx)"; "(d ?x (ln ?x))" => "(/ 1 ?x)"),

        /* integration */
        rw!("i-one"; "(i 1 ?x)" => "?x"),
        rw!("i-power-const"; "(i (pow ?x ?c) ?x)" => "(/ (pow ?x (+ ?c 1)) (+ ?c 1))"
            if is_const("?c")),

        /* trig */
        /* trig basic identity */
        // rw!("tan->sin/cos"; "(tan ?x)" => "(/ (sin ?x) (cos ?x))"),
        // rw!("cos->sin/tan"; "(cos ?x)" => "(/ (sin ?x) (tan ?x))"),
        // rw!("sin->cos*tan"; "(sin ?x)" => "(* (cos ?x) (tan ?x))"),
        // /*
        // don't need these rw because trig basic identity and trig reciprocal identity cover it
        // ```
        // rw!("sin/cos->tan"; "(/ (sin ?x) (cos ?x))" => "(tan ?x)"),
        // rw!("sin/tan->cos"; "(/ (sin ?x) (tan ?x))" => "(cos ?x)"),
        // rw!("cos*tan->sin"; "(* (cos ?x) (tan ?x))" => "(sin ?x)"),
        // rw!("cot->cos/sin"; "(cot ?x)" => "(/ (cos ?x) (sin ?x))"),
        // rw!("cos/sin->cot"; "(/ (cos ?x) (sin ?x))" => "(cot ?x)"),
        // ```
        //  */
        // /* trig reciprocal identity <-> */
        // rw!("csc->1/sin"; "(csc ?x)" => "(/ 1 (sin ?x))"),
        // rw!("sec->1/cos"; "(sec ?x)" => "(/ 1 (cos ?x))"),
        // rw!("cot->1/tan"; "(cot ?x)" => "(/ 1 (tan ?x))"),
        // rw!("1/sin->csc"; "(/ 1 (sin ?x))" => "(csc ?x)"),
        // rw!("1/cos->sec"; "(/ 1 (cos ?x))" => "(sec ?x)"),
        // rw!("1/tan->cot"; "(/ 1 (tan ?x))" => "(cot ?x)"),
        // /*
        // don't need these rw because trig reciprocal identity & multiplication <-> division identity
        // cover it
        // ```
        // rw!("sin->1/csc"; "(sin ?x)" => "(/ 1 (csc ?x))"),
        // rw!("cos->1/sec"; "(cos ?x)" => "(/ 1 (sec ?x))"),
        // rw!("tan->1/cot"; "(tan ?x)" => "(/ 1 (cot ?x))"),
        // rw!("1/csc->sin"; "(/ 1 (csc ?x))" => "(sin ?x)"),
        // rw!("1/sec->cos"; "(/ 1 (sec ?x))" => "(cos ?x)"),
        // rw!("1/cot->tan"; "(/ 1 (cot ?x))" => "(tan ?x)"),
        // ```
        //  */
        // /* pythagorean identity <-> */
        // rw!("sin^2+cos^2->1"; "(+ (pow (sin ?x) 2) (pow (cos ?x) 2))" => "1"),
        // rw!("tan^2+1->sec^2"; "(+ (pow (tan ?x) 2) 1)" => "(pow (sec ?x) 2)"),
        // rw!("cot^2+1->csc^2"; "(+ (pow (cot ?x) 2) 1)" => "(pow (csc ?x) 2)"),
        // // rw!("1->sin^2+cos^2"; "1" => "(+ (pow (sin ?x) 2) (pow (cos ?x) 2))"),
        // rw!("sec^2->tan^2+1->"; "(pow (sec ?x) 2)" => "(+ (pow (tan ?x) 2) 1)"),
        // rw!("csc^2->cot^2+1"; "(pow (csc ?x) 2)" => "(+ (pow (cot ?x) 2) 1)"),
        // /* even-odd identity <-> */
        // rw!("sin(-x)->-sin(x)"; "(sin (* -1 ?x))" => "(* -1 (sin ?x))"),
        // rw!("cos(-x)->cos(x)"; "(cos (* -1 ?x))" => "(cos ?x)"),
        // rw!("tan(-x)->-tan(x)"; "(tan (* -1 ?x))" => "(* -1 (tan ?x))"),
        // rw!("csc(-x)->-csc(x)"; "(csc (* -1 ?x))" => "(* -1 (csc ?x))"),
        // rw!("sec(-x)->sec(x)"; "(sec (* -1 ?x))" => "(sec ?x)"),
        // rw!("cot(-x)->-cot(x)"; "(cot (* -1 ?x))" => "(* -1 (cot ?x))"),
        // rw!("-sin(x)->sin(-x)"; "(* -1 (sin ?x))" => "(sin (* -1 ?x))"),
        // rw!("cos(x)->cos(-x)"; "(cos ?x)" => "(cos (* -1 ?x))"),
        // rw!("-tan(x)->tan(-x)"; "(* -1 (tan ?x))" => "(tan (* -1 ?x))"),
        // rw!("-csc(x)->csc(-x)"; "(* -1 (csc ?x))" => "(csc (* -1 ?x))"),
        // rw!("sec(x)->sec(-x)"; "(sec ?x)" => "(sec (* -1 ?x))"),
        // rw!("-cot(x)->cot(-x)"; "(* -1 (cot ?x))" => "(cot (* -1 ?x))"),
        // /* double angle identity */
        // rw!("sin(2x)->2sin(x)cos(x)"; "(sin (* 2 ?x))" => "(* 2 (* (sin ?x) (cos ?x)))"),
        // rw!("cos(2x)->cos^2-sin^2"; "(cos (* 2 ?x))" => "(- (pow (cos ?x) 2) (pow (sin ?x) 2))"),
        // rw!("cos(2x)->2cos^2-1"; "(cos (* 2 ?x))" => "(- (* 2 (pow (cos ?x) 2)) 1)"),
        // rw!("cos(2x)->1-2sin^2"; "(cos (* 2 ?x))" => "(- 1 (* 2 (pow (sin ?x) 2)))"),
        // rw!("tan(2x)->2tan(x)/(1-tan^2)"; "(tan (* 2 ?x))" => "(/ (* 2 (tan ?x)) (- 1 (pow (tan ?x) 2)))"),
        // rw!("2sin(x)cos(x)->sin(2x)"; "(* 2 (* (sin ?x) (cos ?x)))" => "(sin (* 2 ?x))"),
        // rw!("cos^2-sin^2->cos(2x)"; "(- (pow (cos ?x) 2) (pow (sin ?x) 2))" => "(cos (* 2 ?x))"),
        // rw!("2cos^2-1->cos(2x)"; "(- (* 2 (pow (cos ?x) 2)) 1)" => "(cos (* 2 ?x))"),
        // rw!("1-2sin^2->cos(2x)"; "(- 1 (* 2 (pow (sin ?x) 2)))" => "(cos (* 2 ?x))"),
        // rw!("2tan(x)/(1-tan^2)->tan(2x)"; "(/ (* 2 (tan ?x)) (- 1 (pow (tan ?x) 2)))" => "(tan (* 2 ?x))"),
        // /* half angle identity doesn't work */
        // // rw!("sin(x/2)=sqrt((1-cos(x))/2)"; "(sin (/ ?x 2))" => "(sqrt (/ (- 1 (cos ?x)) 2))"),
        // // rw!("sin(x/2)=-sqrt((1-cos(x))/2)"; "(sin (/ ?x 2))" => "(* -1 (sqrt (/ (- 1 (cos ?x)) 2)))"),
        // // rw!("cos(x/2)=sqrt((1+cos(x))/2)"; "(cos (/ ?x 2))" => "(sqrt (/ (+ 1 (cos ?x)) 2))"),
        // // rw!("cos(x/2)=-sqrt((1+cos(x))/2)"; "(cos (/ ?x 2))" => "(* -1 (sqrt (/ (+ 1 (cos ?x)) 2)))"),
        // // rw!("tan(x/2)=sqrt((1-cos(x))/(1+cos(x)))"; "(tan (/ ?x 2))" => "(sqrt (/ (- 1 (cos ?x)) (+ 1 (cos ?x))))"),
        // // rw!("tan(x/2)=-sqrt((1-cos(x))/(1+cos(x)))"; "(tan (/ ?x 2))" => "(* -1 (sqrt (/ (- 1 (cos ?x)) (+ 1 (cos ?x)))))"),
        // /* product to sum identity */
        // rw!("sin(a)sin(b)->(1/2)(cos(a-b)-cos(a+b))";
        //     "(* (sin ?x) (sisn ?y))" => "(* (/ 1 2) (- (cos (- ?x ?y)) (cos (+ ?x ?y))))"),
        // rw!("cos(a)cos(b)->(1/2)(cos(a-b)+cos(a+b))";
        //     "(* (cos ?x) (cos ?y))" => "(* (/ 1 2) (+ (cos (- ?x ?y)) (cos (+ ?x ?y))))"),
        // rw!("sin(a)cos(b)->(1/2)(sin(a+b)+sin(a-b))";
        //     "(* (sin ?x) (cos ?y))" => "(* (/ 1 2) (+ (sin (+ ?x ?y)) (sin (- ?x ?y))))"),
        // rw!("cos(a)sin(b)->(1/2)(sin(a+b)-sin(a-b))";
        //     "(* (cos ?x) (sin ?y))" => "(* (/ 1 2) (- (sin (+ ?x ?y)) (sin (- ?x ?y))))"),
        // /* sum to product identity */
        // rw!("sin(a)+sin(b)->2sin((a+b)/2)cos((a-b)/2)";
        //     "(+ (sin ?x) (sin ?y))" => "(* 2 (* (sin (/ (+ ?x ?y) 2)) (cos (/ (- ?x ?y) 2))))"),
        // rw!("sin(a)-sin(b)->2cos((a+b)/2)sin((a-b)/2)";
        //     "(- (sin ?x) (sin ?y))" => "(* 2 (* (cos (/ (+ ?x ?y) 2)) (sin (/ (- ?x ?y) 2))))"),
        // rw!("cos(a)+cos(b)->2cos((a+b)/2)cos((a-b)/2)";
        //     "(+ (cos ?x) (cos ?y))" => "(* 2 (* (cos (/ (+ ?x ?y) 2)) (cos (/ (- ?x ?y) 2))))"),
        // rw!("cos(a)-cos(b)->2sin((a+b)/2)sin((a-b)/2)";
        //     "(- (cos ?x) (cos ?y))" => "(* -2 (* (sin (/ (+ ?x ?y) 2)) (sin (/ (- ?x ?y) 2))))"),

        /* hyperbolic identity */
        /* basic identity */
        rw!("cosh(x)+sinh(x)->e^x"; "(+ (cosh ?x) (sinh ?x))" => "(exp ?x)"),
        rw!("cosh(x)-sinh(x)->e^(-x)"; "(- (cosh ?x) (sinh ?x))" => "(exp (* -1 ?x))"),
        /* pythagorean identity <-> */
        rw!("cosh^2(x)-sinh^2(x)->1"; "(- (pow (cosh ?x) 2) (pow (sinh ?x) 2))" => "1"),
        rw!("1-tanh^2->sech^2"; "(- 1 (pow (tanh ?x) 2))" => "(pow (sech ?x) 2)"),
        rw!("coth^2-1->csch^2"; "(- (pow (coth ?x) 2) 1)" => "(pow (csch ?x) 2)"),
        // rw!("1->cosh^2(x)-sinh^2(x)"; "1" => "(- (pow (cosh ?x) 2) (pow (sinh ?x) 2))"),
        rw!("sech^2->1-tanh^2->"; "(pow (sech ?x) 2)" => "(- 1 (pow (tanh ?x) 2))"),
        rw!("csch^2->coth^2-1"; "(pow (csch ?x) 2)" => "(- (pow (coth ?x) 2) 1)"),
        /* even-odd identity <-> */
        rw!("sinh(-x)->-sinh(x)"; "(sinh (* -1 ?x))" => "(* -1 (sinh ?x))"),
        rw!("cosh(-x)->cosh(x)"; "(cosh (* -1 ?x))" => "(cosh ?x)"),
        rw!("tanh(-x)->-tanh(x)"; "(tanh (* -1 ?x))" => "(* -1 (tanh ?x))"),
        rw!("csch(-x)->-csch(x)"; "(csch (* -1 ?x))" => "(* -1 (csch ?x))"),
        rw!("sech(-x)->sech(x)"; "(sech (* -1 ?x))" => "(sech ?x)"),
        rw!("coth(-x)->-coth(x)"; "(coth (* -1 ?x))" => "(* -1 (coth ?x))"),
        rw!("-sinh(x)->sinh(-x)"; "(* -1 (sinh ?x))" => "(sinh (* -1 ?x))"),
        rw!("cosh(x)->cosh(-x)"; "(cosh ?x)" => "(cosh (* -1 ?x))"),
        rw!("-tanh(x)->tanh(-x)"; "(* -1 (tanh ?x))" => "(tanh (* -1 ?x))"),
        rw!("-csch(x)->csch(-x)"; "(* -1 (csch ?x))" => "(csch (* -1 ?x))"),
        rw!("sech(x)->sech(-x)"; "(sech ?x)" => "(sech (* -1 ?x))"),
        rw!("-coth(x)->coth(-x)"; "(* -1 (coth ?x))" => "(coth (* -1 ?x))"),
        /* double angle identity */
        rw!("sinh(2x)->2sinh(x)cosh(x)"; "(sinh (* 2 ?x))" => "(* 2 (* (sinh ?x) (cosh ?x)))"),
        rw!("cosh(2x)->sinh^2+cosh^2"; "(cosh (* 2 ?x))" => "(+ (pow (sinh ?x) 2) (pow (cosh ?x) 2))"),
        rw!("cosh(2x)->2sinh^2+1"; "(cosh (* 2 ?x))" => "(+ (* 2 (pow (sinh ?x) 2)) 1)"),
        rw!("cosh(2x)->2cosh^2-1"; "(cosh (* 2 ?x))" => "(- (* 2 (pow (sinh ?x) 2)) 1)"),
        rw!("tanh(2x)->2tanh(x)/(1+tanh^2)"; "(tanh (* 2 ?x))" => "(/ (* 2 (tanh ?x)) (+ 1 (pow (tanh ?x) 2)))"),
        rw!("2sinh(x)cosh(x)->sinh(2x)"; "(* 2 (* (sinh ?x) (cosh ?x)))" => "(sinh (* 2 ?x))"),
        rw!("sinh^2+cosh^2->cosh(2x)"; "(+ (pow (sinh ?x) 2) (pow (cosh ?x) 2))" => "(cosh (* 2 ?x))"),
        rw!("2sinh^2+1->cosh(2x)"; "(+ (* 2 (pow (sinh ?x) 2)) 1)" => "(cosh (* 2 ?x))"),
        rw!("2cosh^2-1>cosh(2x)"; "(- (* 2 (pow (cosh ?x) 2)) 1)" => "(cosh (* 2 ?x))"),
        rw!("2tanh(x)/(1+tanh^2)->tanh(2x)"; "(/ (* 2 (tanh ?x)) (+ 1 (pow (tanh ?x) 2)))" => "(tanh (* 2 ?x))"),
        /* half angle identity */
        /* sum/difference identity */
        rw!("sinh(a+b)->sinh(a)cosh(b)+cosh(a)sinh(b)";
            "(sinh (+ ?x ?y))" => "(+ (* (sinh ?x) (cosh ?y)) (* (cosh ?x) (sinh ?y)))"),
        rw!("sinh(a-b)->sinh(a)cosh(b)-cosh(a)sinh(b)";
            "(sinh (- ?x ?y))" => "(- (* (sinh ?x) (cosh ?y)) (* (cosh ?x) (sinh ?y)))"),
        rw!("cosh(a+b)->cosh(a)cosh(b)+sinh(a)sinh(b)";
            "(cosh (+ ?x ?y))" => "(+ (* (cosh ?x) (cosh ?y)) (* (sinh ?x) (sinh ?y)))"),
        rw!("cosh(a-b)->cosh(a)cosh(b)-sinh(a)sinh(b)";
            "(cosh (- ?x ?y))" => "(- (* (cosh ?x) (cosh ?y)) (* (sinh ?x) (sinh ?y)))"),
        rw!("tanh(a+b)->((tanh(a)+tanh(b))/(1+tanh(a)tanh(b)))";
            "(tanh (+ ?x ?y))" => "(/ (+ (tanh ?x) (tanh ?y)) (+ 1 (* (tan ?x) (tan ?y))))"),
        rw!("tanh(a-b)->((tanh(a)-tanh(b))/(1-tanh(a)tanh(b)))";
            "(tanh (- ?x ?y))" => "(/ (- (tanh ?x) (tanh ?y)) (- 1 (* (tanh ?x) (tanh ?y))))"),

        /* =========================== derivative =========================== */
        /* basic derivative */
        rw!("d/dx c"; "(d ?x ?c)" => "0" if is_const("?c")),
        /* generalized chain rule */
        rw!("d/dx f(x)^c"; "(d ?x (pow ?f ?c))" => "(* (* ?c (pow ?f (- ?c 1))) (d ?x ?f))" if is_const("?c")),
        /* distributive property */
        rw!("d/dx c*f(x)"; "(d ?x (* ?c ?f))" => "(* ?c (d ?x ?f))" if is_const("?c")),
        rw!("d/dx f(x)+g(x)"; "(d ?x (+ ?y ?g))" => "(+ (d ?x ?f) (d ?x ?g))"),
        rw!("d/dx f(x)-g(x)"; "(d ?x (- ?f ?g))" => "(- (d ?x ?f) (d ?x ?g))"),
        /* polynomial */
        rw!("d/d?x ?x^c"; "(d ?x (pow ?x ?c))" => "(* ?c (pow ?x (- ?c 1)))" if is_const("?c")),
        /* generalized trig (chain rule) */
        rw!("d/dx sin(u)"; "(d ?x (sin ?u))" => "(* (cos ?u) (d ?x ?u))"),
        rw!("d/dx cos(u)"; "(d ?x (cos ?u))" => "(* (* -1 (sin ?u)) (d ?x ?u))"),
        rw!("d/dx tan(u)"; "(d ?x (tan ?u))" => "(* (pow (sec ?u) 2) (d ?x ?u))"),
        rw!("d/dx csc(u)"; "(d ?x (csc ?u))" => "(* (* -1 (* (csc ?u) (cot ?u))) (d ?x ?u))"),
        rw!("d/dx sec(u)"; "(d ?x (sec ?u))" => "(* (* (sec ?u) (tan ?u)) (d ?x ?u))"),
        rw!("d/dx cot(u)"; "(d ?x (cot ?u))" => "(* (* -1 (pow (csc ?x) 2)) (d ?x ?u))"),
        /* generalized inv trig (chain rule) */
        rw!("d/dx asin(u)"; "(d ?x (asin ?u))" => "(* (/ 1 (sqrt (- 1 (pow ?u 2)))) (d ?x ?u))"),
        rw!("d/dx acos(u)"; "(d ?x (acos ?u))" => "(* (/ -1 (sqrt (- 1 (pow ?u 2)))) (d ?x ?u))"),
        rw!("d/dx atan(u)"; "(d ?x (atan ?u))" => "(* (/ 1 (+ 1 (pow ?u 2))) (d ?x ?u))"),
        rw!("d/dx acsc(u)"; "(d ?x (acsc ?u))" => "(* (/ -1 (* (abs ?u) (sqrt (- 1 (pow ?u 2))))) (d ?x ?u))"),
        rw!("d/dx asec(u)"; "(d ?x (asec ?u))" => "(* (/ 1 (* (abs ?u) (sqrt (- 1 (pow ?u 2))))) (d ?x ?u))"),
        rw!("d/dx acot(u)"; "(d ?x (acot ?u))" => "(* (/ -1 (+ 1 (pow ?u 2))) (d ?x ?u))"),
        /* generalized hyperbolic (chain rule) */
        rw!("d/dx sinh(u)"; "(d ?x (sinh ?u))" => "(* (cosh ?u) (d ?x ?u))"),
        rw!("d/dx cosh(u)"; "(d ?x (cosh ?u))" => "(*  (sinh ?u) (d ?x ?u))"),
        rw!("d/dx tanh(u)"; "(d ?x (tanh ?u))" => "(* (pow (sech ?u) 2) (d ?x ?u))"),
        rw!("d/dx csch(u)"; "(d ?x (csch ?u))" => "(* (* -1 (* (csch ?u) (coth ?u))) (d ?x ?u))"),
        rw!("d/dx sech(u)"; "(d ?x (sech ?u))" => "(* (* -1 (* (sech ?u) (tanh ?u))) (d ?x ?u))"),
        rw!("d/dx coth(u)"; "(d ?x (coth ?u))" => "(* (* -1 (pow (csch ?u) 2)) (d ?x ?u))"),
        /* generalized inv hyperbolic (chain rule) */
        rw!("d/dx asinh(u)"; "(d ?x (asinh ?u))" => "(* (/ 1 (sqrt (+ (pow ?u 2) 1))) (d ?x ?u))"),
        rw!("d/dx acosh(u)"; "(d ?x (acosh ?u))" => "(* (/ 1 (sqrt (- (pow ?u 2) 1))) (d ?x ?u))"),
        rw!("d/dx atanh(u)"; "(d ?x (atanh ?u))" => "(* (/ 1 (- 1 (pow ?u 2))) (d ?x ?u))"),
        rw!("d/dx acsch(u)"; "(d ?x (acsch ?u))" => "(* (/ -1 (* (abs ?u) (sqrt (+ 1 (pow ?u 2))))) (d ?x ?u))"),
        rw!("d/dx asech(u)"; "(d ?x (asech ?u))" => "(* (/ -1 (* (abs ?u) (sqrt (- 1 (pow ?u 2))))) (d ?x ?u))"),
        rw!("d/dx acoth(u)"; "(d ?x (acoth ?u))" => "(* (/ 1 (- 1 (pow ?u 2))) (d ?x ?u))"),
        /* ================================================================== */
    ]
}