# A pile of hacks to make `bindgen!` actually work without crashing

DEPS = ../../target/deps
CLANG = $(shell ldd $(DEPS)/libbindgen*.so | grep clang | cut -d' ' -f3)

all:
	LD_PRELOAD=$(CLANG) rustc -L $(DEPS) --pretty=expanded bindgen.rs | sed -e '1,/CUT/d' > ffi.rs
