.PHONY: dev build release test clean watch fmt lint

redis:
	docker compose -f docker-compose.dev.yml up -d

redis-stop:
	docker compose -f docker-compose.dev.yml down

redis-logs:
	docker compose -f docker-compose.dev.yml logs -f
