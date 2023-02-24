use rs_graph::{
    maxflow::pushrelabel, traits::Indexable, vecgraph::VecGraphBuilder, Buildable, Builder, Net,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SimpleMaxFlow {
    edges: HashMap<usize, (usize, usize)>,
    graph_builder: VecGraphBuilder<u32>,
    capacities: Vec<i32>,
}

impl Default for SimpleMaxFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct MaxFlowResult {
    pub flow: Vec<i32>,
    pub max_flow: i32,
}

#[wasm_bindgen]
impl SimpleMaxFlow {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            capacities: Vec::new(),
            graph_builder: Net::new_builder(),
            edges: HashMap::new(),
        }
    }

    pub fn add_nodes(&mut self, n: usize) -> Vec<usize> {
        let nodes = self.graph_builder.add_nodes(n);
        nodes
            .into_iter()
            .map(|node| self.graph_builder.node2id(node))
            .collect()
    }

    pub fn add_node(&mut self) -> usize {
        let node = self.graph_builder.add_node();
        self.graph_builder.node2id(node)
    }

    pub fn add_arc_with_capacity(&mut self, u: usize, v: usize, capacity: i32) -> usize {
        let edge = self
            .graph_builder
            .add_edge(self.graph_builder.id2node(u), self.graph_builder.id2node(v));
        self.capacities.push(capacity);

        let edge_id = self.graph_builder.edge2id(edge);
        self.edges.insert(edge_id, (u, v));

        edge_id
    }

    pub fn solve_maxflow(&self, source: usize, sink: usize) -> Result<JsValue, JsValue> {
        let src = self.graph_builder.id2node(source);
        let snk = self.graph_builder.id2node(sink);
        let (value, flow, _) =
            pushrelabel(&self.graph_builder.clone().into_graph(), src, snk, |e| {
                self.capacities[e.index()]
            });

        let res = MaxFlowResult {
            flow: flow.into_iter().map(|(_, flow)| flow).collect(),
            max_flow: value,
        };

        Ok(serde_wasm_bindgen::to_value(&res)?)
    }

    pub fn tail(&self, edge_id: usize) -> usize {
        self.edges.get(&edge_id).unwrap().0
    }

    pub fn head(&self, edge_id: usize) -> usize {
        self.edges.get(&edge_id).unwrap().1
    }

    pub fn num_arcs(&self) -> usize {
        self.graph_builder.num_edges()
    }
}
