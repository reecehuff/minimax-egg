use egg::{*, rewrite as rw}; 

pub fn generate_tree() -> RecExpr<SimpleLanguage> {
    // Define a simple expression
    // let expr: RecExpr<SimpleLanguage> = "(white (black -3 -1) (black 6 -4) )".parse().unwrap();
    let expr: RecExpr<SimpleLanguage> = "(white (black (white -4 2) (white -10 10)) (black (white 6 -2) (white 3 -5)) )".parse().unwrap();
    
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

fn make_rules() -> Vec<Rewrite<SimpleLanguage, ()>> {
    vec![
        // Again, when considering a move as the white player (us),
        // we pick the move that maximizes the evaluation
        // rw!("white-wants-max"; "(white ?a ?b)" => ""),
        rw!("white-wants-max"; "(white ?a ?b)" => { MinOrMax {
            a: "?a".parse().unwrap(),
            b: "?b".parse().unwrap(),
            min_or_max: "max",
        }}),

        // Similarly, when considering a move as the black player (our opponent),
        // we assume they play perfectly (i.e., pick the move that minimizing the eval)
        rw!("black-wants-min"; "(black ?a ?b)" => { MinOrMax {
            a: "?a".parse().unwrap(),
            b: "?b".parse().unwrap(),
            min_or_max: "min",
        }}),
    ]
}

// See https://docs.rs/egg/latest/egg/trait.Applier.html for more information
struct MinOrMax {
    a: Var,
    b: Var,
    min_or_max: &'static str
}
impl Applier<SimpleLanguage, ()> for MinOrMax {

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

        // Isolate the evaluations of nodes a and b
        // NOTE: evaluation being the last node is simply by construction
        let num_a = egraph[a_id].nodes.last().unwrap().clone();
        let num_b = egraph[b_id].nodes.last().unwrap().clone();

        // Depending on if you want a min or max, add it to the egraph
        let new_id: Id; 
        if self.min_or_max == "min" {
            new_id = egraph.add(num_a.min(num_b));
        } else if self.min_or_max == "max" {
            new_id = egraph.add(num_a.max(num_b));
        } else {
            panic!("min_or_max &str needs to be \"min\" or \"max\"")
        }
        // Add the Id of the evaluation only when it hasn't already been added
        if egraph.union(matched_id, new_id) {
            vec![new_id]
        } else {
            vec![]
        }
    }
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