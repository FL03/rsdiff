/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::{DefaultIx, EdgeIndex, NodeIndex};
use core::marker::PhantomData;

pub trait GraphEntry {
    type Idx;
    type Weight;
}

pub trait ComputeGraph {
    type Edge: GraphEntry;
    type Node: GraphEntry;

    fn add_node(
        &mut self,
        node: <Self::Node as GraphEntry>::Weight,
    ) -> <Self::Node as GraphEntry>::Idx;

    fn add_edge(
        &mut self,
        src: <Self::Node as GraphEntry>::Idx,
        target: <Self::Node as GraphEntry>::Idx,
        weight: <Self::Edge as GraphEntry>::Weight,
    ) -> <Self::Edge as GraphEntry>::Idx;

    fn clear(&mut self);
}

pub struct Edge<Idx = DefaultIx, W = ()> {
    pub link: Link<Idx>,
    pub weight: Option<W>,
    _idx: PhantomData<EdgeIndex<Idx>>,
}

impl<Idx, W> Edge<Idx, W> {
    pub fn new(src: NodeIndex<Idx>, target: NodeIndex<Idx>, weight: Option<W>) -> Self {
        Self {
            link: Link::new(src, target),
            weight,
            _idx: PhantomData,
        }
    }

    pub fn from_link(link: Link<Idx>) -> Self {
        Self {
            link,
            weight: None,
            _idx: PhantomData,
        }
    }

    pub fn unweighted(src: NodeIndex<Idx>, target: NodeIndex<Idx>) -> Self {
        Self::new(src, target, None)
    }

    pub fn weighted(src: NodeIndex<Idx>, target: NodeIndex<Idx>, weight: W) -> Self {
        Self::new(src, target, Some(weight))
    }

    pub fn with_weight(self, weight: W) -> Self {
        Self {
            weight: Some(weight),
            ..self
        }
    }

    pub fn is_unweighted(&self) -> bool {
        self.weight.is_none()
    }

    pub fn is_weighted(&self) -> bool {
        self.weight.is_some()
    }

    pub fn weight(&self) -> Option<&W> {
        self.weight.as_ref()
    }

    pub fn weight_mut(&mut self) -> Option<&mut W> {
        self.weight.as_mut()
    }
}

impl<Idx, W> GraphEntry for Edge<Idx, W> {
    type Idx = EdgeIndex<Idx>;
    type Weight = W;
}

pub struct Link<Idx = DefaultIx> {
    pub src: NodeIndex<Idx>,
    pub target: NodeIndex<Idx>,
}

impl<Idx> Link<Idx> {
    pub fn new(src: NodeIndex<Idx>, target: NodeIndex<Idx>) -> Self {
        Self { src, target }
    }
}

pub trait GraphEdge {
    type Idx;

    fn src(&self) -> NodeIndex<Self::Idx>;

    fn target(&self) -> NodeIndex<Self::Idx>;
}

impl<Idx> GraphEdge for Link<Idx>
where
    Idx: Copy,
{
    type Idx = Idx;

    fn src(&self) -> NodeIndex<Idx> {
        self.src
    }

    fn target(&self) -> NodeIndex<Idx> {
        self.target
    }
}

impl<Idx, W> GraphEdge for Edge<Idx, W>
where
    Idx: Copy,
{
    type Idx = Idx;

    fn src(&self) -> NodeIndex<Idx> {
        self.link.src
    }

    fn target(&self) -> NodeIndex<Idx> {
        self.link.target
    }
}
