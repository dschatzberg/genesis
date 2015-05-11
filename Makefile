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

ARCH ?= x86_64

ARCHDIR := $(CURDIR)/src/arch/$(ARCH)

TARGET ?= $(ARCH)-unknown-linux-gnu

TARGET_DIR ?= $(CURDIR)/target

TOOLCHAIN_TARGET ?= $(ARCH)-elf

TOOLCHAIN_DIR ?= $(CURDIR)/toolchain/install

CROSSAS ?= $(TOOLCHAIN_DIR)/bin/$(TOOLCHAIN_TARGET)-as
CROSSLD ?= $(TOOLCHAIN_DIR)/bin/$(TOOLCHAIN_TARGET)-ld
CARGO ?= cargo

MKDIR ?= mkdir

LDFLAGS ?= -z max-page-size=0x1000 --gc-sections

BUILDDIR ?= $(CURDIR)/build

ifeq ($(DEBUG),1)
  ASFLAGS ?= -g
  GENESISLIB := $(TARGET_DIR)/$(TARGET)/debug/libgenesis.a
else
  CARGOFLAGS := --release
  LDFLAGS += --strip-debug
  GENESISLIB := $(TARGET_DIR)/$(TARGET)/release/libgenesis.a
endif

OBJS :=

include $(ARCHDIR)/Makefile.in

.PHONY: all clean $(GENESISLIB) toolchain

all: $(BUILDDIR)/kernel.elf

clean:
	$(CARGO) clean
	-$(RM) $(wildcard $(OBJS) $(BUILDDIR)/kernel.elf)

toolchain:
	$(MAKE) -C toolchain

$(GENESISLIB): Makefile
	$(CARGO) build $(CARGOFLAGS) --target=$(TARGET)

$(BUILDDIR):
	$(MKDIR) $(BUILDDIR)

$(BUILDDIR)/%.o: $(ARCHDIR)/%.s Makefile | $(BUILDDIR)
	$(CROSSAS) $(ASFLAGS) -o $@ $<

$(BUILDDIR)/kernel.elf: $(LDSCRIPT) $(OBJS) $(GENESISLIB) Makefile | $(BUILDDIR)
	$(CROSSLD) $(LDFLAGS) -o $@ -T $(LDSCRIPT) $(OBJS) $(GENESISLIB)
