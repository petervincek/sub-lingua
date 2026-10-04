# Define the target: build (will download the dependencies and compile the project)
build:
	PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:/opt/homebrew/opt/mpv/lib/pkgconfig:$$PKG_CONFIG_PATH" \
	LIBRARY_PATH="/opt/homebrew/lib:$$LIBRARY_PATH" \
	cargo build

# Define the target: build-prod
build-prod:
	PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:/opt/homebrew/opt/mpv/lib/pkgconfig:$$PKG_CONFIG_PATH" \
	LIBRARY_PATH="/opt/homebrew/lib:$$LIBRARY_PATH" \
	cargo build --release

# Define the target: clean
clean:
	cargo clean

# Define the target: update
update:
	cargo update

# Define the target: test
test:
	PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:/opt/homebrew/opt/mpv/lib/pkgconfig:$$PKG_CONFIG_PATH" \
	LIBRARY_PATH="/opt/homebrew/lib:$$LIBRARY_PATH" \
	cargo test

# Define the target: coverage
coverage:
	cargo llvm-cov --workspace --html

# Define the target: coverage-lcov
coverage-lcov:
	cargo llvm-cov --workspace --lcov --output-path lcov.info

# Define the target: check
check:
	PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:/opt/homebrew/opt/mpv/lib/pkgconfig:$$PKG_CONFIG_PATH" \
	LIBRARY_PATH="/opt/homebrew/lib:$$LIBRARY_PATH" \
	cargo check

# Define the target: lint
lint:
	cargo clippy

# Define the target: lint-fix
lint-fix:
	cargo clippy --fix

# Define the target: run
run:
	PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:/opt/homebrew/opt/mpv/lib/pkgconfig:$$PKG_CONFIG_PATH" \
	LIBRARY_PATH="/opt/homebrew/lib:$$LIBRARY_PATH" \
	cargo run

# Define the target: run-release (will compile while eliminating the debug statements)
run-release:
	PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:/opt/homebrew/opt/mpv/lib/pkgconfig:$$PKG_CONFIG_PATH" \
	LIBRARY_PATH="/opt/homebrew/lib:$$LIBRARY_PATH" \
	cargo run --release

# Define the target: cloc
cloc:
	cloc --exclude-dir=target  .