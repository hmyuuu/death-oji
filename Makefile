.PHONY: dev run backend frontend build deploy clean

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
