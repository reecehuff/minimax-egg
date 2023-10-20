use egg::{*, rewrite as rw}; 

pub fn generate_tree() -> RecExpr<SimpleLanguage> {
    // Define the expression
    // let expr: RecExpr<SimpleLanguage> = "(list 3 4)".parse().unwrap();
    let expr: RecExpr<SimpleLanguage> = "(white (black -3 -1) (black 6 -4) )".parse().unwrap();
    
    // Return the expression 
    expr
}

#[allow(dead_code)]
pub fn simple_expr() -> RecExpr<SimpleLanguage> {
    // Define the expression
    let expr: RecExpr<SimpleLanguage> = "(/ (* x 2 ) 2)".parse().unwrap();

    // Return the expression 
    expr
}

define_language! {
    pub enum SimpleLanguage {
        // string variant with no children
        "pi" = Pi,

        // string variants with an array of child `Id`s (any static size)
        // any type that implements LanguageChildren may be used here
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "black" = Black([Id; 2]),
        "max" = Max([Id; 1]),

        // can also do a variable number of children in a boxed slice
        // this will only match if the lengths are the same
        "list" = List(Box<[Id]>),

        // string variants with a single child `Id`
        // note that this is distinct from `Sub`, even though it has the same
        // string, because it has a different number of children
        "-"  = Neg(Id),

        // data variants with a single field
        // this field must implement `FromStr` and `Display`
        Num(i32),
        // language items are parsed in order, and we want symbol to
        // be a fallback, so we put it last
        Symbol(Symbol),
        // This is the ultimate fallback, it will parse any operator (as a string)
        // and any number of children.
        // Note that if there were 0 children, the previous branch would have succeeded
        Other(Symbol, Vec<Id>),
    }
}

fn make_rules() -> Vec<Rewrite<SimpleLanguage, ()>> {
    vec![
        rw!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rw!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        rw!("add-0"; "(+ ?a 0)" => "?a"),
        rw!("mul-0"; "(* ?a 0)" => "0"),
        rw!("mul-1"; "(* ?a 1)" => "?a"),
        rw!("div-mul-same"; "(/ (* ?b ?a) ?b)" => "?a"),

        // Define some rules for taking the min or max of each of the leafs
        // rw!("white-wants-max"; "(white ?a ?b)" => "?a"),
        // rw!("black-wants-min"; "(black ?a ?b)" => "?b")

        // rewrite!("silly"; "(black ?a ?b)" => { MySillyApplier("foo") }),
        rw!("funky"; "(black ?a ?b)" => { MaxApplier {
            a: "?a".parse().unwrap(),
            b: "?b".parse().unwrap(),
        }}),

        // rw!("white-wants-max"; "(white ?a ?b)" => "?a" if max_rule("?a", "?b") ),
        // rw!("black-wants-min"; "(black ?a ?b)" => "?b" )
    ]
}

// See https://docs.rs/egg/latest/egg/trait.Applier.html

struct Funky {
    a: Var,
    b: Var
}

impl Applier<SimpleLanguage, ()> for Funky {

    fn apply_one(&self, egraph: &mut EGraph<SimpleLanguage,()>, matched_id: Id, subst: &Subst, _: Option<&PatternAst<SimpleLanguage>>, _: Symbol) -> Vec<Id> {
        let a: Id = subst[self.a];
        let b: Id = subst[self.b];
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        let a0: Id = egraph.add(SimpleLanguage::Max([a]));
        if egraph.union(matched_id, a0) {
            vec![a0]
        } else {
            vec![]
        }
    }
}
#[derive(Debug)]
struct MaxApplier {
    a: Var,
    b: Var
}

impl Applier<SimpleLanguage, ()> for MaxApplier {

    fn apply_one(&self, egraph: &mut EGraph<SimpleLanguage,()>, matched_id: Id, subst: &Subst, _: Option<&PatternAst<SimpleLanguage>>, _: Symbol) -> Vec<Id> {
        let a_id: Id = subst[self.a];
        let b_id: Id = subst[self.b];
        println!("a   : {:?}", &self.a);
        println!("b   : {:?}", &self.b);
        println!("a_id: {:?}", a_id);
        println!("b_id: {:?}", b_id);
        let value_a = &egraph[a_id].nodes;
        let value_b = &egraph[b_id].nodes;
        println!("a_ns: {:?}", value_a);
        println!("b_ns: {:?}", value_b);

        println!("compare: {:?}", value_a > value_b);
        
        let new_id: Id; 
        if value_a > value_b {
            new_id = egraph.add(SimpleLanguage::Max([a_id]));
        } else {
            new_id = egraph.add(SimpleLanguage::Max([b_id]));
        }

        // let a0: Id = egraph.add(SimpleLanguage::Max([new_id]));
        if egraph.union(matched_id, new_id) {
            vec![new_id]
        } else {
            vec![]
        }
    }
}

