# Install mise (platform-specific)
install:
    #!/usr/bin/env bash
    if command -v mise &> /dev/null; then \
        echo "✓ mise is already installed"; \
        mise --version; \
    elif [[ "$OSTYPE" == "darwin"* ]]; then \
        echo "Installing mise via Homebrew..."; \
        brew install mise; \
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then \
        echo "Installing mise via curl..."; \
        curl https://mise.run | sh; \
    else \
        echo "Please install mise manually from https://github.com/jdx/mise"; \
        exit 1; \
    fi && \
    echo "✓ mise installed successfully" && \
    echo "Run 'mise install' to install project tools"

# Run both backend and frontend in development mode
dev: run

# Alias for dev
run:
    cargo run --manifest-path backend/Cargo.toml & bun run --cwd frontend dev

# Run backend only
backend:
    cargo run --manifest-path backend/Cargo.toml

# Run frontend only
frontend:
    bun run --cwd frontend dev

# Build for production
build:
    cargo build --release --manifest-path backend/Cargo.toml
    bun run --cwd frontend build

# Deploy (build and show output locations)
deploy: build
    @echo "Backend binary: ./backend/target/release/death-oji-backend"
    @echo "Frontend dist: ./frontend/dist/"

# Clean build artifacts
clean:
    cargo clean --manifest-path backend/Cargo.toml
    rm -rf frontend/dist frontend/node_modules
