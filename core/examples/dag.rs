/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_core as acme;

use acme::prelude::Result;
use daggy::petgraph as pg;

use pg::Outgoing;
use pg::algo::toposort;
use pg::visit::IntoEdgesDirected;
use daggy::Dag;

fn main() -> Result<()> {
    let mut dag = Dag::<&str, &str>::new();

    let a = dag.add_node("a");
    let b = dag.add_node("b");
    let c = dag.add_node("ab");
    let d = dag.add_node("aab");

    dag.extend_with_edges([(a, c), (b, c), (a, d), (c, d)])?;

    println!("{:?}", &dag.edges_directed(c, Outgoing));

    let mut res = toposort(&dag, None)?;
    res.reverse();
    println!("{:?}", res);

    Ok(())
}
