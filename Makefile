# stego-sign — Makefile

# -- environment variables
include .env
export

# -- config
DB_CONTAINER     ?= stego-db
DB_USER          ?= $(POSTGRES_USER)
DB_NAME          ?= $(POSTGRES_DB)
SERVER_CONTAINER ?= stego-server
APP_CONTAINER    ?= stego-app

# -- detect if DB is running in Docker or local
# -- if DB_CONTAINER is running, use docker exec; otherwise use psql directly
DB_RUNNING := $(shell docker ps --format '{{.Names}}' 2>/dev/null | grep -w $(DB_CONTAINER))

ifeq ($(DB_RUNNING),$(DB_CONTAINER))
  PSQL = docker exec -i $(DB_CONTAINER) psql -U $(DB_USER) -d $(DB_NAME)
else
  PSQL = psql "$(DATABASE_URL)"
endif

MIGRATIONS_DIR = server/migrations

# -- help
.DEFAULT_GOAL := help

help: ## show available commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | while IFS= read -r line; do \
		target=$$(echo "$$line" | cut -d: -f1); \
		desc=$$(echo "$$line" | sed 's/^[^#]*## //'); \
		printf "  \033[36m%-20s\033[0m %s\n" "$$target" "$$desc"; \
	done

# -- docker compose
up: ## start all services
	docker compose up -d

up-infra: ## start only db and aistor (for local dev)
	docker compose up -d db aistor

down: ## stop all services
	docker compose down

down-v: ## stop all services and remove volumes
	docker compose down -v

restart: ## restart all services
	docker compose restart

build: ## build all docker images
	docker compose build

build-server: ## build server image only
	docker compose build server

build-app: ## build app image only
	docker compose build app

logs: ## follow logs (usage: make logs s=server)
	docker compose logs -f $(s)

ps: ## show running containers
	docker compose ps

# -- deploy profiles
deploy-aistor: ## deploy with local aistor storage (no aws s3)
	STORAGE_PROVIDER=aistor \
	STORAGE_ENDPOINT=http://aistor:9000 \
	docker compose up -d db aistor server app

deploy-aws: ## deploy with aws s3 storage (no aistor)
	STORAGE_PROVIDER=aws \
	STORAGE_ENDPOINT="" \
	docker compose up -d --scale aistor=0 db server app

recreate-server: ## force recreate server container (picks up new env vars)
	docker compose up -d --force-recreate server

# -- database migrations
migrate: ## apply all schemas (files first, then app)
	@echo "→ applying schema_files..."
	@$(PSQL) < $(MIGRATIONS_DIR)/schema_files.sql
	@echo "→ applying schema_app..."
	@$(PSQL) < $(MIGRATIONS_DIR)/schema_app.sql
	@echo "✓ migrations applied"

migrate-reset: ## reset all data (keeps schema)
	@echo "→ resetting schema_app data..."
	@$(PSQL) < $(MIGRATIONS_DIR)/reset_schema_app.sql
	@echo "→ resetting schema_files data..."
	@$(PSQL) < $(MIGRATIONS_DIR)/reset_schema_files.sql
	@echo "✓ data reset"

migrate-drop: ## drop all schemas completely
	@echo "→ dropping schema_app..."
	@$(PSQL) < $(MIGRATIONS_DIR)/delete_schema_app.sql
	@echo "→ dropping schema_files..."
	@$(PSQL) < $(MIGRATIONS_DIR)/delete_schema_files.sql
	@echo "✓ schemas dropped"

migrate-fresh: migrate-drop migrate ## drop and re-apply all schemas

# -- local development
dev-server: ## run server locally (requires db + aistor running)
	cd server && cargo run

dev-app: ## run app locally (requires server running)
	cd app && cargo leptos watch

dev: up-infra ## start infra + print next steps
	@echo "→ infrastructure ready"
	@echo "→ run 'make dev-server' and 'make dev-app' in separate terminals"

# -- keys
keygen: ## generate ed25519 key pair and print to stdout
	@echo "→ generating keys..."
	@curl -s http://localhost:$(APP_PORT)/api/v1/admin/keygen | python3 -m json.tool

# -- cleanup
clean: ## remove build artifacts
	cd server && cargo clean
	cd app && cargo clean

prune: ## remove unused docker resources
	docker system prune -f