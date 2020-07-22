use std::path::PathBuf;

fn main() {
	println!("cargo:rerun-if-changed=wrapper.hpp");

	std::env::set_var("RUST_LOG", "bindgen");
	env_logger::init();

	let bindings = bindgen::Builder::default()
		.header("wrapper.hpp")
		.detect_include_paths(false)
		.clang_arg("-std=c++14")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Couldn't make bindings!");

	let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}