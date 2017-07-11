extern crate ignore;


use std::path::Path;

use ignore::gitignore::GitignoreBuilder;


#[test]
fn test_empty_rule() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_none());
    assert!(m("ROOT/parent_dir/file").is_none());
    assert!(m("ROOT/parent_dir/child_dir/file").is_none());
}

#[test]
fn test_empty_slash() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "/").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_ignore());
    assert!(m("ROOT/parent_dir/file").is_ignore());
    assert!(m("ROOT/parent_dir/child_dir/file").is_ignore());
}

#[test]
fn test_empty_double_slash() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "//").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_none());
    assert!(m("ROOT/parent_dir/file").is_none());
    assert!(m("ROOT/parent_dir/child_dir/file").is_none());
}

#[test]
fn test_empty_triple_slash() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "///").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_none());
    assert!(m("ROOT/parent_dir/file").is_none());
    assert!(m("ROOT/parent_dir/child_dir/file").is_none());
}

#[test]
fn test_empty_negate() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "!").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_whitelist());
    assert!(m("ROOT/parent_dir/file").is_whitelist());
    assert!(m("ROOT/parent_dir/child_dir/file").is_whitelist());
}

#[test]
fn test_empty_double_negate() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "!!").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_none());
    assert!(m("ROOT/parent_dir/file").is_none());
    assert!(m("ROOT/parent_dir/child_dir/file").is_none());
}

#[test]
fn test_empty_asterisk() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "*").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_ignore());
    assert!(m("ROOT/parent_dir/file").is_ignore());
    assert!(m("ROOT/parent_dir/child_dir/file").is_ignore());
}

#[test]
fn test_empty_double_asterisk() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "**").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_none());
    assert!(m("ROOT/parent_dir/file").is_none());
    assert!(m("ROOT/parent_dir/child_dir/file").is_none());
}

#[test]
fn test_empty_triple_asterisk() {
    let mut builder = GitignoreBuilder::new("ROOT");

    assert!(builder.add_line(None, "***").is_err())
}

#[test]
fn test_empty_question_mark() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "?").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_none());
    assert!(m("ROOT/parent_dir/file").is_none());
    assert!(m("ROOT/parent_dir/child_dir/file").is_none());
}

#[test]
fn test_empty_double_question_mark() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "??").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_none());
    assert!(m("ROOT/parent_dir/file").is_none());
    assert!(m("ROOT/parent_dir/child_dir/file").is_none());
}

#[test]
fn test_empty_triple_question_mark() {
    let mut builder = GitignoreBuilder::new("ROOT");

    builder.add_line(None, "???").unwrap();

    let gitignore = builder.build().unwrap();
    let m = |path: &str| gitignore.matched_recursive(Path::new(path), false);

    assert!(m("ROOT/file").is_none());
    assert!(m("ROOT/parent_dir/file").is_none());
    assert!(m("ROOT/parent_dir/child_dir/file").is_none());
}
