DOCKER_COMPOSE_EXEC = docker-compose exec trust
IP=192.168.0.2

.PHONY: run ping

run:
	@$(DOCKER_COMPOSE_EXEC) bash ./run

ping:
	@$(DOCKER_COMPOSE_EXEC) bash ./c_ping $(IP)
