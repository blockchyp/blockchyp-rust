# Version config
TAG := $(shell git tag --points-at HEAD | sort --version-sort | tail -n 1)
LASTTAG := $(or $(shell git tag -l | sort -r -V | head -n 1),0.1.0)
SNAPINFO := $(shell date +%Y%m%d%H%M%S)git$(shell git log -1 --pretty=%h)
RELEASE := $(or $(BUILD_NUMBER), 1)
VERSION := $(or $(TAG:v%=%),$(LASTTAG:v%=%))-$(or $(BUILD_NUMBER), 1)$(if $(TAG),,.$(SNAPINFO))

# Executables
DOCKER = docker
CARGO = cargo
SED = sed
SED_SUBST = $(SED)
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	SED_SUBST += -i ''
else
	SED_SUBST += -i
endif

# Integration test config
export BC_TEST_DELAY := 5
IMAGE := rust:1.79-bookworm
SCMROOT := $(shell git rev-parse --show-toplevel)
PWD := $(shell pwd)
CACHE := $(HOME)/.local/share/blockchyp/itest-cache
CONFIGFILE := $(HOME)/.config/blockchyp/sdk-itest-config.json
CACHEPATHS := $(dir $(CONFIGFILE)) $(HOME)/.cargo $(HOME)/.config/configstore
ifeq ($(shell uname -s), Linux)
HOSTIP = $(shell ip -4 addr show docker0 | grep -Po 'inet \K[\d.]+')
else
HOSTIP = host.docker.internal
endif

# Default target
.PHONY: all
all: clean build test

# Cleans the package
.PHONY: clean
clean:
	$(CARGO) clean

# Builds the package
.PHONY: build
build:
	$(CARGO) build

# Runs unit tests
.PHONY: test
test:

# Lint the package
.PHONY: lint
lint:
	$(CARGO) clippy

# Publish the package
.PHONY: publish
publish: 
	$(CARGO) publish

# Test publishing the package
.PHONY: dry-run-publish
dry-run-publish: 
	$(CARGO) publish --dry-run

# Performs any tasks necessary before a release build
.PHONY: stage
stage:
	$(SED_SUBST) "s/^version = \".*\"/version = \"$(VERSION)\"/" Cargo.toml

# Runs integration tests
.PHONY: integration
integration:
	$(if $(LOCALBUILD), \
		$(CARGO) build && $(CARGO) test $(if $(TEST),--test (TEST),--no-fail-fast), \
		$(foreach path,$(CACHEPATHS),mkdir -p $(CACHE)/$(path) ; ) \
		sed 's/localhost/$(HOSTIP)/' $(CONFIGFILE) >$(CACHE)/$(CONFIGFILE) ; \
		$(DOCKER) run \
			-v $(SCMROOT):$(SCMROOT):Z \
			$(foreach path,$(CACHEPATHS),-v $(CACHE)$(path):$(path):Z) \
			-e BC_TEST_DELAY=$(BC_TEST_DELAY) \
			-e HOME=$(HOME) \
			-w $(PWD) \
			--init \
			--rm -it $(IMAGE) \
			/bin/sh -c "$(CARGO) build && $(CARGO) test --release $(if $(TEST),--test $(TEST),--tests --no-fail-fast)")
