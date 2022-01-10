gcc $@ -S -fno-asynchronous-unwind-tables -std=c99 -no-pie test.c -o test_asm.S
gcc test_asm.S -o test