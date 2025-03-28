use idml::{parse, Parser, Token};

#[test]
fn _0001() {
  // Empty input.
  let input = "";
  assert_eq!("empty input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0002() {
  // Input beginning with space(s) is invalid.
  let input = " ";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 1", parse(input).unwrap_err().to_string());
}

#[test]
fn _0003() {
  // Input beginning with tab(s) is invalid.
  let input = "\t";
  assert_eq!("unexpected character: '\t' 0x09 at row 1 and column 1", parse(input).unwrap_err().to_string());
}

#[test]
fn _0004() {
  // No node name, only delimiter present.
  let input = ".";
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0005() {
  // No node name, space after delimiter.
  let input = ". ";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 2", parse(input).unwrap_err().to_string());
}

#[test]
fn _0006() {
  // No node name, line feed after delimiter.
  let input = ".\n";
  assert_eq!("unexpected character: '\n' 0x0A at row 1 and column 2", parse(input).unwrap_err().to_string());
}

#[test]
fn _0007() {
  // No node name, carriage return after delimiter.
  let input = ".\r";
  assert_eq!("unexpected character: '\r' 0x0D at row 1 and column 2", parse(input).unwrap_err().to_string());
}

#[test]
fn _0008() {
  // No node name, carriage return and line feed after delimiter.
  let input = ".\r\n";
  assert_eq!("unexpected character: '\r' 0x0D at row 1 and column 2", parse(input).unwrap_err().to_string());
}

#[test]
fn _0009() {
  // Node name is not followed by a whitespace, newline or both.
  let input = ".A";
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0010() {
  // Node name is not followed by a whitespace and newline.
  let input = ".A ";
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0011() {
  // Root node name must be at the very beginning of the line, without any indentation.
  let input = " .A\n";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 1", parse(input).unwrap_err().to_string());
}

#[test]
fn _0012() {
  // Root node name must be at the very beginning of the line,
  // without any additional characters before it.
  let input = "n.A\n";
  assert_eq!("unexpected character: 'n' 0x6E at row 1 and column 1", parse(input).unwrap_err().to_string());
}

#[test]
fn _0013() {
  // Node name must start with a single dot.
  let input = "..A\n";
  assert_eq!("unexpected character: '.' 0x2E at row 1 and column 2", parse(input).unwrap_err().to_string());
}

#[test]
fn _0014() {
  // Node name must end with whitespace or newline or whitespace and newline.
  let input = ".A!\n";
  assert_eq!("unexpected character: '!' 0x21 at row 1 and column 3", parse(input).unwrap_err().to_string());
}

#[test]
fn _0015() {
  // After delimiter there must be a node name start.
  let input = ".A\n   .!\n";
  assert_eq!("unexpected character: '!' 0x21 at row 2 and column 5", parse(input).unwrap_err().to_string());
}

#[test]
fn _0016() {
  // Spaces after the last newline character.
  let input = r#".MODEL
    .NAMESPACE https://decision-toolkit.org/2_0001/
  "#;
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0017() {
  // Malformed indentation
  let input = r#".A
    .B
    .C
   .D
"#;
  assert_eq!("malformed indentation 3, expected multiplication of 4", parse(input).unwrap_err().to_string());
}

#[test]
fn _0018() {
  // Malformed indentation
  let input = r#".A
        .B
    .C
    .D
"#;
  assert_eq!("malformed indentation 4, expected multiplication of 8", parse(input).unwrap_err().to_string());
}

#[test]
fn _0019() {
  // No indentation token.
  let tokens = vec![Token::NodeName("A".to_string(), '.')];
  assert_eq!("expected indentation token", Parser::new(tokens).parse().unwrap_err().to_string())
}

#[test]
fn _0020() {
  // No node name token.
  let tokens = vec![Token::Indentation(0), Token::NodeContent("content".to_string())];
  assert_eq!("expected node name token", Parser::new(tokens).parse().unwrap_err().to_string())
}

#[test]
fn _0021() {
  // No node content token.
  let tokens = vec![Token::Indentation(0), Token::NodeName("name".to_string(), '.'), Token::Indentation(0)];
  assert_eq!("expected node content token", Parser::new(tokens).parse().unwrap_err().to_string())
}
