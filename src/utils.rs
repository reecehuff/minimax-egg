use egg::{*, rewrite as rw}; 

pub fn test_code(){
    use egg::{*, SymbolLang as S};
    let mut egraph = EGraph::<S, ()>::default();
    let x = egraph.add(S::leaf("x"));
    let y = egraph.add(S::leaf("y"));
    let plus = egraph.add(S::new("+", vec![x, y]));
    let plus_recexpr = "(+ x y)".parse().unwrap();
    assert_eq!(plus, egraph.add_expr(&plus_recexpr));
}

pub fn generate_tree() -> RecExpr<SimpleLanguage> {
    // Define the expression
    // let expr: RecExpr<SimpleLanguage> = "(list 3 4)".parse().unwrap();
    let expr: RecExpr<SimpleLanguage> = "(white (black -3 -1) (black 6 -4) )".parse().unwrap();
    // let expr: RecExpr<SimpleLanguage> = "(white (black (white -4 2) (white -10 10)) (black (white 6 -2) (white 3 -5)) )".parse().unwrap();
    
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

        // Let consider that we are the white pieces, 
        // and our goal is **maximize** the evaluation...
        // Therefore, when presented with a choice of moves, 
        // we will select the move that maximizes the evaluation.
        // We will assume that black plays perfectly. 
        // That is to say that black will always pick the move that 
        // **minimizes** the evaluation.

        // White pieces
        "white" = White([Id; 2]),
        "max" = Max([Id; 1]),

        // Black pieces
        "black" = Black([Id; 2]),
        "min" = Min([Id; 1]),

        // Our evaluation will just be integers for simplicity
        Num(i32),

    }
}

impl SimpleLanguage {
    /// Create a tuple with the given operation and children
    pub fn new(op: impl Into<Symbol>, children: Vec<Id>) -> (Symbol, Vec<Id>) {
        let op = op.into();
        (op, children)
    }
}

fn make_rules() -> Vec<Rewrite<SimpleLanguage, ()>> {
    vec![
        // Again, when considering a move as the white player (us),
        // we pick the move that maximizes the evaluation
        rw!("white-wants-max"; "(white ?a ?b)" => { MaxApplier {
            a: "?a".parse().unwrap(),
            b: "?b".parse().unwrap(),
        }}),

        // Similarly, when considering a move as the black player (our opponent),
        // we assume they play perfectly (i.e., pick the move that minimizing the eval)
        rw!("black-wants-min"; "(black ?a ?b)" => { MaxApplier {
            a: "?b".parse().unwrap(),
            b: "?a".parse().unwrap(),
        }}),
    ]
}

// See https://docs.rs/egg/latest/egg/trait.Applier.html for more information
struct MaxApplier {
    a: Var,
    b: Var
}
impl Applier<SimpleLanguage, ()> for MaxApplier {

    fn apply_one(&self, egraph: &mut EGraph<SimpleLanguage,()>, matched_id: Id, subst: &Subst, _: Option<&PatternAst<SimpleLanguage>>, _: Symbol) -> Vec<Id> {

        // Let's begin discussing what we have access to:
        //  self.a is the Var of the a enode
        //  self.b is the Var of the b enode
        //  egraph is the input EGraph
        //  matched_id is the Id input root enode
        //  subst maps from Var to Id
        // Therefore, we can grab the two children Id's of our parent Id (matched_id)
        let a_id = subst[self.a];
        let b_id = subst[self.b];
        println!("parent Id: {} ", matched_id);
        println!("  has children");
        println!("  child a: {} ", a_id);
        println!("  child b: {} ", b_id);




        vec![]

        // if egraph.union(matched_id, new_id) {
        //     vec![new_id]
        // } else {
        //     vec![]
        // }
    }
}

// struct MinApplier {
//     a: Var,
//     b: Var
// }
// impl Applier<SimpleLanguage, ()> for MinApplier {

//     fn apply_one(&self, egraph: &mut EGraph<SimpleLanguage,()>, matched_id: Id, subst: &Subst, _: Option<&PatternAst<SimpleLanguage>>, _: Symbol) -> Vec<Id> {
//         let a_id: Id = subst[self.a];
//         let b_id: Id = subst[self.b];
//         println!("a   : {:?}", &self.a);
//         println!("b   : {:?}", &self.b);
//         println!("a_id: {:?}", a_id);
//         println!("b_id: {:?}", b_id);
//         let value_a = &egraph[a_id].nodes;
//         let value_b = &egraph[b_id].nodes;
//         println!("a_ns: {:?}", value_a);
//         println!("b_ns: {:?}", value_b);

//         println!("compare: {:?}", value_a > value_b);
        
//         let new_id: Id; 
//         if value_a > value_b {
//             new_id = egraph.add(SimpleLanguage::Min([b_id]));
//         } else {
//             new_id = egraph.add(SimpleLanguage::Min([a_id]));
//         }

//         // let a0: Id = egraph.add(SimpleLanguage::Max([new_id]));
//         if egraph.union(matched_id, new_id) {
//             vec![new_id]
//         } else {
//             vec![]
//         }
//     }
// }

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
    for class in my_runner.egraph.classes() {
        println!("class {:?}", class);
    }

    // Return the best expression by leaving off the semi-colon
    best_expr

}