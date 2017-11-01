BUILDTYPE ?= release
BINDIR := bin

FFIDIR := $(shell readlink -f ../../target/$(BUILDTYPE))
FFILIB := candidateparser_ffi
FFISOF := $(FFIDIR)/lib$(FFILIB).so
FFIARF := $(FFIDIR)/lib$(FFILIB).a
FFISRC := $(shell readlink -f ../src/lib.rs)
FFIINC := -I$(shell readlink -f ../)

LDFLAGS := -L$(FFIDIR) -l$(FFILIB) -Wl,-rpath,$(FFIDIR)
BASEFLAGS := -g -Os -Wextra -Werror -pedantic-errors
CCFLAGS := -std=c99 $(BASEFLAGS) $(FFIINC)
CXXFLAGS := -std=c++1z $(BASEFLAGS) $(FFIINC)

C_OBJDIR := .cobjs
C_SRCDIR := c
C_SRCS := example.c

CPP_OBJDIR := .cppobjs
CPP_SRCDIR := cpp
CPP_SRCS := example.cpp

C_SRCS := $(patsubst %.c,$(C_SRCDIR)/%.c,$(C_SRCS))
C_OBJS := $(patsubst $(C_SRCDIR)/%.c,$(C_OBJDIR)/%.o,$(C_SRCS))
C_DEPS := $(patsubst %.o,%.d,$(C_OBJS))

CPP_SRCS := $(patsubst %.cpp,$(CPP_SRCDIR)/%.cpp,$(CPP_SRCS))
CPP_OBJS := $(patsubst $(CPP_SRCDIR)/%.cpp,$(CPP_OBJDIR)/%.o,$(CPP_SRCS))
CPP_DEPS := $(patsubst %.o,%.d,$(CPP_OBJS))

.PHONY: clean examples/ffi/c examples/ffi/cpp
.SECONDARY: $(C_OBJS) $(CPP_OBJS) $(BINDIR)/c_example $(BINDIR)/cpp_example

all: examples/ffi/c examples/ffi/cpp

clean:
	@rm -rf bin
	@rm -rf $(C_OBJDIR) $(CPP_OBJDIR)

# Fake examples

examples/ffi/c: $(BINDIR)/c_example
examples/ffi/cpp: $(BINDIR)/cpp_example

# Directory targets

$(BINDIR):
	@echo "MKDIR $@"
	@mkdir -p $@

$(C_OBJDIR):
	@echo "MKDIR $@"
	@mkdir -p $@

$(CPP_OBJDIR):
	@echo "MKDIR $@"
	@mkdir -p $@

# Rust library target

$(FFISOF): $(FFISRC)
	@echo "CARGO candidateparser-ffi"
ifeq ($(BUILDTYPE),debug)
		@cargo build
else
		@cargo build --$(BUILDTYPE)
endif

# Real C example

$(C_OBJDIR)/%.o: $(C_SRCDIR)/%.c | $(C_OBJDIR) $(FFISOF)
	@echo "CC $@"
	@$(CC) $(CCFLAGS) -MMD -MP -c -o $@ $<

-include $(C_OBJDIR)/*.d

$(BINDIR)/c_example: $(C_OBJS) | $(BINDIR)
	@echo "LINK $@"
	@$(CC) $(LDFLAGS) -o $@ $<

# Real C++ example

$(CPP_OBJDIR)/%.o: $(CPP_SRCDIR)/%.cpp | $(CPP_OBJDIR) $(FFISOF)
	@echo "CXX $@"
	@$(CXX) $(CXXFLAGS) -MMD -MP -c -o $@ $<

-include $(CPP_OBJDIR)/*.d

$(BINDIR)/cpp_example: $(CPP_OBJS) | $(BINDIR)
	@echo "LINK $@"
	@$(CXX) $(LDFLAGS) -o $@ $<
