/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_core as acme;

use acme::prelude::Result;
use daggy::petgraph::algo::toposort;
use daggy::Dag;

fn main() -> Result<()> {
    let mut dag = Dag::<&str, &str>::new();

    let a = dag.add_node("a");

    let d = dag.add_node("aab");
    let b = dag.add_node("b");
    let c = dag.add_node("ab");

    dag.extend_with_edges([(a, b), (b, c), (a, d), (c, d)])?;

    let mut res = toposort(&dag, None)?;
    res.reverse();
    println!("{:?}", res);

    Ok(())
}
