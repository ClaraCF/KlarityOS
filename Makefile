# Build the x86_64 by default
.DEFAULT_GOAL := build-x86_64


# Set some variables
CC := x86_64-elf-gcc
LD := x86_64-elf-ld


# Gather all source files and objects
kernel_source_files := $(shell find src/impl/kernel -name *.c)
kernel_object_files := $(patsubst src/impl/kernel/%.c,build/kernel/%.o,$(kernel_source_files))

x86_64_c_source_files := $(shell find src/impl/x86_64 -name *.c)
x86_64_c_object_files := $(patsubst src/impl/x86_64/%.c,build/x86_64/%.o,$(x86_64_c_source_files))

x86_64_asm_source_files := $(shell find src/impl/x86_64 -name *.asm)
x86_64_asm_object_files := $(patsubst src/impl/x86_64/%.asm,build/x86_64/%.o,$(x86_64_asm_source_files))

# Gather both C and ASM objects
x86_64_object_files := $(x86_64_c_object_files) $(x86_64_asm_object_files)


# Build the kernel objects
$(kernel_object_files): build/kernel/%.o : src/impl/kernel/%.c
	mkdir -p $(dir $@)
	$(CC) -c -I src/intf -ffreestanding $(patsubst build/kernel/%.o,src/impl/kernel/%.c,$@) -o $@

# Build the C objects
$(x86_64_c_object_files): build/x86_64/%.o : src/impl/x86_64/%.c
	mkdir -p $(dir $@)
	$(CC) -c -I src/intf -ffreestanding $(patsubst build/x86_64/%.o,src/impl/x86_64/%.c,$@) -o $@

# Build the ASM objects
$(x86_64_asm_object_files): build/x86_64/%.o : src/impl/x86_64/%.asm
	mkdir -p $(dir $@)
	nasm -f elf64 $(patsubst build/x86_64/%.o,src/impl/x86_64/%.asm,$@) -o $@


# Build the 64-bit system
.PHONY: build-x86_64
build-x86_64: $(kernel_object_files) $(x86_64_object_files)
	mkdir -p dist/x86_64
	$(LD) -n -o dist/x86_64/kernel.bin -T targets/x86_64/linker.ld $(kernel_object_files) $(x86_64_object_files)
	cp dist/x86_64/kernel.bin targets/x86_64/iso/boot/kernel.bin
	grub-mkrescue /usr/lib/grub/i386-pc -o dist/x86_64/kernel.iso targets/x86_64/iso

# Remove unlinked objects and kernel binaries
.PHONY: clean
clean:
	rm -rf build/kernel								# Kernel objects
	rm -rf build/x86_64								# C and ASM objects
	rm -rf dist/x86_64/kernel.bin					# Kernel binary in dist directory
	rm -rf targets/x86_64/iso/boot/kernel.bin		# Kernel binary in targets directory

# Build the x86_64 by default
.PHONY: default
default: build-x86_64