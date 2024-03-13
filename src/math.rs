use crate::{rewrite as rw, *};
use ordered_float::NotNan;

/// mathematical expression egraph
pub type MathEGraph = EGraph<Math, ConstantFold>;
pub type Rewrite = crate::Rewrite<Math, ConstantFold>;

pub type Constant = NotNan<f64>;

/* math operators */
define_language! {
    #[allow(missing_docs)]
    /// enum representing mathematical operations
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
        // "exp"=Exp(Id),
        "ln"=Ln(Id),
        "log"=Log([Id;2]),

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

    fn make(egraph: &mut MathEGraph, enode: &Math) -> Self::Data {
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
#[rustfmt::skip]
pub fn math_rule() -> Vec<Rewrite> {
    vec![
        /* ==================== basic arithmetic simplification ===================== */
        rw!("x+0=x"; "(+ ?x 0)" => "?x"),
        rw!("x*0=0"; "(* 0 ?x)" => "0"),
        rw!("x*1=x"; "(* ?x 1)" => "?x"),
        rw!("x/1=x"; "(/ ?x 1)" => "?x"),
        rw!("x-x=0"; "(- ?x ?x)" => "0"),
        rw!("x/x=1"; "(/ ?x ?x)" => "1" if not_zero("?x")),
        rw!("(-1)*(-1)=1"; "(* -1 -1)" => "1"),
        rw!("x*(1/x)=1"; "(* ?x (/ 1 ?x))" => "1" if not_zero("?x")),
        rw!("x+(-1*y)=x-y"; "(+ ?x (* -1 ?y))" => "(- ?x ?y)"),
        // rw!("x-y->x+(-1*y)"; "(- ?x ?y)" => "(+ ?x (* -1 ?y))"),
        rw!("-(x+y)=-x-y"; "(* -1 (- ?x ?y))" => "(- (* -1 ?x) ?y)"),
        rw!("-x-y=-(x+y)"; "(- (* -1 ?x) ?y)" => "(* -1 (+ ?x ?y))"),
        rw!("-(x-y)=y-x"; "(* -1 (- ?x ?y))" => "(- ?y ?x)"),
        rw!("y-x=-(x-y)"; "(- ?y ?x)" => "(* -1 (- ?x ?y))"),
        /* ========================================================================== */

        /* =============================== expansion ================================ */
        rw!("x=1*x"; "?x" => "(* 1 ?x)"),
        rw!("x=x^1"; "?x" => "(pow ?x 1)"),
        /* ========================================================================== */

        /* ============================== commutative =============================== */
        /* addition */
        rw!("x+y=y+x"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        /* multiplication */
        rw!("xy=yx"; "(* ?x ?y)" => "(* ?y ?x)"),
        /* ========================================================================== */

        /* ================= distributive property & factorization ================== */
        rw!("ax+bx=(a+b)x"; "(+ (* ?a ?x) (* ?b ?x))" => "(* (+ ?a ?b) ?x)"),
        // rw!("(a+b)x=ax+bx"; "(* (+ ?a ?b) ?x)" => "(+ (* ?a ?x) (* ?b ?x))"),
        rw!("ax-bx=(a-b)x"; "(- (* ?a ?x) (* ?b ?x))" => "(* (- ?a ?b) ?x)"),
        // rw!("(a-b)x=ax-bx"; "(* (- ?a ?b) ?x)" => "(- (* ?a ?x) (* ?b ?x))"),
        rw!("(a+b)(c+d)=ac+ad+bc+bd";
            "(* (+ ?a ?b) (+ ?c ?d))" => "(+ (+ (+ (* ?a ?c) (* ?a ?d)) (* ?b ?c)) (* ?b ?d))"),
        /* ========================================================================== */

        /* =========================== order of operation =========================== */
        /* multiplication & division */
        rw!("(xy)z=x(yz)"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z))"),
        rw!("x(yz)=(xy)z"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),
        rw!("(xy)/z=x(y/z)"; "(/ (* ?x ?y) ?z)" => "(* ?x (/ ?y ?z))"),
        rw!("x(y/z)=(xy)/z"; "(* ?x (/ ?y ?z))" => "(/ (* ?x ?y) ?z)"),
        rw!("(x/y)z=(xz)/y"; "(* (/ ?x ?y) ?z)" => "(/ (* ?x ?z) ?y)"),
        rw!("(xz)/y=(x/y)z"; "(/ (* ?x ?z) ?y)" => "(* (/ ?x ?y) ?z)"),
        rw!("(x/y)z=x(z/y)"; "(* (/ ?x ?y) ?z)" => "(* ?x (/ ?z ?y))"),
        rw!("x(z/y)=(x/y)z"; "(* ?x (/ ?z ?y))" => "(* (/ ?x ?y) ?z)"),
        rw!("(x/y)/z=xz/y"; "(/ (/ ?x ?y) ?z)" => "(/ (* ?x ?z) ?y)"),
        rw!("xz/y=(x/y)/z"; "(/ (* ?x ?z) ?y)" => "(/ (/ ?x ?y) ?z)"),
        /* addition & subtraction */
        rw!("(x+y)+z=x+(y+z)"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z))"),
        rw!("x+(y+z)=(x+y)+z"; "(+ ?x (+ ?y ?z))" => "(+ (+ ?x ?y) ?z)"),
        rw!("(x+y)-z=x+(y-z)"; "(- (+ ?x ?y) ?z)" => "(+ ?x (- ?y ?z))"),
        rw!("x+(y-z)=(x+y)-z"; "(+ ?x (- ?y ?z))" => "(- (+ ?x ?y) ?z)"),
        rw!("(x-y)+z=x-(y-z)"; "(+ (- ?x ?y) ?z)" => "(- ?x (- ?y ?z))"),
        rw!("x-(y-z)=(x-y)+z"; "(- ?x (- ?y ?z))" => "(+ (- ?x ?y) ?z)"),
        rw!("(x-y)-z=x-(y+z)"; "(- (- ?x ?y) ?z)" => "(- ?x (+ ?y ?z))"),
        rw!("x-(y+z)=(x-y)-z"; "(- ?x (+ ?y ?z))" => "(- (- ?x ?y) ?z)"),
        /* ========================================================================== */

        /* ============================ binomial theorem ============================ */
        rw!("(x+y)^2=(x)^2+2xy+(y)^2";
            "(pow (+ ?x ?y) 2)" => "(+ (+ (pow ?x 2) (* 2 (* ?x ?y))) (pow ?y 2))"),
        rw!("(x)^2+2xy+(y)^2=(x+y)^2";
            "(+ (+ (pow ?x 2) (* 2 (* ?x ?y))) (pow ?y 2))" => "(pow (+ ?x ?y) 2)"),
        /* ========================================================================== */

        /* ============================== power rules =============================== */
        /* simplification */
        rw!("pow(0)"; "(pow ?x 0)" => "1"),
        rw!("pow(1)"; "(pow ?x 1)" => "?x"),
        /* basic identities */
        rw!("pow-of-prod"; "(* (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (+ ?y ?z))"),
        rw!("pow-of-quotient"; "(/ (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (- ?y ?z))"),
        rw!("pow-of-pow"; "(pow (pow ?x ?y) ?z)" => "(pow ?x (* ?y ?z))"),
        rw!("pow-of-(-1)"; "(pow ?x -1)" => "(/ 1 ?x)" if not_zero("?x")),
        /* ========================================================================== */

        /* ============================= exponent rules ============================= */
        // rw!("exp(0)"; "(exp 0)" => "1"),
        // rw!("exp(1)"; "(exp 1)" => "E"),
        /* basic rule */
        // rw!("exp-of-prod"; "(* (exp ?x) (exp ?y))" => "(exp (+ ?x ?y))"),
        // rw!("exp-of-quotient"; "(/ (exp ?x) (exp ?y))" => "(exp (- ?x ?y))"),
        // rw!("pow-of-exp"; "(pow (exp ?x) ?y)" => "(exp (* ?x ?y))"),
        /* ========================================================================== */

        /* =============================== logarithm ================================ */
        /* ln */
        rw!("ln(e)"; "(ln e)" => "1"),
        rw!("ln(ab)->ln(a)+ln(b)"; "(ln (* ?a ?b))" => "(+ (ln ?a) (ln ?b))"),
        rw!("ln(a)+ln(b)->ln(ab)"; "(+ (ln ?a) (ln ?b))" => "(ln (* ?a ?b))"),
        rw!("ln(a/b)->ln(a)-ln(b)"; "(ln (/ ?a ?b))" => "(- (ln ?a) (ln ?b))"),
        rw!("ln(a)-ln(b)->ln(a/b)"; "(- (ln ?a) (ln ?b))" => "(ln (/ ?a ?b))"),
        rw!("ln(x^a)->aln(x)"; "(ln (pow ?x ?a))" => "(* ?a (ln ?x))"),
        rw!("aln(x)->ln(x^a)"; "(* ?a (ln ?x))" => "(ln (pow ?x ?a))"),
        /* log */
        rw!("log(b)->1"; "(log ?b ?b)" => "1"),
        rw!("log(xy)->log(x)+log(y)"; "(log ?b (* ?x ?y))" => "(+ (log ?b ?x) (log ?b ?y))"),
        rw!("log(x)+log(y)->log(xy)"; "(+ (log ?b ?x) (log ?b ?y))" => "(log ?b (* ?x ?y))"),
        rw!("log(x/y)->log(x)-log(y)"; "(log ?b (/ ?x ?y))" => "(- (log ?b ?x) (log ?b ?y))"),
        rw!("log(x)-log(y)->log(x/y)"; "(- (log ?b ?x) (log ?b ?y))" => "(log ?b (/ ?x ?y))"),
        rw!("log(x^a)->alog(x)"; "(log ?b (pow ?x ?a))" => "(* ?a (log ?b ?x))"),
        rw!("alog(x)->log(x^a)"; "(* ?a (log ?b ?x))" => "(log ?b (pow ?x ?a))"),
        /* ========================================================================== */

        /* ================================= trig =================================== */
        /* basic identities */
        rw!("tan->sin/cos"; "(tan ?x)" => "(/ (sin ?x) (cos ?x))"),
        rw!("cos->sin/tan"; "(cos ?x)" => "(/ (sin ?x) (tan ?x))"),
        rw!("sin->cos*tan"; "(sin ?x)" => "(* (cos ?x) (tan ?x))"),
        /* reciprocal identities <-> */
        rw!("1/sin->csc"; "(/ 1 (sin ?x))" => "(csc ?x)"),
        rw!("1/cos->sec"; "(/ 1 (cos ?x))" => "(sec ?x)"),
        rw!("1/tan->cot"; "(/ 1 (tan ?x))" => "(cot ?x)"),
        rw!("csc->1/sin"; "(csc ?x)" => "(/ 1 (sin ?x))"),
        rw!("sec->1/cos"; "(sec ?x)" => "(/ 1 (cos ?x))"),
        rw!("cot->1/tan"; "(cot ?x)" => "(/ 1 (tan ?x))"),
        /*
        don't need these rw because trig basic identity and trig reciprocal identity cover it
        ```
        rw!("sin/cos->tan"; "(/ (sin ?x) (cos ?x))" => "(tan ?x)"),
        rw!("sin/tan->cos"; "(/ (sin ?x) (tan ?x))" => "(cos ?x)"),
        rw!("cos*tan->sin"; "(* (cos ?x) (tan ?x))" => "(sin ?x)"),
        rw!("cot->cos/sin"; "(cot ?x)" => "(/ (cos ?x) (sin ?x))"),
        rw!("cos/sin->cot"; "(/ (cos ?x) (sin ?x))" => "(cot ?x)"),
        ```
         */
        /*
        don't need these rw because trig reciprocal identity & multiplication <-> division identity
        ```
        rw!("sin->1/csc"; "(sin ?x)" => "(/ 1 (csc ?x))"),
        rw!("cos->1/sec"; "(cos ?x)" => "(/ 1 (sec ?x))"),
        rw!("tan->1/cot"; "(tan ?x)" => "(/ 1 (cot ?x))"),
        rw!("1/csc->sin"; "(/ 1 (csc ?x))" => "(sin ?x)"),
        rw!("1/sec->cos"; "(/ 1 (sec ?x))" => "(cos ?x)"),
        rw!("1/cot->tan"; "(/ 1 (cot ?x))" => "(tan ?x)"),
        ```
         */
        /* Pythagorean identities */
        rw!("sin^2+cos^2->1"; "(+ (pow (sin ?x) 2) (pow (cos ?x) 2))" => "1"),
        // rw!("1->sin^2+cos^2"; "1" => "(+ (pow (sin ?x) 2) (pow (cos ?x) 2))"),
        rw!("sin->sqrt(1-cos^2)"; "(sin ?x)" => "(sqrt (- 1 (pow (cos ?x) 2)))"),
        rw!("sqrt(1-cos^2)->sin"; "(sqrt (- 1 (pow (cos ?x) 2)))" => "(sin ?x)"),
        rw!("sin->-sqrt(1-cos^2)"; "(sin ?x)" => "(* -1 (sqrt (- 1 (pow (cos ?x) 2))))"),
        rw!("-sqrt(1-cos^2)->sin"; "(* -1 (sqrt (- 1 (pow (cos ?x) 2))))" => "(sin ?x)"),
        rw!("cos->sqrt(1-sin^2)"; "(cos ?x)" => "(sqrt (- 1 (pow (sin ?x) 2)))"),
        rw!("sqrt(1-sin^2)->cos"; "(sqrt (- 1 (pow (sin ?x) 2)))" => "(cos ?x)"),
        rw!("cos->-sqrt(1-sin^2)"; "(cos ?x)" => "(* -1 (sqrt (- 1 (pow (sin ?x) 2))))"),
        rw!("-sqrt(1-cos^2)->sin"; "(* -1 (sqrt (- 1 (pow (cos ?x) 2))))" => "(sin ?x)"),
        rw!("tan^2+1->sec^2"; "(+ (pow (tan ?x) 2) 1)" => "(pow (sec ?x) 2)"),
        rw!("sec^2->tan^2+1->"; "(pow (sec ?x) 2)" => "(+ (pow (tan ?x) 2) 1)"),
        rw!("cot^2+1->csc^2"; "(+ (pow (cot ?x) 2) 1)" => "(pow (csc ?x) 2)"),
        rw!("csc^2->cot^2+1"; "(pow (csc ?x) 2)" => "(+ (pow (cot ?x) 2) 1)"),
        rw!("sec^2+csc^2->sec^2csc^2";
            "(+ (pow (sec ?x) 2) (pow (csc ?x) 2))" => "(* (pow (sec ?x) 2) (pow (csc ?x) 2))"),
        rw!("sec^2csc^2->sec^2+csc^2";
            "(* (pow (sec ?x) 2) (pow (csc ?x) 2))" => "(+ (pow (sec ?x) 2) (pow (csc ?x) 2))"),
        /* even-odd identities */
        rw!("sin(-x)=-sin(x)"; "(sin (* -1 ?x))" => "(* -1 (sin ?x))"),
        rw!("-sin(x)=sin(-x)"; "(* -1 (sin ?x))" => "(sin (* -1 ?x))"),
        rw!("cos(-x)=cos(x)"; "(cos (* -1 ?x))" => "(cos ?x)"),
        rw!("cos(x)->cos(-x)"; "(cos ?x)" => "(cos (* -1 ?x))"),
        rw!("tan(-x)=-tan(x)"; "(tan (* -1 ?x))" => "(* -1 (tan ?x))"),
        rw!("-tan(x)->tan(-x)"; "(* -1 (tan ?x))" => "(tan (* -1 ?x))"),
        rw!("csc(-x)=-csc(x)"; "(csc (* -1 ?x))" => "(* -1 (csc ?x))"),
        rw!("-csc(x)->csc(-x)"; "(* -1 (csc ?x))" => "(csc (* -1 ?x))"),
        rw!("sec(-x)=sec(x)"; "(sec (* -1 ?x))" => "(sec ?x)"),
        rw!("sec(x)->sec(-x)"; "(sec ?x)" => "(sec (* -1 ?x))"),
        rw!("cot(-x)=-cot(x)"; "(cot (* -1 ?x))" => "(* -1 (cot ?x))"),
        rw!("-cot(x)->cot(-x)"; "(* -1 (cot ?x))" => "(cot (* -1 ?x))"),
        /* shifts & periodicity */
        /* 1/4 period */
        rw!("sin(x+0.5pi)=cos(x)"; "(sin (+ ?x (* 0.5 pi)))" => "(cos ?x)"),
        rw!("cos(x)=sin(x+0.5pi)"; "(cos ?x)" => "(sin (+ ?x (* 0.5 pi)))"),
        rw!("sin(x-0.5pi)=-cos(x)"; "(sin (- ?x (* 0.5 pi)))" => "(* -1 (cos ?x))"),
        rw!("-cos(x)=sin(x-0.5pi)"; "(* -1 (cos ?x))" => "(sin (- ?x (* 0.5 pi)))"),
        rw!("cos(x+0.5pi)=-sin(x)"; "(cos (+ ?x (* 0.5 pi)))" => "(* -1 (sin ?x))"),
        rw!("-sin(x)=cos(x+0.5pi)"; "(* -1 (sin ?x))" => "(cos (+ ?x (* 0.5 pi)))"),
        rw!("cos(x-0.5pi)=sin(x)"; "(cos (- ?x (* 0.5 pi)))" => "(sin ?x)"),
        rw!("sin(x)=cos(x-0.5pi)"; "(sin ?x)" => "(cos (- ?x (* 0.5 pi)))"),
        rw!("csc(x+0.5pi)=sec(x)"; "(csc (+ ?x (* 0.5 pi)))" => "(sec ?x)"),
        rw!("sec(x)=csc(x+0.5pi)"; "(sec ?x)" => "(csc (+ ?x (* 0.5 pi)))"),
        rw!("csc(x-0.5pi)=-sec(x)"; "(csc (- ?x (* 0.5 pi)))" => "(* -1 (sec ?x))"),
        rw!("-sec(x)=csc(x-0.5pi)"; "(* -1 (sec ?x))" => "(csc (- ?x (* 0.5 pi)))"),
        rw!("sec(x+0.5pi)=-csc(x)"; "(sec (+ ?x (* 0.5 pi)))" => "(* -1 (csc ?x))"),
        rw!("-csc(x)=sec(x+0.5pi)"; "(* -1 (csc ?x))" => "(sec (+ ?x (* 0.5 pi)))"),
        rw!("sec(x-0.5pi)=csc(x)"; "(sec (- ?x (* 0.5 pi)))" => "(csc ?x)"),
        rw!("csc(x)=sec(x-0.5pi)"; "(csc ?x)" => "(sec (- ?x (* 0.5 pi)))"),
        rw!("tan(x+0.25pi)=(tan+1)/(1-tan)";
            "(tan (+ ?x (* 0.25 pi)))" => "(/ (+ (tan ?x) 1) (- 1 (tan ?x)))"),
        rw!("(tan+1)/(1-tan)=tan(x+0.25pi)";
            "(/ (+ (tan ?x) 1) (- 1 (tan ?x)))" => "(tan (+ ?x (* 0.25 pi)))"),
        rw!("tan(x-0.25pi)=(tan-1)/(1+tan)";
            "(tan (- ?x (* 0.25 pi)))" => "(/ (- (tan ?x) 1) (+ 1 (tan ?x)))"),
        rw!("(tan-1)/(1+tan)=tan(x-0.25pi)";
            "(/ (- (tan ?x) 1) (+ 1 (tan ?x)))" => "(tan (- ?x (* 0.25 pi)))"),
        rw!("cot(x+0.25pi)=(cot-1)/(1+cot)";
            "(cot (+ ?x (* 0.25 pi)))" => "(/ (- (cot ?x) 1) (+ 1 (cot ?x)))"),
        rw!("(cot-1)/(1+cot)=cot(x+0.25pi)";
            "(/ (- (cot ?x) 1) (+ 1 (cot ?x)))" => "(cot (+ ?x (* 0.25 pi)))"),
        rw!("cot(x-0.25pi)=(cot+1)/(1-cot)";
            "(cot (- ?x (* 0.25 pi)))" => "(/ (+ (cot ?x) 1) (- 1 (cot ?x)))"),
        rw!("(cot+1)/(1-cot)=cot(x-0.25pi)";
            "(/ (+ (cot ?x) 1) (- 1 (cot ?x)))" => "(cot (- ?x (* 0.25 pi)))"),
        /* 1/2 period */
        rw!("sin(x+pi)=-sin(x)"; "(sin (+ ?x pi))" => "(* -1 (sin ?x))"),
        rw!("-sin(x)=sin(x+pi)"; "(* -1 (sin ?x))" => "(sin (+ ?x pi))"),
        rw!("cos(x+pi)=-cos(x)"; "(cos (+ ?x pi))" => "(* -1 (cos ?x))"),
        rw!("-cos(x)=cos(x+pi)"; "(* -1 (cos ?x))" => "(cos (+ ?x pi))"),
        rw!("csc(x+pi)=-csc(x)"; "(csc (+ ?x pi))" => "(* -1 (csc ?x))"),
        rw!("-csc(x)=csc(x+pi)"; "(* -1 (csc ?x))" => "(csc (+ ?x pi))"),
        rw!("sec(x+pi)=-sec(x)"; "(sec (+ ?x pi))" => "(* -1 (sec ?x))"),
        rw!("-sec(x)=sec(x+pi)"; "(* -1 (sec ?x))" => "(sec (+ ?x pi))"),
        rw!("tan(x+0.5pi)=-cot(x)"; "(tan (+ ?x (* 0.5 pi)))" => "(* -1 (cot ?x))"),
        rw!("-cot(x)=tan(x+0.5pi)"; "(* -1 (cot ?x))" => "(tan (+ ?x (* 0.5 pi)))"),
        rw!("cot(x+0.5pi)=-tan(x)"; "(cot (+ ?x (* 0.5 pi)))" => "(* -1 (tan ?x))"),
        rw!("-tan(x)=cot(x+0.5pi)"; "(* -1 (tan ?x))" => "(cot (+ ?x (* 0.5 pi)))"),
        /* 1 period */
        rw!("sin(x+2pi)=sin(x)"; "(sin (+ ?x (* 2 pi)))" => "(sin ?x)"),
        rw!("sin(x)=sin(x+2pi)"; "(sin ?x)" => "(sin (+ ?x (* 2 pi)))"),
        rw!("cos(x+2pi)=cos(x)"; "(cos (+ ?x (* 2 pi)))" => "(cos ?x)"),
        rw!("cos(x)=cos(x+2pi)"; "(cos ?x)" => "(cos (+ ?x (* 2 pi)))"),
        rw!("csc(x+2pi)=csc(x)"; "(csc (+ ?x (* 2 pi)))" => "(csc ?x)"),
        rw!("csc(x)=csc(x+2pi)"; "(csc ?x)" => "(csc (+ ?x (* 2 pi)))"),
        rw!("sec(x+2pi)=sec(x)"; "(sec (+ ?x (* 2 pi)))" => "(sec ?x)"),
        rw!("sec(x)=sec(x+2pi)"; "(sec ?x)" => "(sec (+ ?x (* 2 pi)))"),
        rw!("tan(x+pi)=tan(x)"; "(tan (+ ?x pi))" => "(tan ?x)"),
        rw!("tan(x)=tan(x+pi)"; "(tan ?x)" => "(tan (+ ?x pi))"),
        rw!("cot(x+pi)=cot(x)"; "(cot (+ ?x pi))" => "(cot ?x)"),
        rw!("cot(x)=cot(x+pi)"; "(cot ?x)" => "(cot (+ ?x pi))"),
        /* reflections */
        rw!("sin(1.5pi-x)=-cos(x)"; "(sin (- (* 1.5 pi) ?x))" => "(* -1 (cos ?x))"),
        rw!("-cos(x)=sin(1.5pi-x)"; "(* -1 (cos ?x))" => "(sin (- (* 1.5 pi) ?x))"),
        rw!("cos(1.5pi-x)=-sin(x)"; "(cos (- (* 1.5 pi) ?x))" => "(* -1 (sin ?x))"),
        rw!("-sin(x)=cos(1.5pi-x)"; "(* -1 (sin ?x))" => "(cos (- (* 1.5 pi) ?x))"),
        rw!("tan(1.5pi-x)=cot(x)"; "(tan (- (* 1.5 pi) ?x))" => "(cot ?x)"),
        rw!("cot(x)=tan(1.5pi-x)"; "(cot ?x)" => "(tan (- (* 1.5 pi) ?x))"),
        rw!("csc(1.5pi-x)=-sec(x)"; "(csc (- (* 1.5 pi) ?x))" => "(* -1 (sec ?x))"),
        rw!("-sec(x)=csc(1.5pi-x)"; "(* -1 (sec ?x))" => "(csc (- (* 1.5 pi) ?x))"),
        rw!("sec(1.5pi-x)=-csc(x)"; "(sec (- (* 1.5 pi) ?x))" => "(* -1 (csc ?x))"),
        rw!("-csc(x)=sec(1.5pi-x)"; "(* -1 (csc ?x))" => "(sec (- (* 1.5 pi) ?x))"),
        rw!("cot(1.5pi-x)=tan(x)"; "(cot (- (* 1.5 pi) ?x))" => "(tan ?x)"),
        rw!("tan(x)=cot(1.5pi-x)"; "(tan ?x)" => "(cot (- (* 1.5 pi) ?x))"),
        /* sum and difference identities */
        rw!("sin(a+b)=sin(a)cos(b)+cos(a)sin(b)";
            "(sin (+ ?a ?b))" => "(+ (* (sin ?a) (cos ?b)) (* (cos ?a) (sin ?b)))"),
        rw!("sin(a)cos(b)+cos(a)sin(b)=sin(a+b)";
            "(+ (* (sin ?a) (cos ?b)) (* (cos ?a) (sin ?b)))" => "(sin (+ ?a ?b))"),
        rw!("sin(a-b)=sin(a)cos(b)-cos(a)sin(b)";
            "(sin (- ?a ?b))" => "(- (* (sin ?a) (cos ?b)) (* (cos ?a) (sin ?b)))"),
        rw!("sin(a)cos(b)-cos(a)sin(b)=sin(a-b)";
            "(- (* (sin ?a) (cos ?b)) (* (cos ?a) (sin ?b)))" => "(sin (- ?a ?b))"),
        rw!("cos(a+b)=cos(a)cos(b)-sin(a)sin(b)";
            "(cos (+ ?a ?b))" => "(- (* (cos ?a) (cos ?b)) (* (sin ?a) (sin ?b)))"),
        rw!("cos(a)cos(b)-sin(a)sin(b)=cos(a+b)";
            "(- (* (cos ?a) (cos ?b)) (* (sin ?a) (sin ?b)))" => "(cos (+ ?a ?b))"),
        rw!("cos(a-b)=cos(a)cos(b)+sin(a)sin(b)";
            "(cos (- ?a ?b))" => "(+ (* (cos ?a) (cos ?b)) (* (sin ?a) (sin ?b)))"),
        rw!("cos(a)cos(b)+sin(a)sin(b)=cos(a-b)";
            "(+ (* (cos ?a) (cos ?b)) (* (sin ?a) (sin ?b)))" => "(cos (- ?a ?b))"),
        rw!("tan(a+b)=(tan(a)+tan(b))/(1-tan(a)tan(b))";
            "(tan (+ ?a ?b))" => "(/ (+ (tan ?a) (tan ?b)) (- 1 (* (tan ?a) (tan ?b))))"),
        rw!("(tan(a)+tan(b))/(1-tan(a)tan(b))=tan(a+b)";
            "(/ (+ (tan ?a) (tan ?b)) (- 1 (* (tan ?a) (tan ?b))))" => "(tan (+ ?a ?b))"),
        rw!("tan(a-b)=(tan(a)-tan(b))/(1+tan(a)tan(b))";
            "(tan (- ?a ?b))" => "(/ (- (tan ?a) (tan ?b)) (+ 1 (* (tan ?a) (tan ?b))))"),
        rw!("(tan(a)-tan(b))/(1+tan(a)tan(b))=tan(a-b)";
            "(/ (- (tan ?a) (tan ?b)) (+ 1 (* (tan ?a) (tan ?b))))" => "(tan (- ?a ?b))"),
        // csc & sec rw exceed length limit
        rw!("cot(a+b)=(cot(a)cot(b)-1)/(cot(b)+cot(a))";
            "(cot (+ ?a ?b))" => "(/ (- (* (cot ?a) (cot ?b)) 1) (+ (cot ?b) (cot ?a)))"),
        rw!("cot(a-b)=(cot(a)cot(b)+1)/(cot(b)1cot(a))";
            "(cot (- ?a ?b))" => "(/ (+ (* (cot ?a) (cot ?b)) 1) (- (cot ?b) (cot ?a)))"),
        /* double-angle formulae */
        rw!("sin(x)=2sin(x/2)cos(x/2)"; "(sin ?x)" => "(* 2 (* (sin (/ ?x 2)) (cos (/ ?x 2))))"),
        rw!("2sin(x)cos(x)=sin(2x)"; "(* 2 (* (sin ?x) (cos ?x)))" => "(sin (* 2 ?x))"),
        rw!("sin(x)=(sin(x/2)+cos(x/2))^2-1";
            "(sin ?x)" => "(- (pow (+ (sin (/ ?x 2)) (cos (/ ?x 2))) 2) 1)"),
        rw!("(sin(x)+cos(x))^2-1=sin(2x)";
            "(- (pow (+ (sin ?x) (cos ?x)) 2) 1)" => "(sin (* 2 ?x))"),
        rw!("sin(x)=2tan(0.5x)/(1+tan^2(0.5x)";
            "(sin ?x)" => "(/ (* 2 (tan (/ ?x 2))) (+ 1 (pow (tan (/ ?x 2)) 2)))"),
        rw!("2tan(x)/(1+tan^2(x)=sin(2x)";
            "(/ (* 2 (tan ?x)) (+ 1 (pow (tan ?x) 2)))" => "(sin (* 2 ?x))"),
        rw!("cos(x)=cos^2(x/2)-sin^2(x/2)";
            "(cos ?x)" => "(- (pow (cos (/ ?x 2)) 2) (pow (sin (/ ?x 2)) 2))"),
        rw!("cos^2(x)-sin^2(x)=cos(2x)";
            "(- (pow (cos ?x) 2) (pow (sin ?x) 2))" => "(cos (* 2 ?x))"),
        rw!("cos(x)=2cos^2(x/2)-1"; "(cos ?x)" => "(- (* 2 (pow (cos (/ ?x 2)) 2)) 1)"),
        rw!("2cos^2(x)-1=cos(2x)"; "(- (* 2 (pow (cos ?x) 2)) 1)" => "(cos (* 2 ?x))"),
        rw!("cos(x)=1-2sin^2(x/2)"; "(cos ?x)" => "(- 1 (* 2 (pow (sin (/ ?x 2)) 2)))"),
        rw!("1-2sin^2(x)=cos(2x)"; "(- 1 (* 2 (pow (sin ?x) 2)))" => "(cos (* 2 ?x))"),
        rw!("cos(x)=(1-tan^2(x/2))/(1+tan^2(x/2))";
            "(cos ?x)" => "(/ (- 1 (pow (tan (/ ?x 2)) 2)) (+ 1 (pow (tan (/ ?x 2)) 2)))"),
        rw!("(1-tan^2(x))/(1+tan^2(x))=cos(2x)";
            "(/ (- 1 (pow (tan ?x) 2)) (+ 1 (pow (tan ?x) 2)))" => "(cos (* 2 ?x))"),
        rw!("tan(x)=2tan(x/2)/(1-tan^2(x/2))";
            "(tan ?x)" => "(/ (* 2 (tan (/ ?x 2))) (- 1 (pow (tan (/ ?x 2)) 2)))"),
        rw!("2tan(x)/(1-tan^2(x))=tan(2x)";
            "(/ (* 2 (tan ?x)) (- 1 (pow (tan ?x) 2)))" => "(tan (* 2 ?x))"),
        rw!("csc(x)=(sec(x/2)csc(x/2))/2"; "(csc ?x)" => "(/ (* (sec (/ ?x 2)) (csc (/ ?x 2))) 2)"),
        rw!("csc(2x)=(sec(x)csc(x))/2"; "(/ (* (sec ?x) (csc ?x)) 2)" => "(csc (* 2 ?x))"),
        rw!("csc(x)=(1+tan^2(x/2))/(2tan(x/2))";
            "(csc ?x)" => "(/ (+ 1 (pow (tan (/ ?x 2)) 2)) (* 2 (tan (/ ?x 2))))"),
        rw!("(1+tan^2(x))/(2tan(x))=csc(2x)";
            "(/ (+ 1 (pow (tan ?x) 2)) (* 2 (tan ?x)))" => "(csc (* 2 ?x))"),
        rw!("sec(x)=sec^2(x/2)/(2-sec^2(x/2))";
            "(sec ?x)" => "(/ (pow (sec (/ ?x 2)) 2) (- 2 (pow (sec (/ ?x 2)) 2)))"),
        rw!("sec^2(x)/(2-sec^2(x))=sec(2x)";
            "(/ (pow (sec ?x) 2) (- 2 (pow (sec ?x) 2)))" => "(sec (* 2 ?x))"),
        rw!("sec(x)=(1+tan^2(x/2))/(1-tan^2(x/2))";
            "(sec ?x)" => "(/ (+ 1 (pow (tan (/ ?x 2)) 2)) (- 1 (pow (tan (/ ?x 2)) 2)))"),
        rw!("(1+tan^2(x))/(1-tan^2(x))=sec(2x)";
            "(/ (+ 1 (pow (tan ?x) 2)) (- 1 (pow (tan ?x) 2)))" => "(sec (* 2 ?x))"),
        rw!("cot(x)=(cot^2(x/2)-1)/2cot(x/2)";
            "(cot ?x)" => "(/ (- (pow (cot (/ ?x 2)) 2) 1) (* 2 (cot (/ ?x 2))))"),
        rw!("(cot^2(x)-1)/2cot(x)=cot(2x)";
            "(/ (- (pow (cot ?x) 2) 1) (* 2 (cot ?x)))" => "(cot (* 2 ?x))"),
        rw!("cot(x)=(1-tan^2(x/2))/(2tan(x/2))";
            "(cot ?x)" => "(/ (- 1 (pow (tan (/ ?x 2)) 2)) (* 2 (tan (/ ?x 2))))"),
        rw!("(1-tan^2(x))/(2tan(x))=cot(2x)";
            "(/ (- 1 (pow (tan ?x) 2)) (* 2 (tan ?x)))" => "(cot (* 2 ?x))"),
        /* triple-angle formulae */
        rw!("sin(x)=3sin(x/3)-4sin^3(x/3)";
            "(sin ?x)" => "(- (* 3 (sin (/ ?x 3))) (* 4 (pow (sin (/ ?x 3)) 3)))"),
        rw!("3sin(x)-4sin^3(x)=sin(3x)";
            "(- (* 3 (sin ?x)) (* 4 (pow (sin ?x) 3)))" => "(sin ( * 3 ?x))"),
        rw!("cos(x)=4cos^3(x/3)-3cos(x/3)";
            "(cos ?x)" => "(- (* 4 (pow (cos (/ ?x 3)) 3)) (* 3 (cos (/ ?x 3))))"),
        rw!("4cos^3(x)-3cos(x)=cos(3x)";
            "(- (* 4 (pow (cos ?x) 3)) (* 3 (cos ?x)))" => "(cos ( * 3 ?x))"),
        // tan & csc & sec & cot rw exceed length limit
        /* half-angle formulae */
        rw!("sin(x)=sqrt((1-cos(2x))/2)"; "(sin ?x)" => "(sqrt (/ (- 1 (cos (* 2 ?x))) 2))"),
        rw!("sqrt((1-cos(x))/2)=sin(2x)"; "(sqrt (/ (- 1 (cos ?x)) 2))" => "(sin (/ ?x 2))"),
        rw!("sin(x)=-sqrt((1-cos(2x))/2)"; "(sin ?x)" => "(* -1 (sqrt (/ (- 1 (cos (* 2 ?x))) 2)))"),
        rw!("-sqrt((1-cos(x))/2)=sin(2x)"; "(* -1 (sqrt (/ (- 1 (cos ?x)) 2)))" => "(sin (/ ?x 2))"),
        rw!("cos(x)=sqrt((1+cos(2x))/2)"; "(cos (/ ?x 2))" => "(sqrt (/ (+ 1 (cos (* 2 ?x))) 2))"),
        rw!("sqrt((1+cos(x))/2)=cos(2x)"; "(sqrt (/ (+ 1 (cos ?x)) 2))" => "(cos (/ ?x 2))"),
        rw!("cos(x)=-sqrt((1+cos(2x))/2)"; "(cos (/ ?x 2))" => "(* -1 (sqrt (/ (+ 1 (cos (* 2 ?x))) 2)))"),
        rw!("-sqrt((1+cos(x))/2)=cos(2x)"; "(* -1 (sqrt (/ (+ 1 (cos ?x)) 2)))" => "(cos (/ ?x 2))"),
        rw!("tan(x)=(1-cos(2x))/sin(2x)"; "(tan ?x)" => "(/ (- 1 (cos (* 2 ?x))) (sin (* 2 ?x)))"),
        rw!("(1-cos(x))/sin(x)=tan(x/2)"; "(/ (- 1 (cos ?x)) (sin ?x))" => "(tan (/ ?x 2))"),
        rw!("tan(x)=sin(2x)/(1+cos(2x))"; "(tan ?x)" => "(/ (sin (* 2 ?x)) (+ 1 (cos (* 2 ?x))))"),
        rw!("(sin(x)/(1+cos(x))=tan(x/2)"; "(/ (sin ?x) (+ 1 (cos ?x)))" => "(tan (/ ?x 2))"),
        rw!("tan(x)=csc(2x)-cot(2x)"; "(tan ?x)" => "(- (csc (* 2 ?x)) (cot (* 2 ?x)))"),
        rw!("csc(x)-cot(x)=tan(x/2)"; "(- (csc ?x) (cot ?x))" => "(tan (/ ?x 2))"),
        rw!("tan(x)=tan(2x)/(1+sec(2x))"; "(tan ?x)" => "(/ (tan (* 2 ?x)) (+ 1 (sec (* 2 ?x))))"),
        rw!("tan(x)/(1+sec(x))=tan(x/2)"; "(/ (tan (* 2 ?x)) (+ 1 (sec (* 2 ?x))))" => "(tan (/ ?x 2))"),
        rw!("tan(x)=sqrt((1-cos(2x))/(1+cos(2x)))";
            "(tan ?x)" => "(sqrt (/ (- 1 (cos (* 2 ?x))) (+ 1 (cos (* 2 ?x))))"),
        rw!("sqrt((1-cos(2x))/(1+cos(2x)))=tan(x)";
            "(sqrt (/ (- 1 (cos ?x)) (+ 1 (cos ?x)))" => "(tan (/ ?x 2))"),
        rw!("tan(x)=-sqrt((1-cos(2x))/(1+cos(2x)))";
            "(tan (/ ?x 2))" => "(* -1 (sqrt (/ (- 1 (cos (* 2 ?x))) (+ 1 (cos (* 2 ?x)))))"),
        rw!("-sqrt((1-cos(2x))/(1+cos(2x)))=tan(x)";
            "(* -1 (sqrt (/ (- 1 (cos ?x)) (+ 1 (cos ?x))))" => "(tan (/ ?x 2))"),
        rw!("csc(x)=sqrt(2/(1-cos(2x)))"; "(csc ?x)" => "(sqrt (/ 2 (- 1 (cos (* 2 ?x)))))"),
        rw!("sqrt(2/(1-cos(x)))=csc(x/2)"; "(sqrt (/ 2 (- 1 (cos ?x))))" => "(csc (/ ?x 2))"),
        rw!("csc(x)=-sqrt(2/(1-cos(2x)))"; "(csc ?x)" => "(* -1 (sqrt (/ 2 (- 1 (cos (* 2 ?x))))))"),
        rw!("-sqrt(2/(1-cos(x)))=csc(x/2)"; "( * -1 (sqrt (/ 2 (- 1 (cos ?x)))))" => "(csc (/ ?x 2))"),
        rw!("sec(x)=sqrt(2/(1+cos(2x)))"; "(sec ?x)" => "(sqrt (/ 2 (+ 1 (cos (* 2 ?x)))))"),
        rw!("sqrt(2/(1+cos(x)))=csc(x/2)"; "(sqrt (/ 2 (+ 1 (cos ?x))))" => "(sec (/ ?x 2))"),
        rw!("sec(x)=-sqrt(2/(1+cos(2x)))"; "(sec ?x)" => "(* -1 (sqrt (/ 2 (+ 1 (cos (* 2 ?x))))))"),
        rw!("-sqrt(2/(1+cos(x)))=csc(x/2)"; "(* -1 (sqrt (/ 2 (+ 1 (cos ?x)))))" => "(sec (/ ?x 2))"),
        rw!("cot(x)=(1+cos(2x))/sin(2x)"; "(cot ?x)" => "(/ (+ 1 (cos (* 2 ?x))) (sin (* 2 ?x)))"),
        rw!("(1+cos(x))/sin(x)=cot(x/2)"; "(/ (+ 1 (cos ?x)) (sin ?x))" => "(cot (/ ?x 2))"),
        rw!("cot(x)=sin(2x)/(1-cos(2x))"; "(cot ?x)" => "(/ (sin (* 2 ?x)) (- 1 (cos (* 2 ?x))))"),
        rw!("sin(x)/(1-cos(x))=cot(x/2)"; "(/ (sin ?x) (- 1 (cos ?x)))" => "(cot (/ ?x 2))"),
        rw!("cot(x)=csc(2x)+cot(2x)"; "(cot ?x)" => "(+ (csc (* 2 ?x)) (cot (* 2 ?x)))"),
        rw!("csc(x)+cot(x)=cot(x/2)"; "(+ (csc ?x) (cot ?x))" => "(cot (/ ?x 2))"),
        rw!("cot(x)=sqrt((1+cos(2x))/(1-cos(2x)))";
            "(cot ?x)" => "(sqrt (/ (+ 1 (cos (* 2 ?x))) (- 1 (cos (* 2 ?x))))"),
        rw!("sqrt((1+cos(2x))/(1-cos(2x)))=cot(x)";
            "(sqrt (/ (+ 1 (cos ?x)) (- 1 (cos ?x)))" => "(cot (/ ?x 2))"),
        rw!("cot(x)=-sqrt((1+cos(2x))/(1-cos(2x)))";
            "(cot (/ ?x 2))" => "(* -1 (sqrt (/ (+ 1 (cos (* 2 ?x))) (- 1 (cos (* 2 ?x)))))"),
        rw!("-sqrt((1+cos(2x))/(1-cos(2x)))=cot(x)";
            "(* -1 (sqrt (/ (+ 1 (cos ?x)) (- 1 (cos ?x))))" => "(cot (/ ?x 2))"),
        /* power-reduction formulae */
        rw!("sin^2(x)=(1-cos(2x))/2"; "(pow (sin ?x) 2)" => "(/ (- 1 (cos (* 2 ?x))) 2)"),
        rw!("cos^2(x)=(1+cos(2x))/2"; "(pow (cos ?x) 2)" => "(/ (+ 1 (cos (* 2 ?x))) 2)"),
        rw!("sin^2(x)cos^2(x)=(1-cos(4x))/8";
            "(* (pow (sin ?x) 2) (pow (cos ?x) 2))" => "(/ (- 1 (cos (* 4 ?x))) 8)"),
        rw!("sin^3(x)=(3sin(x)-sin(3x))/4";
            "(pow (sin ?x) 3)" => "(/ (- (* 3 (sin ?x)) (sin (* 3 ?x))) 4)"),
        rw!("cos^3(x)=(3cos(x)+cos(3x))/4";
            "(pow (cos ?x) 3)" => "(/ (+ (* 3 (cos ?x)) (cos (* 3 ?x))) 4)"),
        rw!("sin^3(x)cos^3(x)=(3sin(2x)-sin(6x))/32";
            "(* (pow (sin ?x) 3) (pow (cos ?x) 3))" => "(/ (- (* 3 (sin (* 2 ?x))) (sin (* 6 ?x))) 32)"),
        rw!("sin^4(x)=(3-4cos(2x)+cos(4x))/8";
            "(pow (sin ?x) 4)" => "(/ (+ (- 3 (* 4 (cos (* 2 ?x)))) (cos (* 4 ?x))) 8)"),
        rw!("cos^4(x)=(3+4cos(2x)+cos(4x))/8";
            "(pow (cos ?x) 4)" => "(/ (+ (+ 3 (* 4 (cos (* 2 ?x)))) (cos (* 4 ?x))) 8)"),
        rw!("sin^4(x)cos^4(x)=(3sin(2x)-sin(6x))/32";
            "(* (pow (sin ?x) 4) (pow (cos ?x) 4))" => "(/ (+ (- 3 (* 4 (cos (* 4 ?x)))) (cos (* 8 ?x))) 128)"),
        // sin^5(x) & cos^5(x) & sin^5(x)cos^5(x) exceed length limit
        /* product-to-sum identities */
        rw!("sin(a)sin(b)=(cos(a-b)-cos(a+b))/2";
            "(* (sin ?x) (sin ?y))" => "(/ (- (cos (- ?x ?y)) (cos (+ ?x ?y))) 2)"),
        rw!("(cos(a-b)-cos(a+b))=2sin(a)sin(b)";
            "(- (cos (- ?x ?y)) (cos (+ ?x ?y)))" => "(* 2 (* (sin ?x) (sin ?y)))"),
        rw!("cos(a)cos(b)=(cos(a-b)+cos(a+b))/2";
            "(* (cos ?x) (cos ?y))" => "(/ (+ (cos (- ?x ?y)) (cos (+ ?x ?y))) 2)"),
        rw!("(cos(a-b)+cos(a+b))=2cos(a)cos(b)";
            "(+ (cos (- ?x ?y)) (cos (+ ?x ?y)))" => "(* 2 (* (cos ?x) (cos ?y)))"),
        rw!("sin(a)cos(b)=(sin(a+b)+sin(a-b))/2";
            "(* (sin ?x) (cos ?y))" => "(/ (+ (sin (+ ?x ?y)) (sin (- ?x ?y))) 2)"),
        rw!("(sin(a+b)+sin(a-b))/2=sin(a)cos(b)";
            "(+ (sin (+ ?x ?y)) (sin (- ?x ?y)))" => "(* 2 (* (sin ?x) (cos ?y)))"),
        rw!("cos(a)sin(b)=(sin(a+b)-sin(a-b))/2";
            "(* (cos ?x) (sin ?y))" => "(/ (- (sin (+ ?x ?y)) (sin (- ?x ?y))) 2)"),
        rw!("(sin(a+b)-sin(a-b))=2cos(a)sin(b)";
            "(- (sin (+ ?x ?y)) (sin (- ?x ?y)))" => "(* 2 (* (cos ?x) (sin ?y)))"),
        // tan(a)tan(b) & tan(a)cot(b) rw exceed length limit
        /* sum-to-product identities */
        rw!("sin(a)+sin(b)=2sin((a+b)/2)cos((a-b)/2)";
            "(+ (sin ?x) (sin ?y))" => "(* 2 (* (sin (/ (+ ?x ?y) 2)) (cos (/ (- ?x ?y) 2))))"),
        rw!("sin(a)-sin(b)=2cos((a+b)/2)sin((a-b)/2)";
            "(- (sin ?x) (sin ?y))" => "(* 2 (* (cos (/ (+ ?x ?y) 2)) (sin (/ (- ?x ?y) 2))))"),
        rw!("cos(a)+cos(b)=2cos((a+b)/2)cos((a-b)/2)";
            "(+ (cos ?x) (cos ?y))" => "(* 2 (* (cos (/ (+ ?x ?y) 2)) (cos (/ (- ?x ?y) 2))))"),
        rw!("cos(a)-cos(b)=2sin((a+b)/2)sin((a-b)/2)";
            "(- (cos ?x) (cos ?y))" => "(* -2 (* (sin (/ (+ ?x ?y) 2)) (sin (/ (- ?x ?y) 2))))"),
        rw!("tan(a)+tan(b)=sin(a+b)/(cos(a)cos(b))";
            "(sin (+ ?a ?b))" => "(/ (sin (+ ?a ?b)) (* (cos ?a) (cos ?b)))"),
        rw!("tan(a)-tan(b)=sin(a-b)/(cos(a)cos(b))";
            "(sin (- ?a ?b))" => "(/ (sin (- ?a ?b)) (* (cos ?a) (cos ?b)))"),
        /* ========================================================================== */

        /* ========================= inverse trig =========================== */
        // arcsin x + arcsin y .....
        /* ================================================================== */

        /* ========================== hyperbolic ============================ */
        /* basic identity */
        rw!("sinh(x)=((e^x-e^-x)/2)"; "(sinh ?x)" => "(/ (- (pow e ?x) (pow e (* -1 ?x))) 2)"),
        rw!("cosh(x)=((e^x+e^-x)/2)"; "(cosh ?x)" => "(/ (+ (pow e ?x) (pow e (* -1 ?x))) 2)"),
        rw!("cosh(x)+sinh(x)=e^x"; "(+ (cosh ?x) (sinh ?x))" => "(pow e ?x)"),
        rw!("cosh(x)-sinh(x)=e^(-x)"; "(- (cosh ?x) (sinh ?x))" => "(pow e (* -1 ?x))"),
        /* pythagorean identity <-> */
        rw!("cosh^2(x)-sinh^2(x)->1"; "(- (pow (cosh ?x) 2) (pow (sinh ?x) 2))" => "1"),
        rw!("1-tanh^2->sech^2"; "(- 1 (pow (tanh ?x) 2))" => "(pow (sech ?x) 2)"),
        rw!("coth^2-1->csch^2"; "(- (pow (coth ?x) 2) 1)" => "(pow (csch ?x) 2)"),
        // rw!("1->cosh^2(x)-sinh^2(x)"; "1" => "(- (pow (cosh ?x) 2) (pow (sinh ?x) 2))"),
        rw!("sech^2->1-tanh^2->"; "(pow (sech ?x) 2)" => "(- 1 (pow (tanh ?x) 2))"),
        rw!("csch^2->coth^2-1"; "(pow (csch ?x) 2)" => "(- (pow (coth ?x) 2) 1)"),
        /* even-odd identity <-> */
        rw!("sinh(-x)=-sinh(x)"; "(sinh (* -1 ?x))" => "(* -1 (sinh ?x))"),
        rw!("cosh(-x)=cosh(x)"; "(cosh (* -1 ?x))" => "(cosh ?x)"),
        rw!("tanh(-x)=-tanh(x)"; "(tanh (* -1 ?x))" => "(* -1 (tanh ?x))"),
        rw!("csch(-x)=-csch(x)"; "(csch (* -1 ?x))" => "(* -1 (csch ?x))"),
        rw!("sech(-x)=sech(x)"; "(sech (* -1 ?x))" => "(sech ?x)"),
        rw!("coth(-x)=-coth(x)"; "(coth (* -1 ?x))" => "(* -1 (coth ?x))"),
        rw!("-sinh(x)=sinh(-x)"; "(* -1 (sinh ?x))" => "(sinh (* -1 ?x))"),
        rw!("cosh(x)=cosh(-x)"; "(cosh ?x)" => "(cosh (* -1 ?x))"),
        rw!("-tanh(x)=tanh(-x)"; "(* -1 (tanh ?x))" => "(tanh (* -1 ?x))"),
        rw!("-csch(x)=csch(-x)"; "(* -1 (csch ?x))" => "(csch (* -1 ?x))"),
        rw!("sech(x)=sech(-x)"; "(sech ?x)" => "(sech (* -1 ?x))"),
        rw!("-coth(x)=coth(-x)"; "(* -1 (coth ?x))" => "(coth (* -1 ?x))"),
        /* double angle identity */
        rw!("sinh(x)->2sinh(0.5x)cosh(0.5x)"; "(sinh ?x)" => "(* 2 (* (sinh (/ ?x 2)) (cosh (/ ?x 2))))"),
        rw!("cosh(x)->sinh^2+cosh^2"; "(cosh ?x)" => "(+ (pow (sinh (/ ?x 2)) 2) (pow (cosh (/ ?x 2)) 2))"),
        rw!("cosh(x)->2sinh^2+1"; "(cosh ?x)" => "(+ (* 2 (pow (sinh (/ ?x 2)) 2)) 1)"),
        rw!("cosh(x)->2cosh^2-1"; "(cosh ?x)" => "(- (* 2 (pow (cosh (/ ?x 2)) 2)) 1)"),
        rw!("tanh(x)->2tanh(x)/(1+tanh^2)"; "(tanh ?x)" => "(/ (* 2 (tanh (/ ?x 2))) (+ 1 (pow (tanh (/ ?x 2)) 2)))"),
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
        /* ================================================================== */

        /* ============================ inv hyperbolic ============================== */
        /* basic identity */
        rw!("asinh(x)->ln(x+sqrt(x^2+1))"; "(asinh ?x)" => "(ln (+ ?x (sqrt (+ (pow ?x 2) 1))))"),
        rw!("acosh(x)->ln(x+sqrt(x^2-1))"; "(acosh ?x)" => "(ln (+ ?x (sqrt (- (pow ?x 2) 1))))"),
        rw!("atanh(x)->((1/2)ln((1+x)/(1-x)))"; "(atanh ?x)" => "(* (/ 1 2) (ln (/ (+ 1 ?x) (- 1 ?x))))"),
        rw!("acsch(x)->ln(1/x+sqrt(1/x^2+1))"; "(acsch ?x)" => "(ln (+ (/ 1 ?x) (sqrt (+ (/ 1 (pow ?x 2)) 1))))"),
        rw!("asech(x)->ln(1/x+sqrt(1/x^2-1))"; "(asech ?x)" => "(ln (+ (/ 1 ?x) (sqrt (- (/ 1 (pow ?x 2)) 1))))"),
        rw!("acoth(x)->(1/2)ln((x+1)/(x-1))"; "(acoth ?x)" => "(* (/ 1 2) (ln (/ (+ ?x 1) (- ?x 1))))"),
        /* other identities */
        rw!("ln->acosh((x^2+1)/(2x))"; "(ln ?x)" => "(acosh (/ (+ (pow ?x 2) 1) (* 2 ?x)))"),
        rw!("acosh((x^2+1)/(2x))->ln"; "(acosh (/ (+ (pow ?x 2) 1) (* 2 ?x)))" => "(ln ?x)"),
        rw!("ln->asinh((x^2-1)/(2x))"; "(ln ?x)" => "(asinh (/ (- (pow ?x 2) 1) (* 2 ?x)))"),
        rw!("asinh((x^2-1)/(2x))->ln"; "(asinh (/ (- (pow ?x 2) 1) (* 2 ?x)))" => "(ln ?x)"),
        rw!("ln->atanh((x^2-1)/(x^2+1))"; "(ln ?x)" => "(atanh (/ (- (pow ?x 2) 1) (+ (pow ?x 2) 1)))"),
        rw!("atanh((x^2-1)/(x^2+1))->ln"; "(atanh (/ (- (pow ?x 2) 1) (+ (pow ?x 2) 1)))" => "(ln ?x)"),
        // a=b=c=d redundant (below)
        // rw!("acosh((x^2+1)/(2x))->asinh((x^2-1)/(2x))";
        //     "(acosh (/ (+ (pow ?x 2) 1) (* 2 ?x)))" => "(asinh (/ (- (pow ?x 2) 1) (* 2 ?x)))"),
        // rw!("asinh((x^2-1)/(2x))->acosh((x^2+1)/(2x))";
        //     "(asinh (/ (- (pow ?x 2) 1) (* 2 ?x)))" => "(acosh (/ (+ (pow ?x 2) 1) (* 2 ?x)))"),
        // rw!("acosh((x^2+1)/(2x))->atanh((x^2-1)/(x^2+1))";
        //     "(acosh (/ (+ (pow ?x 2) 1) (* 2 ?x)))" => "(atanh (/ (- (pow ?x 2) 1) (+ (pow ?x 2) 1)))"),
        // rw!("atanh((x^2-1)/(x^2+1))->acosh((x^2+1)/(2x))";
        //     "(atanh (/ (- (pow ?x 2) 1) (+ (pow ?x 2) 1)))" => "(acosh (/ (+ (pow ?x 2) 1) (* 2 ?x)))"),
        // rw!("asinh((x^2-1)/(2x))->atanh((x^2-1)/(x^2+1))";
        //     "(asinh (/ (- (pow ?x 2) 1) (* 2 ?x)))" => "(atanh (/ (- (pow ?x 2) 1) (+ (pow ?x 2) 1)))"),
        // rw!("atanh((x^2-1)/(x^2+1))->asinh((x^2-1)/(2x))";
        //     "(atanh (/ (- (pow ?x 2) 1) (+ (pow ?x 2) 1)))" => "(asinh (/ (- (pow ?x 2) 1) (* 2 ?x)))"),
        /* composition of hyperbolic and inverse hyperbolic functions */
        rw!("sinh(acosh)->sqrt(x^2-1)"; "(sinh (acosh ?x))" => "(sqrt (- (pow ?x 2) 1))"),
        rw!("sqrt(x^2-1)->sinh(acosh)"; "(sqrt (- (pow ?x 2) 1))" => "(sinh (acosh ?x))"),
        rw!("sinh(atanh)->x/sqrt(1-x^2)"; "(sinh (atanh ?x))" => "(/ ?x (sqrt (- 1 (pow ?x 2))))"),
        rw!("x/sqrt(1-x^2)->sinh(atanh)"; "(/ ?x (sqrt (- 1 (pow ?x 2))))" => "(sinh (atanh ?x))"),
        rw!("cosh(asinh)->sqrt(1+x^2)"; "(cosh (asinh ?x))" => "(sqrt (+ 1 (pow ?x 2)))"),
        rw!("sqrt(1+x^2)->cosh(asinh)"; "(sqrt (+ 1 (pow ?x 2)))" => "(cosh (asinh ?x))"),
        rw!("cosh(atanh)->1/sqrt(1-x^2)"; "(cosh (atanh ?x))" => "(/ 1 (sqrt (- 1 (pow ?x 2))))"),
        rw!("1/sqrt(1-x^2)->cosh(atanh)"; "(/ 1 (sqrt (- 1 (pow ?x 2))))" => "(cosh (atanh ?x))"),
        rw!("tanh(asinh)->x/sqrt(1+x^2)"; "(tanh (asinh ?x))" => "(/ ?x (sqrt (+ 1 (pow ?x 2))))"),
        rw!("x/sqrt(1+x^2)->tanh(asinh)"; "(/ ?x (sqrt (+ 1 (pow ?x 2))))" => "(tanh (asinh ?x))"),
        rw!("tanh(acosh)->sqrt(x^2-1)/x"; "(tanh (acosh ?x))" => "(/ (sqrt (- (pow ?x 2) 1)) ?x)"),
        rw!("sqrt(x^2-1)/x->tanh(acosh)"; "(/ (sqrt (- (pow ?x 2) 1)) ?x)" => "(tanh (acosh ?x))"),
        /* composition of inverse hyperbolic and circular functions */
        rw!("ln(|tan|)->-atan(cos(2x))"; "(ln (abs (tan ?x)))" => "(atan (cos (* 2 ?x)))"),
        rw!("-atan(cos)->ln(|tan(0.5x)|)"; "(atan (cos (* ?x)))" => "(ln (abs (tan (* 0.5 ?x))))"),
        rw!("asinh(tan)->atanh(sin)"; "(asinh (tan ?x))" => "(atanh (sin ?x))"),
        rw!("atanh(sin)->asinh(tan)"; "(atanh (sin ?x))" => "(asinh (tan ?x))"),
        rw!("asinh(tan)->ln((1+sin)/(cos))"; "(asinh (tan ?x))" => "(ln (/ (+ 1 (sin ?x)) (cos ?x)))"),
        rw!("ln((1+sin)/(cos))->asinh(tan)"; "(ln (/ (+ 1 (sin ?x)) (cos ?x)))" => "(asinh (tan ?x))"),
        rw!("asinh(tan)->acosh(1/cos)"; "(asinh (tan ?x))" => "(acosh (/ 1 (cos ?x)))"),
        rw!("acosh(1/cos)->asinh(tan)"; "(acosh (/ 1 (cos ?x)))" => "(asinh (tan ?x))"),
        rw!("asinh(tan)->-acosh(1/cos)"; "(asinh (tan ?x))" => "(* -1 (acosh (/ 1 (cos ?x))))"),
        rw!("-acosh(1/cos)->asinh(tan)"; "(* -1 (acosh (/ 1 (cos ?x))))" => "(asinh (tan ?x))"),
        // a=b=c=d redundant (below)
        // rw!("atanh(sin)->ln((1+sin)/(cos))"; "(atanh (sin ?x))" => "(ln (/ (+ 1 (sin ?x)) (cos ?x)))"),
        // rw!("ln((1+sin)/(cos))->atanh(sin)"; "(ln (/ (+ 1 (sin ?x)) (cos ?x)))" => "(atanh (sin ?x))"),
        // rw!("atanh(sin)->acosh(1/cos)"; "(atanh (sin ?x))" => "(acosh (/ 1 (cos ?x)))"),
        // rw!("acosh(1/cos)->atanh(sin)"; "(acosh (/ 1 (cos ?x)))" => "(atanh (sin ?x))"),
        // rw!("atanh(sin)->-acosh(1/cos)"; "(atanh (sin ?x))" => "(* -1 (acosh (/ 1 (cos ?x))))"),
        // rw!("-acosh(1/cos)->atanh(sin)"; "(* -1 (acosh (/ 1 (cos ?x))))" => "(atanh (sin ?x))"),
        // rw!("ln((1+sin)/(cos))->acosh(1/cos)"; "(ln (/ (+ 1 (sin ?x)) (cos ?x)))" => "(acosh (/ 1 (cos ?x)))"),
        // rw!("acosh(1/cos)->ln((1+sin)/(cos))"; "(acosh (/ 1 (cos ?x)))" => "(ln (/ (+ 1 (sin ?x)) (cos ?x)))"),
        // rw!("ln((1+sin)/(cos))->-acosh(1/cos)"; "(ln (/ (+ 1 (sin ?x)) (cos ?x)))" => "(* -1 (acosh (/ 1 (cos ?x))))"),
        // rw!("-acosh(1/cos)->ln((1+sin)/(cos))"; "(* -1 (acosh (/ 1 (cos ?x))))" => "(ln (/ (+ 1 (sin ?x)) (cos ?x)))"),
        /* Conversions */
        rw!("ln=atanh((x^-1)/(x^2+1))"; "(ln ?x)" => "(atanh (/ (- (pow ?x 2) 1) (+ (pow ?x 2) 1)))"),
        rw!("atanh((x^-1)/(x^2+1))=ln"; "(atanh (/ (- (pow ?x 2) 1) (+ (pow ?x 2) 1)))" => "(ln ?x)"),
        rw!("ln=asinh((x^-1)/2x)"; "(ln ?x)" => "(asinh (/ (- (pow ?x 2) 1) (* 2 ?x)))"),
        rw!("asinh((x^-1)/2x)=ln"; "(asinh (/ (- (pow ?x 2) 1) (* 2 ?x)))" => "(ln ?x)"),
        rw!("ln=acosh((x^+1)/2x)"; "(ln ?x)" => "(asinh (/ (+ (pow ?x 2) 1) (* 2 ?x)))"),
        rw!("acosh((x^+1)/2x)=ln"; "(asinh (/ (+ (pow ?x 2) 1) (* 2 ?x)))" => "(ln ?x)"),
        rw!("ln=-acosh((x^+1)/2x)"; "(ln ?x)" => "(* -1 (asinh (/ (+ (pow ?x 2) 1) (* 2 ?x))))"),
        rw!("-acosh((x^+1)/2x)=ln"; "(* -1 (asinh (/ (+ (pow ?x 2) 1) (* 2 ?x))))" => "(ln ?x)"),
        rw!("atanh=asinh(x/sqrt(1-x^2))"; "(atanh ?x)" => "(asinh (/ ?x (sqrt (- 1 (pow ?x 2)))))"),
        rw!("asinh(x/sqrt(1-x^2))=atanh"; "(asinh (/ ?x (sqrt (- 1 (pow ?x 2)))))" => "(atanh ?x)"),
        rw!("atanh=acosh(1/sqrt(1-x^2))"; "(atanh ?x)" => "(acosh (/ 1 (sqrt (- 1 (pow ?x 2)))))"),
        rw!("acosh(1/sqrt(1-x^2))=atanh"; "(acosh (/ 1 (sqrt (- 1 (pow ?x 2)))))" => "(atanh ?x)"),
        rw!("atanh=-acosh(1/sqrt(1-x^2))"; "(atanh ?x)" => "(* -1 (acosh (/ 1 (sqrt (- 1 (pow ?x 2))))))"),
        rw!("-acosh(1/sqrt(1-x^2))=atanh"; "(* -1 (acosh (/ 1 (sqrt (- 1 (pow ?x 2))))))" => "(atanh ?x)"),
        // a=b=c=d redundant (below)
        // rw!("asinh(x/sqrt(1-x^2))=acosh(1/sqrt(1-x^2))";
        //     "(asinh (/ ?x (sqrt (- 1 (pow ?x 2)))))" => "(acosh (/ 1 (sqrt (- 1 (pow ?x 2)))))"),
        // rw!("acosh(1/sqrt(1-x^2))=asinh(x/sqrt(1-x^2))";
        //     "(acosh (/ 1 (sqrt (- 1 (pow ?x 2)))))" => "(asinh (/ ?x (sqrt (- 1 (pow ?x 2)))))"),
        // rw!("asinh(x/sqrt(1-x^2))=-acosh(1/sqrt(1-x^2))";
        //     "(asinh (/ ?x (sqrt (- 1 (pow ?x 2)))))" => "(* -1 (acosh (/ 1 (sqrt (- 1 (pow ?x 2))))))"),
        // rw!("-acosh(1/sqrt(1-x^2))=asinh(x/sqrt(1-x^2))";
        //     "(* -1 (acosh (/ 1 (sqrt (- 1 (pow ?x 2))))))" => "(asinh (/ ?x (sqrt (- 1 (pow ?x 2)))))"),
        rw!("asinh=atanh(x/sqrt(1+x^2))"; "(asinh ?x)" => "(atanh (/ ?x (sqrt (+ 1 (pow ?x 2)))))"),
        rw!("atanh(x/sqrt(1+x^2))=asinh"; "(atanh (/ ?x (sqrt (+ 1 (pow ?x 2)))))" => "(asinh ?x)"),
        rw!("asinh=acosh(sqrt(1+x^2))"; "(asinh ?x)" => "(acosh (sqrt (+ 1 (pow ?x 2))))"),
        rw!("acosh(sqrt(1+x^2))=asinh"; "(acosh (sqrt (+ 1 (pow ?x 2))))" => "(asinh ?x)"),
        rw!("asinh=-acosh(sqrt(1+x^2))"; "(asinh ?x)" => "(* -1 (acosh (sqrt (+ 1 (pow ?x 2)))))"),
        rw!("-acosh(sqrt(1+x^2))->asinh"; "(* -1 (acosh (sqrt (+ 1 (pow ?x 2)))))" => "(asinh ?x)"),
        // a=b=c redundant (below)
        // rw!("atanh(x/sqrt(1+x^2))->acosh(sqrt(1+x^2))";
        //     "(atanh (/ ?x (sqrt (+ 1 (pow ?x 2)))))" => "(acosh (sqrt (+ 1 (pow ?x 2))))"),
        // rw!("acosh(sqrt(1+x^2))->atanh(x/sqrt(1+x^2))";
        //     "(acosh (sqrt (+ 1 (pow ?x 2))))" => "(atanh (/ ?x (sqrt (+ 1 (pow ?x 2)))))"),
        // rw!("atanh(x/sqrt(1+x^2))->-acosh(sqrt(1+x^2))";
        //     "(atanh (/ ?x (sqrt (+ 1 (pow ?x 2)))))" => "(* -1 (acosh (sqrt (+ 1 (pow ?x 2)))))"),
        // rw!("-acosh(sqrt(1+x^2))->atanh(x/sqrt(1+x^2))";
        //     "(* -1 (acosh (sqrt (+ 1 (pow ?x 2)))))" => "(atanh (/ ?x (sqrt (+ 1 (pow ?x 2)))))"),
        rw!("acosh=|asinh(sqrt(x^2-1))|"; "(acosh ?x)" => "(abs (asinh (sqrt (- (pow ?x 2) 1))))"),
        rw!("|asinh(sqrt(x^2-1))|=acosh"; "(abs (asinh (sqrt (- (pow ?x 2) 1))))" => "(acosh ?x)"),
        rw!("acosh=|atanh((sqrt(x^2-1))/(x))|";
            "(acosh ?x)" => "(abs (atanh (/ (sqrt (- (pow ?x 2) 1)) ?x)))"),
        rw!("|atanh((sqrt(x^2-1))/(x))|=acosh";
            "(abs (atanh (/ (sqrt (- (pow ?x 2) 1)) ?x)))" => "(acosh ?x)"),
        /* ========================================================================== */

        /* =============================== derivative =============================== */
        /* basic derivative */
        rw!("d/dx c"; "(d ?x ?c)" => "0" if is_const("?c")),
        rw!("d/dx y"; "(d x y)" => "0"),
        rw!("d/dx z"; "(d x z)" => "0"),
        rw!("d/dy x"; "(d y x)" => "0"),
        rw!("d/dy z"; "(d y z)" => "0"),
        rw!("d/dz x"; "(d z x)" => "0"),
        rw!("d/dz y"; "(d z y)" => "0"),
        rw!("d/dx f(x)*g(x)"; "(d ?x (* ?f ?g))" => "(+ (* (d x ?f) ?g) (* ?f (d x ?g)))"),
        /* distributive property */
        rw!("d/dx c*f(x)"; "(d ?x (* ?c ?f))" => "(* ?c (d ?x ?f))" if is_const("?c")),
        rw!("d/dx const*f(x)"; "(d ?x (* c ?f))" => "(* c (d ?x ?f))"),
        rw!("d/dx f(x)+g(x)"; "(d ?x (+ ?f ?g))" => "(+ (d ?x ?f) (d ?x ?g))"),
        rw!("d/dx f(x)-g(x)"; "(d ?x (- ?f ?g))" => "(- (d ?x ?f) (d ?x ?g))"),
        /* generalized chain rule */
        rw!("d/dx f(x)^c"; "(d ?x (pow ?f ?c))" => "(* (* ?c (pow ?f (- ?c 1))) (d ?x ?f))" if is_const("?c")),
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
        /* generalized exponential (chain rule) */
        rw!("d/dx e^u"; "(d ?x (pow e ?u))" => "(* (e ?u) (d ?x ?u))"),
        /* generalized ln (chain rule) */
        rw!("d/dx ln(u)"; "(d ?x (ln ?u))" => "(* (/ 1 ?u) (d ?x ?u))"),
        /* generalized log (chain rule) */
        rw!("d/dx log"; "(d ?x (log ?b ?u))" => "(* (/ 1 (* ?u (ln ?b))) (d ?x ?u))"),
        /* ========================================================================== */

        /* ============================== integration =============================== */
        // rw!("i-one"; "(i 1 ?x)" => "?x"),
        // rw!("i-power-const"; "(i (pow ?x ?c) ?x)" => "(/ (pow ?x (+ ?c 1)) (+ ?c 1))"
        //     if is_const("?c")),
        /* ========================================================================== */
    ]
}