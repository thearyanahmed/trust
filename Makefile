DOCKER_COMPOSE_EXEC = docker-compose exec trust
IP=192.168.0.2

.PHONY: run ping

run:
	@$(DOCKER_COMPOSE_EXEC) bash ./run

analyze:
	@$(DOCKER_COMPOSE_EXEC) bash tshark -i tun0

ping:
	@$(DOCKER_COMPOSE_EXEC) bash ./container_commands ping $(IP)

nc:
	@$(DOCKER_COMPOSE_EXEC) bash ./container_commands nc $(IP) $(filter-out $@,$(MAKECMDGOALS))

%:
	@

tshark:
	@$(DOCKER_COMPOSE_EXEC) bash ./container_commands tshark

down:
	@docker-compose down

up:
	@docker-compose up