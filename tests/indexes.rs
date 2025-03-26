use idml::parse;

#[test]
fn _0001() {
  let input = r#".A
    .B
    .C
    .D
"#;
  let root = parse(input).unwrap();
  assert_eq!("A", root.first("A").unwrap().name());
  assert_eq!("B", root.first("A").unwrap().first("B").unwrap().name());
}

#[test]
fn _0002() {
  let input = r#".A
    .B
    .C
    .D
"#;
  let root = parse(input).unwrap();
  assert_eq!("A", root.last("A").unwrap().name());
  assert_eq!("D", root.last("A").unwrap().last("D").unwrap().name());
}
