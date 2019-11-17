docker-init:
	cp .env.sample .env
	make docker-start

docker-start:
	docker-compose up -d

docker-stop:
	docker-compose stop

docker-down:
	docker-compose down
	rm -r ./tmp || true
	rm .env || true

docker-shell-backend:
	docker exec  -it howtocards-public-api bash

docker-migration:
	docker exec -i howtocards-public-api bash -c 'cd /app && diesel migration run'
