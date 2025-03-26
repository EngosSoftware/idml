use idml::parse;

#[test]
fn _0001() {
  let input = ".A\n";
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  let node = root.children().next().unwrap();
  assert_eq!('.', node.delimiter());
  assert_eq!("A", node.name());
  assert_eq!("a", node.tag());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
}

#[test]
fn _0002() {
  let input = "😀_this_is-pascal_case-tag\n";
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  let node = root.children().next().unwrap();
  assert_eq!('😀', node.delimiter());
  assert_eq!("_this_is-pascal_case-tag", node.name());
  assert_eq!("ThisIsPascalCaseTag", node.tag());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
}

#[test]
fn _0003() {
  let input = "😀this_is-camel_case-tag\n";
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  let node = root.children().next().unwrap();
  assert_eq!('😀', node.delimiter());
  assert_eq!("this_is-camel_case-tag", node.name());
  assert_eq!("thisIsCamelCaseTag", node.tag());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
}

#[test]
fn _0004() {
  let input = ">z\n";
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  let node = root.children().next().unwrap();
  assert_eq!('>', node.delimiter());
  assert_eq!("z", node.name());
  assert_eq!("z", node.tag());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
}

#[test]
fn _0005() {
  let input = ".A1\n";
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  let node = root.children().next().unwrap();
  assert_eq!('.', node.delimiter());
  assert_eq!("A1", node.name());
  assert_eq!("a1", node.tag());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
}

#[test]
fn _0006() {
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
fn _0007() {
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
fn _0008() {
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
  assert_eq!("_", second.tag());
  assert_eq!("\n", second.content());
  assert_eq!("", second.text());
  assert!(children.next().is_none());
}

#[test]
fn _0009() {
  let input = ".A\n$B\n";
  let root = parse(input).unwrap();
  let mut children = root.children();
  let first = children.next().unwrap();
  assert_eq!('.', first.delimiter());
  assert_eq!("A", first.name());
  assert_eq!("\n$B\n", first.content());
  assert_eq!("$B", first.text());
  assert!(children.next().is_none());
}

#[test]
fn _0010() {
  let input = include_str!("input_0001.idml");
  let root = parse(input).unwrap();
  assert_eq!(input, root.document(4));
}
