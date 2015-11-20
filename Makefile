# This file is part of Genesis.

# Genesis is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# Genesis is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.

# You should have received a copy of the GNU Affero General Public License
# along with Genesis.  If not, see <http://www.gnu.org/licenses/>.
-include config.mk

# Configuration
ARCH ?= x86_64
TARGET_TRIPLE ?= $(ARCH)-none-elf

# Directories
ARCH_DIR ?= $(CURDIR)/src/arch/$(ARCH)
BUILD_DIR ?= $(CURDIR)/build/$(ARCH)
ifeq ($(DEBUG),1)
CARGO_OUT_DIR ?= $(CURDIR)/target/target/debug
else
CARGO_OUT_DIR ?= $(CURDIR)/target/target/release
endif
CORE_DIR ?= $(CURDIR)/ext/core
TOOLCHAIN_DIR ?= $(CURDIR)/toolchain/install

# Executables
CARGO ?= cargo
CROSSAS ?= $(TOOLCHAIN_DIR)/bin/$(TARGET_TRIPLE)-as
CROSSLD ?= $(TOOLCHAIN_DIR)/bin/$(TARGET_TRIPLE)-ld
MKDIR ?= mkdir
RUSTC ?= rustc
TAR ?= tar
WGET ?= wget

# Files
TARGET_SPEC ?= $(ARCH_DIR)/target.json
LDSCRIPT := $(ARCH_DIR)/linkerscript.ld

# Misc helpers
rparen := )

# Flags
ASFLAGS ?=
CARGOFLAGS ?=
LDFLAGS ?= -z max-page-size=0x1000 --gc-sections
RUSTCFLAGS ?=
ifeq ($(DEBUG),1)
ASFLAGS += -g
else
CARGOFLAGS += --release
LDFLAGS += --strip-debug
RUSTCFLAGS += -O
endif

# Misc info
RUSTC_PREFIX ?= rustc-nightly
RUSTC_SRC_TAR ?= $(RUSTC_PREFIX)-src.tar.gz
RUSTC_SRC_URL ?= https://static.rust-lang.org/dist/$(RUSTC_SRC_TAR)

# Source
ARCH_SRCS ?= $(wildcard $(ARCH_DIR)/*.s)

# Objects
ARCH_OBJS ?= $(patsubst $(ARCH_DIR)/%.s,$(BUILD_DIR)/%.o,$(ARCH_SRCS))
CORE_LIB ?= $(BUILD_DIR)/libcore.rlib
KERNEL_LIB ?= $(CARGO_OUT_DIR)/libgenesis.a

OBJS ?= $(ARCH_OBJS)

KERNEL ?= $(BUILD_DIR)/genesis.elf

export PATH := $(CURDIR)/bin:$(PATH)
export CORE_LIB_PATH := $(BUILD_DIR)

.SUFFIXES:
.PHONY: all clean clean-cargo clean-core clean-kernel clean-objs \
	clean-toolchain core dist-clean kernel_lib toolchain

all: $(KERNEL)

$(BUILD_DIR):
	$(MKDIR) $(BUILD_DIR)

$(BUILD_DIR)/%.o: $(ARCH_DIR)/%.s Makefile | $(BUILD_DIR)
	$(CROSSAS) $(ASFLAGS) -o $@ $<

kernel_lib: core
	$(CARGO) build $(CARGOFLAGS) --target=$(TARGET_SPEC)

$(KERNEL): $(OBJS) kernel_lib $(LDSCRIPT) Makefile | $(BUILD_DIR)
	$(CROSSLD) $(LDFLAGS) -o $@ -T $(LDSCRIPT) $(OBJS) $(KERNEL_LIB)

# clean
clean: clean-cargo clean-kernel clean-objs

clean-cargo:
	$(CARGO) clean

clean-core:
	-$(RM) $(CORE_DIR)/$(RUSTC_SRC_TAR)
	-$(RM) -r $(CORE_DIR)/src/*
	-$(RM) $(CORE_LIB)

clean-kernel:
	-$(RM) $(wildcard $(KERNEL))

clean-objs:
	-$(RM) $(wildcard $(OBJS))

clean-toolchain:
	$(MAKE) -C toolchain clean

dist-clean: clean-cargo clean-core clean-kernel clean-objs clean-toolchain

# core
core: $(CORE_LIB)

$(CORE_LIB): $(CORE_DIR)/src/lib.rs $(TARGET_SPEC) Makefile | $(BUILD_DIR)
	$(RUSTC) -O $(RUSTCFLAGS) --target=$(TARGET_SPEC) -o $@ $<

$(CORE_DIR)/src:
	$(MKDIR) -p $(CORE_DIR)/src

$(CORE_DIR)/src/lib.rs: $(CORE_DIR)/$(RUSTC_SRC_TAR) | $(CORE_DIR)/src
	$(TAR) xf $< -C $(CORE_DIR)/src $(RUSTC_PREFIX)/src/libcore --strip 3 -m

$(CORE_DIR)/$(RUSTC_SRC_TAR):
	$(WGET) -P $(CORE_DIR) $(RUSTC_SRC_URL) -N -nv

# toolchain
toolchain:
	$(MAKE) -C toolchain

# misc
print-%:
	@echo $* = $($*)
