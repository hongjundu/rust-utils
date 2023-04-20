
# rustutils for C

## Reference

https://stackoverflow.com/questions/65229930/is-it-possible-to-call-a-function-from-static-library-that-built-in-rust-using-c

## Steps

* cargo new rustutils --lib

* Cargo.toml

	```
	[lib]
	crate-type = ["cdylib"]      # dynamic lib
	# crate-type = ["staticlib"] # static lib
	```

* Add a C API

	```
	#[no_mangle]
	pub extern "C" fn message() {
	  println!("Hello C!")
	}
	```

* Build
	```
	cargo build --release
	```

* Generate C header file

	```
	cargo install cbindgen
	touch cbindgen.toml
	cbindgen --config cbindgen.toml --crate rustutils  --output rustutils.h --lang c
	```

* Compile C example

    Move rustutils.h and librustutils.a to examples folder

    ```
    cd example
    gcc main.c -L. -lrustutils -o main
    ```