	.file	"test.c"
	.text
	.globl	func2
	.type	func2, @function
func2:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$1, -8(%rbp)
	movl	$2, -4(%rbp)
	movl	-8(%rbp), %edx
	movl	-4(%rbp), %eax
	addl	%edx, %eax
	popq	%rbp
	ret
	.size	func2, .-func2
	.globl	func
	.type	func, @function
func:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp
	movl	$1, -8(%rbp)
	movl	$2, -4(%rbp)
	movl	-8(%rbp), %edx
	movl	-4(%rbp), %eax
	addl	%edx, %eax
	cmpl	$2, %eax
	jne	.L4
	movl	$0, %eax
	call	func2
	addl	%eax, -8(%rbp)
.L4:
	movl	-8(%rbp), %eax
	leave
	ret
	.size	func, .-func
	.globl	main
	.type	main, @function
main:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp
	movl	$2, -4(%rbp)
	movl	$0, %eax
	call	func
	leave
	ret
	.size	main, .-main
	.ident	"GCC: (Ubuntu 7.5.0-3ubuntu1~18.04) 7.5.0"
	.section	.note.GNU-stack,"",@progbits
