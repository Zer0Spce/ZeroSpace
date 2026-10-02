PS5_HOST ?= 192.168.1.50
PS5_PORT ?= 9021

ifdef PS5_PAYLOAD_SDK
include $(PS5_PAYLOAD_SDK)/toolchain/prospero.mk
else
$(error PS5_PAYLOAD_SDK is undefined)
endif

ELF := zerospace-helper.elf

# The audited GPL payload source will be added under src/ before this target is
# enabled. Keeping the target explicit prevents accidentally shipping an
# untracked upstream binary.
SRCS := $(wildcard src/*.c)
INCLUDES := -Iinclude
CFLAGS := -Wall -Wextra -Werror -g $(INCLUDES)

all:
	@if [ -z "$(SRCS)" ]; then echo "ZeroSpace Helper source import not complete; see README.md"; exit 1; fi
	$(CC) $(CFLAGS) -o $(ELF) $(SRCS)

send: $(ELF)
	@echo "Sending $(ELF) to $(PS5_HOST):$(PS5_PORT)"
	nc -w 1 $(PS5_HOST) $(PS5_PORT) < $(ELF)

clean:
	rm -f $(ELF) *.o

.PHONY: all send clean
