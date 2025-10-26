.PHONY: install dev run backend frontend build deploy clean

install:
	@if command -v mise > /dev/null 2>&1; then \
		echo "✓ mise is already installed"; \
		mise --version; \
	else \
		echo "Installing mise..."; \
		curl https://mise.run | sh; \
		echo "✓ mise installed successfully"; \
		echo "Run 'mise install' to install project tools"; \
	fi

dev: run

run:
	@make -j2 backend frontend

backend:
	@cd backend && cargo run

frontend:
	@cd frontend && bun run dev

build:
	@cd backend && cargo build --release
	@cd frontend && bun run build

deploy: build
	@echo "Backend binary: ./backend/target/release/death-oji-backend"
	@echo "Frontend dist: ./frontend/dist/"

clean:
	@cd backend && cargo clean
	@cd frontend && rm -rf dist node_modules
