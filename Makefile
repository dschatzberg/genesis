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

TOOLCHAIN_TARGET ?= $(ARCH)-elf

TOOLCHAIN_DIR ?= $(CURDIR)/toolchain/install

CROSSAS ?= $(TOOLCHAIN_DIR)/bin/$(TOOLCHAIN_TARGET)-as
CROSSLD ?= $(TOOLCHAIN_DIR)/bin/$(TOOLCHAIN_TARGET)-ld

MKDIR ?= mkdir

LDFLAGS ?= -z max-page-size=0x1000

BUILDDIR ?= $(CURDIR)/build

ifeq ($(DEBUG),1)
  ASFLAGS ?= -g
else
  LDFLAGS += --strip-debug
endif

OBJS :=

include $(ARCHDIR)/Makefile.in

.PHONY: all clean toolchain

all: $(BUILDDIR)/kernel.elf

clean:
	-$(RM) $(wildcard $(OBJS) $(BUILDDIR)/kernel.elf)

toolchain:
	$(MAKE) -C toolchain

$(BUILDDIR):
	$(MKDIR) $(BUILDDIR)

$(BUILDDIR)/%.o: $(ARCHDIR)/%.s Makefile | $(BUILDDIR)
	$(CROSSAS) $(ASFLAGS) -o $@ $<

$(BUILDDIR)/kernel.elf: $(LDSCRIPT) $(OBJS) Makefile | $(BUILDDIR)
	$(CROSSLD) $(LDFLAGS) -o $@ -T $(LDSCRIPT) $(OBJS)
