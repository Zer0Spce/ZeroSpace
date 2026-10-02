PS5_HOST ?= 192.168.1.50
PS5_PORT ?= 9021
PYTHON ?= python3

ifdef PS5_PAYLOAD_SDK
include $(PS5_PAYLOAD_SDK)/toolchain/prospero.mk
endif

UPSTREAM_DIR := upstream/payload
UPSTREAM_ELF := $(UPSTREAM_DIR)/ps5upload.elf
ELF := zerospace-helper.elf

all: source
ifndef PS5_PAYLOAD_SDK
	$(error PS5_PAYLOAD_SDK is undefined)
endif
	$(MAKE) -C $(UPSTREAM_DIR)
	cp $(UPSTREAM_ELF) $(ELF)
	@echo "Built $(ELF) from pinned GPL source; see NOTICE.md and upstream/LICENSE"

source:
	$(PYTHON) scripts/import_upstream.py

send: $(ELF)
	@echo "Sending $(ELF) to $(PS5_HOST):$(PS5_PORT)"
	nc -w 1 $(PS5_HOST) $(PS5_PORT) < $(ELF)

clean:
	rm -rf upstream $(ELF)

.PHONY: all source send clean
