use idml::tokenize;

#[test]
fn _0001() {
  // Empty input is invalid.
  let input = "";
  assert_eq!("empty input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0002() {
  // Unexpected end of input, usually there should be another '/' after the first one
  // if a single line comment was the intention.
  let input = "/";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0003() {
  // Unexpected character on input.
  let input = "/a";
  assert_eq!("unexpected character: 'a' 0x61 at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0004() {
  // Single line comment SHOULD always end with a newline character.
  let input = "//";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0005() {
  // Single line comment SHOULD always end with a newline character.
  let input = "//comment";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0006() {
  // Single line comment SHOULD always end with a newline character.
  let input = "// invalid comment without newline at the end";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0007() {
  // No root node after the proper comment.
  let input = "// Comment\n";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0008() {
  // Not closed multiline line comment.
  let input = "/*";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0009() {
  // Not closed multiline line comment.
  let input = "/* comment *";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0010() {
  // No root node after the comment
  let input = "/* comment */";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0011() {
  // Not closed multiline line comment.
  let input = "/* comment */";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0012() {
  // No node name, only delimiter present.
  let input = ".";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0013() {
  // No node name, space after delimiter.
  let input = ". ";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0014() {
  // No node name, line feed after delimiter.
  let input = ".\n";
  assert_eq!("unexpected character: '\n' 0xA at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0015() {
  // No node name, carriage return after delimiter.
  let input = ".\r";
  assert_eq!("unexpected character: '\r' 0xD at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0016() {
  // No node name, carriage return and line feed after delimiter.
  let input = ".\r\n";
  assert_eq!("unexpected character: '\r' 0xD at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0017() {
  // Node name is not followed by a whitespace, newline or both.
  let input = ".A";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0018() {
  // Node name is not followed by a whitespace and newline.
  let input = ".A ";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0019() {
  // Root node name must be at the very beginning of the line, without any indentation.
  let input = " .A\n";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 1", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0020() {
  // Root node name must be at the very beginning of the line,
  // without any additional characters before it.
  let input = "n.A\n";
  assert_eq!("unexpected character: 'n' 0x6E at row 1 and column 1", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0021() {
  // Node name must start with a single dot.
  let input = "..A\n";
  assert_eq!("unexpected character: '.' 0x2E at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0022() {
  // Node name must end with whitespace or newline or whitespace and newline.
  let input = ".A!\n";
  assert_eq!("unexpected character: '!' 0x21 at row 1 and column 3", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0023() {
  // After delimiter there must be a node name start.
  let input = ".A\n   .!\n";
  assert_eq!("unexpected character: '!' 0x21 at row 2 and column 5", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0024() {
  // Spaces after the last newline character.
  let input = r#".MODEL
    .NAMESPACE https://decision-toolkit.org/2_0001/
  "#;
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}
