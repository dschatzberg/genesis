BINUTILS-VERSION ?= 2.25

TARGET ?= x86_64-elf

SRCDIR := $(CURDIR)/src
BUILDDIR := $(CURDIR)/build
INSTALLDIR := $(CURDIR)/install

BINUTILS-TAR := binutils-$(BINUTILS-VERSION).tar.bz2

BINUTILS-TAR-PATH := $(SRCDIR)/$(BINUTILS-TAR)

TOOLCHAIN-TAR ?= genesis-toolchain.tar.bz2
TOOLCHAIN-TAR-URL ?= https://github.com/dschatzberg/genesis/releases/download/toolchain-v0.0.1/$(TOOLCHAIN-TAR)

BINUTILS-URL ?= ftp://ftp.gnu.org/gnu/binutils/$(BINUTILS-TAR)

BINUTILS-SRCDIR := $(SRCDIR)/binutils-$(BINUTILS-VERSION)

BINUTILS-BUILDDIR := $(BUILDDIR)/binutils-$(BINUTILS-VERSION)

CD ?= cd
MKDIR ?= mkdir
TAR ?= tar
TOUCH ?= touch
WGET ?= wget

.PHONY: all fetch install clean \
	binutils-$(BINUTILS-VERSION)-build \
	binutils-$(BINUTILS-VERSION)-install

all: fetch

fetch: $(TOOLCHAIN-TAR)
	$(TAR) -xf $(TOOLCHAIN-TAR)

install: $(BINUTILS-BUILDDIR)/installed

clean:
	-$(RM) -r $(SRCDIR) $(BUILDDIR) $(INSTALLDIR) $(TOOLCHAIN-TAR)

# fetch tar

$(TOOLCHAIN-TAR):
	$(WGET) $(TOOLCHAIN-TAR-URL)

# General directories

$(SRCDIR):
	$(MKDIR) $(SRCDIR)

$(BUILDDIR):
	$(MKDIR) $(BUILDDIR)

$(INSTALLDIR):
	$(MKDIR) $(INSTALLDIR)

# fetch tars

$(BINUTILS-TAR-PATH): | $(SRCDIR)
	$(WGET) $(BINUTILS-URL) -O $(BINUTILS-TAR-PATH)

# untar

$(BINUTILS-SRCDIR)/configure: $(BINUTILS-TAR-PATH)
	$(TAR) --touch -xf $(BINUTILS-TAR-PATH) -C $(SRCDIR)

# make build dirs

$(BINUTILS-BUILDDIR): | $(BUILDDIR)
	$(MKDIR) $(BINUTILS-BUILDDIR)

# configure

$(BINUTILS-BUILDDIR)/Makefile: $(BINUTILS-SRCDIR)/configure | $(BINUTILS-BUILDDIR)
	$(CD) $(BINUTILS-BUILDDIR) && $(BINUTILS-SRCDIR)/configure \
		--target=$(TARGET) \
		--prefix=$(INSTALLDIR) --enable-static --disable-shared \
		--disable-nls --disable-werror

# build phonies

binutils-$(BINUTILS-VERSION)-build: $(BINUTILS-BUILDDIR)/built

# build

$(BINUTILS-BUILDDIR)/built: $(BINUTILS-BUILDDIR)/Makefile
	$(MAKE) MAKEINFO=true -C $(BINUTILS-BUILDDIR)
	$(TOUCH) $(BINUTILS-BUILDDIR)/built

# install phonies

binutils-$(BINUTILS-VERSION)-install: $(BINUTILS-BUILDDIR)/installed

# install

$(BINUTILS-BUILDDIR)/installed: $(BINUTILS-BUILDDIR)/built
	$(MAKE) -C $(BINUTILS-BUILDDIR) MAKEINFO=true install
	$(TOUCH) $(BINUTILS-BUILDDIR)/installed
