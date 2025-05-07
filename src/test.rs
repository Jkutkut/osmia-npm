use osmia::{Osmia, CodeInterpreter};
use crate::{run, run_ctx, VERSION};

fn rok_eq(code: &str, expected: &str) {
	println!("Running code (should be ok):\n{}\nexpected:\n{}", code, expected);
	let r = run(code);
	ok(&r);
	assert_eq!(r.unwrap(), expected);
}

fn rctx_eq(json: &str, code: &str, expected: &str) {
	println!("Running code (should be ok):\n{}\ncontext:\n{}\nexpected:\n{}", code, json, expected);
	let r = run_ctx(json, code);
	ok(&r);
	assert_eq!(r.unwrap(), expected);
}

fn ok(result: &Result<String, String>) {
	println!("{:?}", result);
	assert!(result.is_ok());
}

fn err(result: &Result<String, String>) {
	println!("{:?}", result);
	assert!(result.is_err());
}

#[test]
fn osmia_version() {rok_eq("{{ _OSMIA_VERSION }}", Osmia::VERSION)}

#[test]
fn osmia_npm_version() {rok_eq("{{ _OSMIA_NPM_VERSION }}", VERSION)}

#[test]
fn json() {rctx_eq(r#"{"usr":{"name":"Marvin"}}"#, "{{usr.name}}", "Marvin")}

#[test]
fn fail_ctx() {err(&run("{{usr.name}}"))}

#[test]
fn fail_code01() {err(&run("{{if }}"))}

#[test]
fn fail_code02() {err(&run("{{if true}}foo"))}
