CARGO = cargo

BIN_NAME = scop

SRC_FILES = src/main.rs \
            src/compile_shaders.rs \
            src/globals.rs \
            src/init_opengl.rs \
            src/obj_parser.rs \
            src/render.rs \
            src/texture_loader.rs \
            src/models/gl_var.rs \
            src/models/mat4.rs \
            src/models/mod.rs \
            src/models/obj_data.rs \
            src/models/vec3.rs


all: $(BIN_NAME)

$(BIN_NAME): $(SRC_FILES) Cargo.toml
	$(CARGO) build --release
	cp target/release/$(BIN_NAME) $@

clean:
	$(CARGO) clean

fclean: clean
	rm -f $(BIN_NAME)

re: fclean all

.PHONY: all clean fclean re
