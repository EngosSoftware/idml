use idml::parse;
use std::fs;

#[test]
fn _0001() {
  let input = ".A\n";
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  let node = root.children().next().unwrap();
  assert_eq!('.', node.delimiter());
  assert_eq!("A", node.name());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
}

#[test]
fn _0002() {
  let input = ">z\n";
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  let node = root.children().next().unwrap();
  assert_eq!('>', node.delimiter());
  assert_eq!("z", node.name());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
}

#[test]
fn _0003() {
  let input = ".A1\n";
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  let node = root.children().next().unwrap();
  assert_eq!('.', node.delimiter());
  assert_eq!("A1", node.name());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
}

#[test]
fn _0004() {
  let input = ".A\n.k\n";
  let root = parse(input).unwrap();
  let mut children = root.children();
  let first = children.next().unwrap();
  assert_eq!('.', first.delimiter());
  assert_eq!("A", first.name());
  assert_eq!("\n", first.content());
  assert_eq!("", first.text());
  let second = children.next().unwrap();
  assert_eq!('.', second.delimiter());
  assert_eq!("k", second.name());
  assert_eq!("\n", second.content());
  assert_eq!("", second.text());
  assert!(children.next().is_none());
}

#[test]
fn _0005() {
  let input = "$x\n$_\n";
  let root = parse(input).unwrap();
  let mut children = root.children();
  let first = children.next().unwrap();
  assert_eq!('$', first.delimiter());
  assert_eq!("x", first.name());
  assert_eq!("\n", first.content());
  assert_eq!("", first.text());
  let second = children.next().unwrap();
  assert_eq!('$', second.delimiter());
  assert_eq!("_", second.name());
  assert_eq!("\n", second.content());
  assert_eq!("", second.text());
  assert!(children.next().is_none());
}

#[test]
fn _0006() {
  let input = "*n\n    *_\n";
  let root = parse(input).unwrap();
  let mut children = root.children();
  let first = children.next().unwrap();
  assert_eq!('*', first.delimiter());
  assert_eq!("n", first.name());
  assert_eq!("\n", first.content());
  assert_eq!("", first.text());
  let second = first.children().next().unwrap();
  assert_eq!('*', second.delimiter());
  assert_eq!("_", second.name());
  assert_eq!("\n", second.content());
  assert_eq!("", second.text());
  assert!(children.next().is_none());
}

#[test]
fn _0007() {
  let content = fs::read_to_string("./examples/compatibility/level_2/2_0001.dmm").expect("failed to load test file");
  let root = parse(&content).unwrap();
  assert_eq!(content, root.document(4));
}
