godot4 = $(if $(GODOT4_BIN),$(GODOT4_BIN),godot4)

build:
	cargo build
	$(godot4) --import --headless

edit:
	$(godot4) --editor

format:
	cargo fmt

run:
	$(godot4)

clean:
	git clean -Xdf
