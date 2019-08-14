use regex::Regex;
use serde::Serialize;
use std::cell::RefCell;
use std::error::Error;
use std::fs;

pub struct Config {
  pub input_filename: String,
  pub output_filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let input_filename = args[1].clone();
    let output_filename = args[2].clone();

    Ok(Config {
      input_filename,
      output_filename,
    })
  }
}

type Tag = String;
type Data = String;
type Pointer = String;

#[derive(Debug, Serialize)]
pub struct Node<'a> {
  data: Data,
  tag: Tag,
  pointer: Pointer,
  level: i32,
  children: RefCell<Vec<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
  fn new(level: i32, tag: &str, data: &str, pointer: &str) -> Node<'a> {
    Node {
      level,
      tag: String::from(tag),
      data: String::from(data),
      pointer: String::from(pointer),
      children: RefCell::new(vec![]),
    }
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.input_filename)?;

  let result = parse_to_json(contents)?;

  fs::write(config.output_filename, result)?;
  Ok(())
}

fn parse_to_json(contents: String) -> Result<String, Box<dyn Error>> {
  let mut all_nodes = parse_into_nodes(contents)?;
  let mut dummy_root = Node::new(-1, "dummy", "", "");
  build_tree(&mut all_nodes, &mut dummy_root)?;
  let json_string = serde_json::to_string_pretty(&dummy_root.children)
    .unwrap()
    .to_owned();

  Ok(json_string)
}

macro_rules! asstr {
  () => {
    |m| m.as_str()
  };
}

fn parse_into_nodes<'a>(contents: String) -> Result<Vec<Node<'a>>, Box<dyn Error>> {
  let re = Regex::new(r"\s*(0|[1-9]+[0-9]*) (@[^@]+@ |\b)([A-Za-z0-9_]+)( [^\n\r]*|\b)").unwrap();

  let mut all_nodes = Vec::new();
  for cap in re.captures_iter(contents.as_str()) {
    let level: i32 = cap.get(1).unwrap().as_str().parse()?;
    let pointer = cap.get(2).map_or("", asstr!());
    let tag = cap.get(3).map_or("", asstr!());
    let data = cap.get(4).map_or("", asstr!()).trim();
    let new_node = Node::new(level, tag, data, pointer);

    all_nodes.push(new_node);
  }

  Ok(all_nodes)
}

fn build_tree<'a>(
  ordered_nodes: &'a mut Vec<Node<'a>>,
  dummy_root: &mut Node<'a>,
) -> Result<(), Box<dyn Error>> {
  let mut stack: Vec<&Node> = Vec::new();
  let iter = ordered_nodes.iter_mut();

  stack.push(dummy_root);

  for node in iter {
    let popped = stack.pop();

    if let Some(mut popped) = popped {
      while popped.level >= node.level {
        popped = stack.pop().unwrap();
      }

      if popped.level == node.level - 1 {
        popped.children.borrow_mut().push(node);
      }

      stack.push(popped);
      stack.push(node);
    }
  }

  Ok(())
}
