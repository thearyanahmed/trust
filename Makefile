DOCKER_COMPOSE_EXEC = docker-compose exec trust

.PHONY: run

run: ## Download the depenedencies then build the image :rocket:.
	$(DOCKER_COMPOSE_EXEC) bash -c "$(filter-out $@,$(MAKECMDGOALS))"
