# Output generated by the C-Minus Compiler
.LCX:
	.comm	vsol, 4
	.globl	fibonacci
fibonacci:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	%edi, %edi
	call	writeinteger
	movl	$0, %r12d
	cmpl	%r12d, %edi
	jle	.L14
	movl	$0, %r13d
	movl	%r13d, %eax
	leave
	ret
	jmp	.L16
.L14:
	movl	$1, %r14d
	cmpl	%r14d, %edi
	je	.L18
	movl	$1, %r15d
	movl	%r15d, %eax
	leave
	ret
	jmp	.L20
.L18:
	movl	$2, %r13d
	cmpl	%r13d, %edi
	je	.L22
	movl	$1, %r13d
	movl	%r13d, %eax
	leave
	ret
	jmp	.L24
.L22:
	movl	$1, %r13d
	movl	%edi, %r13d
	subl	%r13d, %r13d
	movl	%r13d, %edi
	call	fibonacci
	movl	%eax, %r13d
	movl	$2, %r13d
	movl	%edi, %r13d
	subl	%r13d, %r13d
	movl	%r13d, %edi
	call	fibonacci
	movl	%eax, %r13d
	movl	%r13d, %r13d
	addl	%r13d, %r13d
	movl	%r13d, %eax
	leave
	ret
.L24:
.L20:
.L16:
	leave
	ret
	.globl	main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp
	call	readinteger
	movl	%eax, %r13d
	movl	%r13d, -4(%rbp)
	movl	-4(%rbp), %edi
	call	fibonacci
	movl	%eax, %r13d
	movl	%r13d, vsol(%rip)
	movl	vsol(%rip), %edi
	call	writeinteger
	movl	$0, %r13d
	movl	%r13d, %eax
	leave
	ret
.LC0:
	.string "%d"
	.globl readinteger
readinteger:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp
	leaq	-12(%rbp), %rsi
	leaq	.LC0(%rip), %rdi
	call	scanf
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
.LC1:
	.string "%d\n"
	.globl writeinteger
writeinteger:
	pushq	%rbp
	movl	%edi, %esi
	leaq	.LC1(%rip), %rdi
	call	printf
	popq	%rbp
	ret
.LC2:
	.string "%u"
	.globl readunsigned
readunsigned:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp
	leaq	-12(%rbp), %rsi
	leaq	.LC2(%rip), %rdi
	call	scanf
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
.LC3:
	.string "%u\n"
	.globl writeunsigned
writeunsigned:
	pushq	%rbp
	movl	%edi, %esi
	leaq	.LC3(%rip), %rdi
	call	printf
	popq	%rbp
	ret
