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
