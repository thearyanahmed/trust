DOCKER_COMPOSE_EXEC = docker-compose exec trust
IP=192.168.0.2

.PHONY: run ping

run:
	@$(DOCKER_COMPOSE_EXEC) bash ./run

analyze:
	@$(DOCKER_COMPOSE_EXEC) bash tshark -i tun0

ping:
	@$(DOCKER_COMPOSE_EXEC) bash ./c_ping $(IP)
