OUTPUT=esp32.svd
BASE=esp32.base.svd

all: clean patch generate form fmt build

codegen: clean generate form fmt build

clean:
	rm -rf src/

patch:
	rm -f svd/$(OUTPUT)
	svd patch svd/patches/esp32.yaml
	mv svd/$(BASE).patched svd/$(OUTPUT)

generate:
	svd2rust --target xtensa-lx -i svd/$(OUTPUT)

form:
	form -i lib.rs -o src/
	rm lib.rs

fmt:
	cargo fmt

build:
	cargo clean
	cargo +esp build -Zbuild-std --target xtensa-esp32-none-elf
