use idml::parse;

#[test]
fn _0001() {
  let input = r#".A
    .B
    .C
    .D
"#;
  let root = parse(input).unwrap();
  assert_eq!("A", root.first_by_name("A").unwrap().name());
  assert_eq!("B", root.first_by_name("A").unwrap().first_by_name("B").unwrap().name());
}

#[test]
fn _0002() {
  let input = r#".A
    .B
    .C
    .D
"#;
  let root = parse(input).unwrap();
  assert_eq!("a", root.first_by_tag("a").unwrap().tag());
  assert_eq!("b", root.first_by_tag("a").unwrap().first_by_name("B").unwrap().tag());
}

#[test]
fn _0003() {
  let input = r#".A
    .B hello
    .C
    .D
"#;
  let root = parse(input).unwrap();
  assert_eq!("a", root.first_by_tag("a").unwrap().tag());
  assert_eq!("hello", root.first_by_name("A").unwrap().first_by_tag("b").unwrap().text());
}
