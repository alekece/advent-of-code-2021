use std::cell::RefCell;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Read};
use std::rc::{Rc, Weak};

use colored::Colorize;
use derivative::Derivative;
use eyre::eyre;
use itertools::Itertools;

use aoc_core::Result;

use crate::string::StringExt;

pub struct Path(Vec<Rc<RefCell<Node>>>);

impl fmt::Display for Path {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      self.0.iter().format_with(",", |node, f| {
        match node.borrow().id.as_str() {
          id @ Graph::START_NODE => f(&id.green().bold()),
          id @ Graph::END_NODE => f(&id.red().bold()),
          id => f(&id),
        }
      })
    )
  }
}

#[derive(PartialEq, Eq, Hash)]
enum NodeKind {
  Small,
  Big,
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, Hash)]
struct Node {
  id: String,
  kind: NodeKind,
  #[derivative(Hash = "ignore", PartialEq = "ignore")]
  edges: Vec<Weak<RefCell<Node>>>,
}

impl Node {
  pub fn new(id: String) -> Self {
    let kind = if id.is_uppercase() {
      NodeKind::Big
    } else {
      NodeKind::Small
    };

    Self {
      id,
      kind,
      edges: Default::default(),
    }
  }
}

#[derive(Copy, Clone)]
pub struct SearchPolicy {
  pub max_small_node_visit: usize,
}

pub struct Graph {
  nodes: HashMap<String, Rc<RefCell<Node>>>,
}

impl Graph {
  const START_NODE: &'static str = "start";
  const END_NODE: &'static str = "end";

  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let reader = BufReader::new(reader);

    let edges = reader
      .lines()
      .map(|line| {
        line.map(|s| {
          s.trim()
            .split_once('-')
            .map(|(start_id, end_id)| {
              (
                Node::new(start_id.to_string()),
                Node::new(end_id.to_string()),
              )
            })
            .ok_or_else(|| eyre!("malformed '{}' edge", s))
        })
      })
      .flatten()
      .collect::<Result<Vec<_>>>()?;

    let mut nodes = HashMap::new();

    for (start_node, end_node) in edges.into_iter() {
      let start_node = insert_node(&mut nodes, start_node);
      let end_node = insert_node(&mut nodes, end_node);

      start_node.borrow_mut().edges.push(Rc::downgrade(&end_node));
      end_node.borrow_mut().edges.push(Rc::downgrade(&start_node));
    }

    if nodes.get(Self::START_NODE).is_none() || nodes.get(Self::END_NODE).is_none() {
      Err(eyre!("missing start and/or end nodes"))
    } else {
      Ok(Self { nodes })
    }
  }

  pub fn get_all_paths(&self, policy: SearchPolicy) -> Vec<Path> {
    let (_, start_node) = self
      .nodes
      .iter()
      .find(|(key, _)| key.as_str() == Self::START_NODE)
      .unwrap();

    find_paths(Rc::clone(start_node), Vec::new(), policy)
  }
}

fn find_paths(
  node: Rc<RefCell<Node>>,
  mut visited_nodes: Vec<Rc<RefCell<Node>>>,
  policy: SearchPolicy,
) -> Vec<Path> {
  visited_nodes.push(Rc::clone(&node));

  if node.borrow().id == Graph::END_NODE {
    vec![Path(visited_nodes)]
  } else {
    node
      .borrow()
      .edges
      .iter()
      .filter(|node| can_visit_node(Weak::clone(node), &visited_nodes, policy))
      .map(|node| find_paths(Weak::upgrade(node).unwrap(), visited_nodes.clone(), policy))
      .filter(|paths| !paths.is_empty())
      .flatten()
      .collect()
  }
}

fn can_visit_node(
  node: Weak<RefCell<Node>>,
  visited_nodes: &[Rc<RefCell<Node>>],
  policy: SearchPolicy,
) -> bool {
  let node = Weak::upgrade(&node).unwrap();
  let id = &node.borrow().id;

  id != Graph::START_NODE
    && (matches!(node.borrow().kind, NodeKind::Big)
      || !visited_nodes.contains(&node)
      || visited_nodes
        .iter()
        .filter(|node| matches!(node.borrow().kind, NodeKind::Small))
        .counts_by(|node| {
          let mut hasher = DefaultHasher::new();

          node.borrow().hash(&mut hasher);

          hasher.finish()
        })
        .iter()
        .all(|(_, count)| *count < policy.max_small_node_visit))
}

fn insert_node(
  nodes: &mut HashMap<String, Rc<RefCell<Node>>>,
  new_node: Node,
) -> Rc<RefCell<Node>> {
  let new_node = nodes
    .entry(new_node.id.clone())
    .or_insert_with(|| Rc::new(RefCell::new(new_node)));

  Rc::clone(new_node)
}
