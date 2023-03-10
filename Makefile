DOCKER_IMAGE="openapitools/openapi-generator-cli"
PACKAGE_NAME="cloud-openapi"

clients: swagger.json
	@echo "==> Building OpenAPI clients..."
	for lang in rust ; do \
		docker run \
			--rm \
			--volume "$(PWD):/local" \
			--workdir "/local" \
			 $(DOCKER_IMAGE) generate -i swagger.json -g $$lang -o clients/$$lang --package-name "$(PACKAGE_NAME)" --config config/$$lang.json ; \
	done

swagger.json:
	curl -sSLko swagger.json https://cloud.fermyon.com/swagger/v1/swagger.json

clean:
	rm -rf clients
