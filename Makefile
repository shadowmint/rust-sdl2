RUSTFLAGS ?=
OUTDIR ?= ./build

BINDIR = $(OUTDIR)/bin
LIBDIR = $(OUTDIR)/lib
TMPDIR = $(OUTDIR)/tmp

RUST_SRC = $(shell find src/. -type f -name '*.rs') \
	src/sdl2/generated/keycode.rs                   \
	src/sdl2/generated/scancode.rs

.PHONY: all
all: $(TMPDIR)/libsdl2.dummy

UNAME=$(shell uname)

$(BINDIR) $(LIBDIR) $(TMPDIR):
	mkdir -p '$@'

$(TMPDIR)/codegen: $(wildcard src/codegen/*.rs) $(TMPDIR)
	rustc -o '$(TMPDIR)/codegen' src/codegen/main.rs $(RUSTFLAGS)

src/sdl2/generated/%.rs: $(TMPDIR)/codegen
	'$(TMPDIR)/codegen' $(patsubst src/sdl2/generated/%,%,$@) src/sdl2/generated/

$(TMPDIR)/libsdl2.dummy: src/sdl2/lib.rs $(RUST_SRC) $(LIBDIR) $(TMPDIR)
	rustc --crate-type=lib src/sdl2/lib.rs -L ../sdl2/build/lib --out-dir ./build/
	touch $@

compile_demo: src/demo/main.rs src/demo/video.rs $(TMPDIR)/libsdl2.dummy $(BINDIR)
	rustc -o '$(BINDIR)/demo' -L '$(LIBDIR)' src/demo/main.rs

demo: compile_demo
	'$(BINDIR)/demo'

.PHONY: clean
clean:
	rm -rf src/sdl2/generated
	rm -rf '$(OUTDIR)'
