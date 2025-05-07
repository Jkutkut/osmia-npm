# ****** Operating System ******
OS = $(shell uname -s)
ifeq ($(OS),Linux)
	DIR = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	DIR = ${PWD}
endif
# REPO = $(shell echo ${DIR} | sed 's/.*\///') # lowercase
REPO = $(shell echo ${DIR} | sed 's/.*\///' | tr '[:upper:]' '[:lower:]')

# ****** Rust Constants ******
CARGO = /root/.cargo/bin/cargo
CODE_VOLUME = -v ${DIR}:/${REPO} -w /${REPO}
CARGO_REGISTRY = -v cargo_registy:/root/.cargo/registry

# ****** Docker Constants ******
DOCKER_RUN = docker run --rm
DOCKER_RUN_IT = ${DOCKER_RUN} -it
DOCKER_RUN_D = ${DOCKER_RUN} -d

RUN_ATTRS = ${CODE_VOLUME} ${CARGO_REGISTRY} \
	-e RUSTFLAGS="-C target-feature=-crt-static"

terminal_installer:
	@echo "cargo --help"
	${DOCKER_RUN_IT} ${RUN_ATTRS} jkutkut/docker4rust
	sudo chown -R ${USER}:${USER} .

# ****** Project ******
# DEV_PORT = 9000
NAME = $(shell grep -m 1 name Cargo.toml | cut -d '"' -f 2)
VERSION = "v$(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)"


DOCKER_REPO = jkutkut/
IMAGE_NAME = ${DOCKER_REPO}${NAME}
DEV_IMAGE_NAME = ${IMAGE_NAME}:dev
RELEASE_IMAGE_NAME = ${IMAGE_NAME}:$(VERSION)
LATEST_IMAGE_NAME = ${IMAGE_NAME}:latest
DEV_CONTAINER_NAME = ${NAME}_dev
RELEASE_CONTAINER_NAME = ${NAME}_$(VERSION)

# ****** Docker Images ******

build_dev_image:
	docker build -t ${DEV_IMAGE_NAME} --target dev .

# ****** Docker Containers ******
stop_dev:
	docker stop ${DEV_CONTAINER_NAME}

stop_release:
	docker stop ${RELEASE_CONTAINER_NAME}

remove_images:
	docker rmi ${DEV_IMAGE_NAME}
	docker rmi ${RELEASE_IMAGE_NAME}
	@#docker rmi ${LATEST_IMAGE_NAME}

test:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint cargo ${DEV_IMAGE_NAME} test

test_watch:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint cargo ${DEV_IMAGE_NAME} watch --clear test

WASM_PACK = wasm-pack
build_dev:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint ${WASM_PACK} ${DEV_IMAGE_NAME} build --target web

connect_dev:
	docker exec -it ${DEV_CONTAINER_NAME} sh

terminal_dev:
	${DOCKER_RUN_IT} ${CODE_VOLUME} ${DEV_IMAGE_NAME}

build_release:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint ${WASM_PACK} ${DEV_IMAGE_NAME} build --target bundler
