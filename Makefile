all: build_release

# ****** Operating System ******
OS = $(shell uname -s)
ifeq ($(OS),Linux)
	DIR = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	DIR = ${PWD}
endif
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
NAME = $(shell grep -m 1 name Cargo.toml | cut -d '"' -f 2)
VERSION = "v$(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)"

DOCKER_REPO = jkutkut/
IMAGE_NAME = ${DOCKER_REPO}${NAME}
DEV_IMAGE_NAME = ${IMAGE_NAME}:dev
DEV_CONTAINER_NAME = ${NAME}_dev

SRC = $(wildcard src/*) \
	  Cargo.toml \
	  Makefile \
	  Dockerfile

test:
	${DOCKER_RUN_IT} --name ${DEV_CONTAINER_NAME} ${RUN_ATTRS} --entrypoint cargo ${DEV_IMAGE_NAME} test

test_watch:
	${DOCKER_RUN_IT} --name ${DEV_CONTAINER_NAME} ${RUN_ATTRS} --entrypoint cargo ${DEV_IMAGE_NAME} watch --clear test

# ****** Makefile stampts ******
STAMP = .stamps
DOCKER_FILE_STAMP = ${STAMP}/Dockerfile
BUILD_DEV_STAMP = ${STAMP}/build_dev
BUILD_RELEASE_STAMP = ${STAMP}/build_release

${STAMP}:
	@mkdir -p ${STAMP}

# ****** Docker Images ******

build_dev_image: ${DOCKER_FILE_STAMP} ${STAMP}
${DOCKER_FILE_STAMP}: Dockerfile
	docker build -t ${DEV_IMAGE_NAME} --target dev .
	@touch $@

remove_images:
	docker rmi ${DEV_IMAGE_NAME}

# ****** Docker Containers ******
stop_dev:
	docker stop ${DEV_CONTAINER_NAME}

connect_dev:
	docker exec -it ${DEV_CONTAINER_NAME} sh

terminal_dev:
	${DOCKER_RUN_IT} ${CODE_VOLUME} ${DEV_IMAGE_NAME}

# ****** Release / Build ******

WASM_PACK = wasm-pack
build_dev: ${BUILD_DEV_STAMP}
${BUILD_DEV_STAMP}: ${SRC} ${STAMP}
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint ${WASM_PACK} ${DEV_IMAGE_NAME} build --target web
	@touch $@

build_release: ${BUILD_RELEASE_STAMP}
${BUILD_RELEASE_STAMP}: ${SRC} ${STAMP}
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint ${WASM_PACK} ${DEV_IMAGE_NAME} build --target nodejs -d pkg-node
	@touch $@

publish_release:
	@echo "v$(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)" > /tmp/osmia-npm-version.txt
	@git tag --list | grep -q -v "^$(shell cat /tmp/osmia-npm-version.txt)$$" || (echo "Error: Tag already exists" && false)
	@echo "Ensuring repo has no uncommited changes..."
	@git diff --quiet && git diff --cached --quiet || (echo "Error: Repository not clean" && false)
	@echo "${REPO} is clean."
	@echo "Building release..."
	@make build_release
	@sudo chown -R ${USER}:${USER} .
	@echo "Preparing for commit..."
	@rm -rf /tmp/osmia-npm-release
	@cp -r pkg-node /tmp/osmia-npm-release
	@echo "Committing release..."
	@git checkout stable
	@rm -rf ./*
	@cp -r /tmp/osmia-npm-release/* .
	@git add .
	@cat /tmp/osmia-npm-version.txt | git commit -F -
	@xargs -n 1 git tag < /tmp/osmia-npm-version.txt
	@echo "Cleaning up..."
	@rm -rf /tmp/osmia-npm-release
	@rm -rf /tmp/osmia-npm-version.txt
	@echo "Done! Publishing release..."
	@git push
	@git push --tags
	@git checkout main

clean:
	@rm -rf ${STAMP}
	@rm -rf pkg-node 2> /dev/null || sudo rm -rf pkg-node
	@rm -rf pkg 2> /dev/null || sudo rm -rf pkg
	@rm -rf target 2> /dev/null || sudo rm -rf target

# ****** Git Hooks ******
prepare_commit: hooks
	${EDITOR} Cargo.toml
	make test
	git add Cargo.toml Cargo.lock; git add -N .;
	git add -p

hooks:
	git config core.hooksPath .githooks
	# git config --unset core.hooksPath
