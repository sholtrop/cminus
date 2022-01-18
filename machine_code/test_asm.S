	.file	"test.c"
	.text
	.globl	main
	.type	main, @function
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$0, -8(%rbp)
	movl	$10, -4(%rbp)
	jmp	.L2
.L3:
	addl	$1, -8(%rbp)
.L2:
	movl	-8(%rbp), %eax
	;		10		~ 0
	cmpl	-4(%rbp), %eax
	jl	.L3
	movl	-8(%rbp), %eax
	popq	%rbp
	ret
	.size	main, .-main
	.ident	"GCC: (Ubuntu 7.5.0-3ubuntu1~18.04) 7.5.0"
	.section	.note.GNU-stack,"",@progbits
