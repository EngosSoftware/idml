use idml::{Node, parse};

fn get_with_name(node: &Node, name: &str) -> Vec<String> {
  node.with_name(name).map(|node| node.name().to_string()).collect::<Vec<String>>()
}

fn get_with_names(node: &Node, names: &[&str]) -> Vec<String> {
  node.with_names(names).map(|node| node.name().to_string()).collect::<Vec<String>>()
}

fn get_except_name(node: &Node, name: &str) -> Vec<String> {
  node.except_name(name).map(|node| node.name().to_string()).collect::<Vec<String>>()
}

fn get_except_names(node: &Node, names: &[&str]) -> Vec<String> {
  node.except_names(names).map(|node| node.name().to_string()).collect::<Vec<String>>()
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
  assert_eq!("\n", node.content());
  assert_eq!("", node.text());
  assert_eq!(vec!["B", "B"], get_with_name(node, "B"));
  assert_eq!(vec!["C"], get_with_name(node, "C"));
  assert_eq!(vec!["D", "D", "D"], get_with_name(node, "D"));
  assert!(get_with_name(node, "E").is_empty());
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
  assert_eq!(vec!["C", "D", "D", "D"], get_except_name(node, "B"));
  assert_eq!(vec!["B", "B", "D", "D", "D"], get_except_name(node, "C"));
  assert_eq!(vec!["B", "B", "C"], get_except_name(node, "D"));
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
  assert_eq!(vec!["B", "B"], get_with_names(node, &["B"]));
  assert_eq!(vec!["B", "B", "D", "D", "D"], get_with_names(node, &["B", "D"]));
  assert_eq!(vec!["B", "B", "C"], get_with_names(node, &["C", "B"]));
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
  assert_eq!(vec!["C", "D", "D", "D"], get_except_names(node, &["B"]));
  assert_eq!(vec!["C"], get_except_names(node, &["B", "D"]));
  assert_eq!(vec!["D", "D", "D"], get_except_names(node, &["C", "B"]));
}
