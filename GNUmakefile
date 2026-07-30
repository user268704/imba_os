.RECIPEPREFIX := >

KERNEL := target/x86_64-unknown-none/debug/imba_os
BUILD_DIR := build
ISO_ROOT := $(BUILD_DIR)/iso_root
ISO := $(BUILD_DIR)/imba_os.iso

LIMINE_DIR := limine
LIMINE_BRANCH := v10.x-binary

.PHONY: all kernel iso run clean distclean

all: iso

kernel:
>cargo build

$(LIMINE_DIR)/limine:
>rm -rf $(LIMINE_DIR)
>git clone \
>    --depth=1 \
>    --branch=$(LIMINE_BRANCH) \
>    https://github.com/limine-bootloader/limine.git \
>    $(LIMINE_DIR)
>$(MAKE) -C $(LIMINE_DIR)

iso: $(LIMINE_DIR)/limine kernel
>rm -rf $(ISO_ROOT)
>mkdir -p $(ISO_ROOT)/boot/limine
>mkdir -p $(BUILD_DIR)
>cp $(KERNEL) $(ISO_ROOT)/boot/kernel
>cp limine.conf $(ISO_ROOT)/boot/limine/limine.conf
>cp $(LIMINE_DIR)/limine-bios.sys $(ISO_ROOT)/boot/limine/
>cp $(LIMINE_DIR)/limine-bios-cd.bin $(ISO_ROOT)/boot/limine/
>xorriso -as mkisofs \
>    -b boot/limine/limine-bios-cd.bin \
>    -no-emul-boot \
>    -boot-load-size 4 \
>    -boot-info-table \
>    $(ISO_ROOT) \
>    -o $(ISO)
>./$(LIMINE_DIR)/limine bios-install $(ISO)
>@echo
>@echo "Created $(ISO)"

run: iso
>qemu-system-x86_64 \
>    -machine q35 \
>    -accel tcg \
>    -m 256M \
>    -cdrom $(ISO) \
>    -boot d \
>    -display none \
>    -serial stdio \
>    -monitor none \
>    -no-reboot \
>    -no-shutdown

clean:
>cargo clean
>rm -rf $(BUILD_DIR)

distclean: clean
>rm -rf $(LIMINE_DIR)
