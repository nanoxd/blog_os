build:
	xargo build --target x86_64-blog_os

run:
	xargo run --target x86_64-blog_os

image:
	bootimage --target x86_64-blog_os

emulator: image
	qemu-system-x86_64 -drive format=raw,file=bootimage.bin
