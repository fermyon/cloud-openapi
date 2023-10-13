DOCKER_IMAGE="openapitools/openapi-generator-cli:v6.6.0"
PACKAGE_NAME="cloud-openapi"
SWAGGER_ENDPOINT?=https://cloud.fermyon.com/swagger/v1/swagger.json"

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
	curl -sSLko swagger.json $(SWAGGER_ENDPOINT)

clean:
	rm -rf clients
