NIGHTLY_TAR ?= rustc-nightly-src.tar.gz
NIGHTLY_URL ?= https://static.rust-lang.org/dist/$(NIGHTLY_TAR)

.PHONY: all clean core $(NIGHTLY_TAR)

all: core

clean:
	-$(RM) $(NIGHTLY_TAR)
	-$(RM) -r src/*

core: $(NIGHTLY_TAR)
	tar xf $< -C src rustc-nightly/src/libcore --transform 's~^rustc-nightly/src/libcore/~~'

$(NIGHTLY_TAR):
	wget $(NIGHTLY_URL) -N -nv
