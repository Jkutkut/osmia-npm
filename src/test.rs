use osmia::{Osmia, CodeInterpreter};
use crate::{run, run_ctx, run_json_ctx, run_yaml_ctx, VERSION};
use macro_test::macro_tests;

fn rok_eq(code: &str, expected: &str) {
	println!("Running code (should be ok):\n{}\nexpected:\n{}", code, expected);
	let r = run(code);
	assert!(r.is_ok());
	assert_eq!(r.unwrap(), expected);
}

fn rctx_eq(json: &str, code: &str, expected: &str) {
	println!("Running code (should be ok):\n{}\ncontext:\n{}\nexpected:\n{}", code, json, expected);
	let r = run_ctx(json, code);
	assert!(r.is_ok());
	assert_eq!(r.unwrap(), expected);
	let r = run_json_ctx(json, code);
	assert!(r.is_ok());
	assert_eq!(r.unwrap(), expected);
}

fn rctx_yaml_eq(yaml: &str, code: &str, expected: &str) {
	println!("Running code (should be ok):\n{}\ncontext:\n{}\nexpected:\n{}", code, yaml, expected);
	let r = run_yaml_ctx(yaml, code);
	assert!(r.is_ok());
	assert_eq!(r.unwrap(), expected);
}

fn err(code: &str) {
	println!("Running code (should fail):\n{}", code);
	let r = run(code);
	assert!(r.is_err());
}

macro_tests!(
	rok_eq,
	(osmia_version, "{{ _OSMIA_VERSION }}", Osmia::VERSION),
	(osmia_npm_version, "{{ _OSMIA_NPM_VERSION }}", VERSION),
);

macro_tests!(
	rctx_eq,
	(json, r#"{"usr":{"name":"Marvin"}}"#, "{{usr.name}}", "Marvin")
);

macro_tests!(
	rctx_yaml_eq,
	(yaml, "usr:\n  name: Marvin", "{{usr.name}}", "Marvin")
);

macro_tests!(
	err,
	(fail_ctx, "{{usr.name}}"),
	(fail_code01, "{{if }}"),
	(fail_code02, "{{if true}}foo")
);

#[test]
fn ctx_json_dump() {
	assert!(crate::ctx_json_dump(None).is_ok());
}

#[test]
fn ctx_yaml_dump() {
	assert!(crate::ctx_yaml_dump(None).is_ok());
}

#[test]
fn ctx_json_dump_with_ctx() {
	let ctx = r#"{"usr":{"name":"Marvin"}}"#;
	let dump = crate::ctx_json_dump(Some(ctx.to_string()));
	assert!(dump.is_ok());
	let dump = dump.unwrap();
	println!("{}", dump);
	assert!(dump.contains(r#""usr":{"#));
	assert!(dump.contains(r#""value":"Marvin""#));
}

#[test]
fn ctx_yaml_dump_with_ctx() {
	let ctx = "usr:\n  name: Marvin";
	let dump = crate::ctx_yaml_dump(Some(ctx.to_string()));
	assert!(dump.is_ok());
	let dump = dump.unwrap();
	println!("{}", dump);
	assert!(dump.contains(r#""usr":{"#));
	assert!(dump.contains(r#""value":"Marvin""#));
}

#[test]
fn ctx_json_dump_variable() {
	let ctx = r#"{"usr":{"name":"Marvin"}}"#;
	let dump = crate::ctx_json_dump_variable("usr.name", Some(ctx.to_string()));
	assert!(dump.is_ok());
	let dump = dump.unwrap();
	println!("{}", dump);
	assert!(dump.contains(r#""value":"Marvin""#));
}

#[test]
fn ctx_yaml_dump_variable() {
	let ctx = "usr:\n  name: Marvin";
	let dump = crate::ctx_yaml_dump_variable("usr.name", Some(ctx.to_string()));
	assert!(dump.is_ok());
	let dump = dump.unwrap();
	println!("{}", dump);
	assert!(dump.contains(r#""value":"Marvin""#));
}
