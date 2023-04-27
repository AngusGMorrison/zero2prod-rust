.PHONY: db migrate

db:
	./scripts/init_db.sh

migrate:
	sqlx migrate run --database-url ${ZERO2PROD_POSTGRES_URL}
