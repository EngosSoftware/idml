use idml::{Token, join_tokens, tokenize};

#[test]
fn _0001() {
  // Valid root node name followed by line feed.
  let input = ".A\n";
  assert_eq!(
    vec![Token::Indentation(0), Token::NodeName("A".to_string(), '.'), Token::NodeContent("\n".to_string())],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0002() {
  // Valid root node name followed by carriage return.
  let input = ".A\r";
  assert_eq!(
    vec![Token::Indentation(0), Token::NodeName("A".to_string(), '.'), Token::NodeContent("\r".to_string())],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0003() {
  // Valid root node name followed by carriage return and line feed.
  let input = ".A\r\n";
  assert_eq!(
    vec![Token::Indentation(0), Token::NodeName("A".to_string(), '.'), Token::NodeContent("\r\n".to_string())],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0004() {
  // Valid root node name followed by newline.
  let input = ".node-name\n";
  assert_eq!(
    vec![Token::Indentation(0), Token::NodeName("node-name".to_string(), '.'), Token::NodeContent("\n".to_string())],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0005() {
  // Valid root node name followed by newline.
  let input = ".Mixed-casing_can-be_USED\n";
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("Mixed-casing_can-be_USED".to_string(), '.'),
      Token::NodeContent("\n".to_string())
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0006() {
  // Valid root node name followed by whitespace and newline.
  let input = ".A \n";
  assert_eq!(
    vec![Token::Indentation(0), Token::NodeName("A".to_string(), '.'), Token::NodeContent(" \n".to_string())],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0007() {
  // Valid root node name followed by whitespace and single line node content.
  let input = ".ID id\n";
  assert_eq!(
    vec![Token::Indentation(0), Token::NodeName("ID".to_string(), '.'), Token::NodeContent(" id\n".to_string())],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0008() {
  // Valid root node name followed by multiline node content with one line.
  let input = ".ID\nFirst line.\n";
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("ID".to_string(), '.'),
      Token::NodeContent("\nFirst line.\n".to_string())
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0009() {
  // Valid root node name followed by multiline node content with multiple lines.
  let input = ".ID\nFirst line.\nSecond line.\nThird line.\n";
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("ID".to_string(), '.'),
      Token::NodeContent("\nFirst line.\nSecond line.\nThird line.\n".to_string())
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0010() {
  // Valid root node name followed by multiline node content with multiple lines with indentations.
  let input = ".ID\n First line.\n  Second line.\n   Third line.\n";
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("ID".to_string(), '.'),
      Token::NodeContent("\n First line.\n  Second line.\n   Third line.\n".to_string())
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0011() {
  // Valid root node name followed by multiline node content with multiple
  // lines with indentations which starts directly after node name.
  let input = ".ID  First line.\n  Second line.\n   Third line.\n";
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("ID".to_string(), '.'),
      Token::NodeContent("  First line.\n  Second line.\n   Third line.\n".to_string())
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0012() {
  let input = ".MODEL\n.DECISION\n";
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("MODEL".to_string(), '.'),
      Token::NodeContent("\n".to_string()),
      Token::Indentation(0),
      Token::NodeName("DECISION".to_string(), '.'),
      Token::NodeContent("\n".to_string()),
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0013() {
  let input = r#".MODEL
    .NAMESPACE https://decision-toolkit.org/2_0001/
"#;
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("MODEL".to_string(), '.'),
      Token::NodeContent("\n".to_string()),
      Token::Indentation(4),
      Token::NodeName("NAMESPACE".to_string(), '.'),
      Token::NodeContent(" https://decision-toolkit.org/2_0001/\n".to_string())
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0014() {
  let input = r#".MODEL
    .NAMESPACE https://decision-toolkit.org/2_0001/
    .NAME 2_0001
"#;
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("MODEL".to_string(), '.'),
      Token::NodeContent("\n".to_string()),
      Token::Indentation(4),
      Token::NodeName("NAMESPACE".to_string(), '.'),
      Token::NodeContent(" https://decision-toolkit.org/2_0001/\n".to_string()),
      Token::Indentation(4),
      Token::NodeName("NAME".to_string(), '.'),
      Token::NodeContent(" 2_0001\n".to_string())
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0015() {
  let input = r#".MODEL
    Line 1
    Line 2

.DECISION Greeting Message
"#;
  assert_eq!(
    vec![
      Token::Indentation(0),
      Token::NodeName("MODEL".to_string(), '.'),
      Token::NodeContent("\n    Line 1\n    Line 2\n\n".to_string()),
      Token::Indentation(0),
      Token::NodeName("DECISION".to_string(), '.'),
      Token::NodeContent(" Greeting Message\n".to_string())
    ],
    tokenize(input).unwrap()
  );
}

#[test]
fn _0016() {
  let input = include_str!("input_0001.idml");
  let tokens = tokenize(input).unwrap();
  assert_eq!(input, join_tokens(tokens));
}
