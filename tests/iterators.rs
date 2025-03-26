use idml::{Node, parse};

fn get_children_by_name(node: &Node, name: &str) -> Vec<String> {
  node.children_by_name(name).map(|node| node.name().to_string()).collect::<Vec<String>>()
}

fn get_children_with_names(node: &Node, names: &[&str]) -> Vec<String> {
  node.children_with_names(names).map(|node| node.name().to_string()).collect::<Vec<String>>()
}

fn get_children_by_tag(node: &Node, tag: &str) -> Vec<String> {
  node.children_by_tag(tag).map(|node| node.tag().to_string()).collect::<Vec<String>>()
}

fn get_children_with_tags(node: &Node, tags: &[&str]) -> Vec<String> {
  node.children_with_tags(tags).map(|node| node.tag().to_string()).collect::<Vec<String>>()
}

fn get_children_except_name(node: &Node, name: &str) -> Vec<String> {
  node.children_except_name(name).map(|node| node.name().to_string()).collect::<Vec<String>>()
}

fn get_children_except_tag(node: &Node, tag: &str) -> Vec<String> {
  node.children_except_tag(tag).map(|node| node.tag().to_string()).collect::<Vec<String>>()
}

fn get_children_except_names(node: &Node, names: &[&str]) -> Vec<String> {
  node.children_except_names(names).map(|node| node.name().to_string()).collect::<Vec<String>>()
}

fn get_children_except_tags(node: &Node, tags: &[&str]) -> Vec<String> {
  node.children_except_tags(tags).map(|node| node.tag().to_string()).collect::<Vec<String>>()
}

#[test]
fn _0001() {
  let input = r#".A
    .B
    .B
    .C
    .D
    .D
    .D
"#;
  let root = parse(input).unwrap();
  assert_eq!(1, root.children().count());
  assert_eq!(1, root.child_count());
  let node = root.children().next().unwrap();
  assert_eq!(6, node.children().count());
  assert_eq!(6, node.child_count());
  assert_eq!('.', node.delimiter());
  assert_eq!("A", node.name());
  assert_eq!("a", node.tag());
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
  assert_eq!(vec!["B", "B"], get_children_by_name(node, "B"));
  assert_eq!(vec!["C"], get_children_by_name(node, "C"));
  assert_eq!(vec!["D", "D", "D"], get_children_by_name(node, "D"));
  assert!(get_children_by_name(node, "E").is_empty());
}

#[test]
fn _0002() {
  let input = r#".A
    .B
    .B
    .C
    .D
    .D
    .D
"#;
  let root = parse(input).unwrap();
  let node = root.children().next().unwrap();
  assert_eq!(vec!["b", "b"], get_children_by_tag(node, "b"));
  assert_eq!(vec!["c"], get_children_by_tag(node, "c"));
  assert_eq!(vec!["d", "d", "d"], get_children_by_tag(node, "d"));
  assert!(get_children_by_tag(node, "e").is_empty());
}

#[test]
fn _0003() {
  let input = r#".A
    .B
    .B
    .C
    .D
    .D
    .D
"#;
  let root = parse(input).unwrap();
  let node = root.children().next().unwrap();
  assert_eq!(vec!["C", "D", "D", "D"], get_children_except_name(node, "B"));
  assert_eq!(vec!["B", "B", "D", "D", "D"], get_children_except_name(node, "C"));
  assert_eq!(vec!["B", "B", "C"], get_children_except_name(node, "D"));
}

#[test]
fn _0004() {
  let input = r#".A
    .B
    .B
    .C
    .D
    .D
    .D
"#;
  let root = parse(input).unwrap();
  let node = root.children().next().unwrap();
  assert_eq!(vec!["c", "d", "d", "d"], get_children_except_tag(node, "b"));
  assert_eq!(vec!["b", "b", "d", "d", "d"], get_children_except_tag(node, "c"));
  assert_eq!(vec!["b", "b", "c"], get_children_except_tag(node, "d"));
}

#[test]
fn _0005() {
  let input = r#".A
    .B
    .B
    .C
    .D
    .D
    .D
"#;
  let root = parse(input).unwrap();
  let node = root.children().next().unwrap();
  assert_eq!(vec!["B", "B"], get_children_with_names(node, &["B"]));
  assert_eq!(vec!["B", "B", "D", "D", "D"], get_children_with_names(node, &["B", "D"]));
  assert_eq!(vec!["B", "B", "C"], get_children_with_names(node, &["C", "B"]));
}

#[test]
fn _0006() {
  let input = r#".A
    .B
    .B
    .C
    .D
    .D
    .D
"#;
  let root = parse(input).unwrap();
  let node = root.children().next().unwrap();
  assert_eq!(vec!["b", "b"], get_children_with_tags(node, &["b"]));
  assert_eq!(vec!["b", "b", "d", "d", "d"], get_children_with_tags(node, &["d", "b"]));
  assert_eq!(vec!["b", "b", "c"], get_children_with_tags(node, &["c", "b"]));
}

#[test]
fn _0007() {
  let input = r#".A
    .B
    .B
    .C
    .D
    .D
    .D
"#;
  let root = parse(input).unwrap();
  let node = root.children().next().unwrap();
  assert_eq!(vec!["C", "D", "D", "D"], get_children_except_names(node, &["B"]));
  assert_eq!(vec!["C"], get_children_except_names(node, &["B", "D"]));
  assert_eq!(vec!["D", "D", "D"], get_children_except_names(node, &["C", "B"]));
}

#[test]
fn _0008() {
  let input = r#".A
    .B
    .B
    .C
    .D
    .D
    .D
"#;
  let root = parse(input).unwrap();
  let node = root.children().next().unwrap();
  assert_eq!(vec!["c", "d", "d", "d"], get_children_except_tags(node, &["b"]));
  assert_eq!(vec!["c"], get_children_except_tags(node, &["b", "d"]));
  assert_eq!(vec!["d", "d", "d"], get_children_except_tags(node, &["c", "b"]));
}
