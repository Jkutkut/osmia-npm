use osmia::{Osmia, CodeInterpreter};
use crate::{run, run_ctx, VERSION};
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
	err,
	(fail_ctx, "{{usr.name}}"),
	(fail_code01, "{{if }}"),
	(fail_code02, "{{if true}}foo")
);
