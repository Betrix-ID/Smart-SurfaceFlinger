TARGET = smart_SurfaceFlinger  
SRC = src/main.rs
CARGO = cargo

all: banner build

build:
	@echo ">> Compiling Rust program..."
	$(CARGO) build
	@echo ""
	@echo ">> Build finished successfully!"

# Menjalankan program
run:
	@echo ">> Running the program..."
	./target/release/$(TARGET)

# Banner tampilan keren
banner:
	@echo
	@echo
	@echo "     ☆=====================================☆"
	@echo
	@echo "      ~ Description Compile Rust Program"
	@echo
	@echo "      - Author             :  @UnixeID"
	@echo "      - Version            :  1.0"
	@echo "      - Release            :  5 - Mei - 2025"
	@echo "      - Project Name       :  Rust Compiler"
	@echo
	@echo "    |_______________________________________|"
	@echo "     \\______________________________________/"
	@echo
	@echo
	@sleep 1

# Clean build files
clean:
	@echo ">> Cleaned build files."
	@rm -rf Cargo.lock  
	@find target/release -type f ! -name 'smart_SurfaceFlinger' -delete
	@find target/debug -type f ! -name 'smart_SurfaceFlinger' -delete
	@find target -type d -empty -delete

# Format source code
format:
	@echo ">> Formatting Rust source code..."
	cargo fmt
	@echo ">> Source code formatted."