use std::path::PathBuf;

fn main() {
	println!("cargo:rustc-link-lib=wxWidgets");
	println!("cargo:rerun-if-changed=wrapper.hpp");

	let cxx_flags = String::from("-std=c++14");

	// It's apparently impossible to define this programmatically, so I have to
	// abuse this environment variable meant to be used by the user!
	std::env::set_var(
		"BINDGEN_EXTRA_CLANG_ARGS",
		std::env::var("BINDGEN_EXTRA_CLANG_ARGS")
			.map(|str| str + " " + cxx_flags.as_ref())
			.unwrap_or(cxx_flags)
	);

	std::env::set_var("RUST_LOG", "bindgen");
	env_logger::init();

	let bindings = bindgen::Builder::default()
		.header("wrapper.hpp")
		.detect_include_paths(false)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Couldn't make bindings!");

	let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}