// fn max_rule(var_a: &'static str, var_b: &'static str) -> impl Fn(&mut EGraph<SimpleLanguage,()>, Id, &Subst) -> bool {
//     let var_a: Var = var_a.parse().unwrap();
//     let var_b: Var = var_b.parse().unwrap();
//     println!("var_a: {}", var_a);
//     println!("bool : {}", var_a.max(var_b) == var_a);
//     move |egraph, _, subst| var_a.max(var_b) == var_a
//     // move |egraph, _, subst| !egraph[subst[var]].nodes.max(&var)
// }

fn max_rule(var_a: &'static str, var_b: &'static str) -> impl Fn(&mut EGraph<SimpleLanguage,()>, Id, &Subst) -> bool {
    let var_a = var_a.parse().unwrap();
    let var_b = var_b.parse().unwrap();

    move |egraph, _, subst| {
        let value_a = &egraph[subst[var_a]];
        let value_b = &egraph[subst[var_b]];

        // println!("a: {:?}", value_a);
        println!("a: {:?}", value_a.nodes);
        // println!("a: {:?}", value_a.nodes[0]);
        // println!("a: {:?}", value_a.nodes[0].children());
        // println!("a: {:?}", value_a.nodes[0].children().iter().max());
        println!("a: {:?}", value_a.nodes.iter().max());



        // You'll need to implement a way to compare these two values
        // For this example, let's assume there's a method `get_numeric_value` 
        // which extracts a numeric value from an e-graph node
        value_a.data > value_b.data
    }
}
#[derive(Debug)]
struct MySillyApplier(&'static str);
impl Applier<SimpleLanguage, ()> for MySillyApplier {
    fn apply_one(&self, egraph: &mut EGraph<SimpleLanguage,()>, in_id: Id, _: &Subst, _: Option<&PatternAst<SimpleLanguage>>, _: Symbol) -> Vec<Id> {
        println!("{:?}", self);
        println!("{:?}", self.0);
        println!("{:?}", in_id);
        println!("{:?}", egraph[in_id]);
        println!("{:?}", egraph[in_id].nodes);
        let new_id = egraph.add(SimpleLanguage::Other(self.0.into(),vec![]));
        vec![new_id]
    }
}

fn min_rule(var_a: &'static str, var_b: &'static str) -> impl Fn(&mut EGraph<SimpleLanguage,()>, Id, &Subst) -> bool {
    let var_a: Var = var_a.parse().unwrap();
    let var_b: Var = var_b.parse().unwrap();
    println!("var_a: {}", var_a);
    println!("bool : {}", var_a.max(var_b) == var_b);
    move |egraph, _, subst| var_a.max(var_b) == var_a
    // move |egraph, _, subst| !egraph[subst[var]].nodes.max(&var)
}

// fn min_rule(var: &'static str) -> impl Fn(&mut EGraph<SimpleLanguage,()>, Id, &Subst) -> bool {
//     let var = var.parse().unwrap();
//     let zero = SimpleLanguage::Num(0);
//     move |egraph, _, subst| egraph[subst[var]].nodes.iter().min() == Some(&zero)
//     // move |egraph, _, subst| !egraph[subst[var]].nodes.max(&var)
// }

// This returns a function that implements Condition
fn is_not_zero(var: &'static str) -> impl Fn(&mut EGraph<SimpleLanguage,()>, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = SimpleLanguage::Num(0);
    move |egraph, _, subst| !egraph[subst[var]].nodes.contains(&zero)
}

pub fn simplify(string_expr: RecExpr<SimpleLanguage>) -> RecExpr<SimpleLanguage> {

    // Add the expression to the E-Graph
    let mut my_egraph: EGraph<SimpleLanguage, ()> = EGraph::default();
    my_egraph.add_expr(&string_expr);

    // Visualize the old E-Graph as a png
    my_egraph.dot().to_png("target/old_egraph.png").unwrap();

    // Define a runner to simplify the egraph 
    let my_runner = Runner::default().with_expr(&string_expr).run(&make_rules());

    // Extract the best expression using an extractor
    let my_extractor = Extractor::new(&my_runner.egraph, AstSize);
    let (_, best_expr) = my_extractor.find_best(my_runner.roots[0]);

    // Visualize the new E-Graph as a png
    my_runner.egraph.dot().to_png("target/new_egraph.png").unwrap();

    // Return the best expression by leaving off the semi-colon
    best_expr

}