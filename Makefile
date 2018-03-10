build:
	xargo build --target x86_64-blog_os

run:
	xargo run --target x86_64-blog_os

emulator:
	qemu-system-x86_64 -drive format=raw,file=bootimage.bin
