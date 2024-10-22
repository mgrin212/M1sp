.global _lisp_entry
.extern _lisp_error
_fib:
	sub sp, sp, #32
	stp fp, lr, [sp, #16]
	add fp, sp, #16
	ldr X0, [sp, #24]
	str X0, [sp, #16]
	mov X0, #8
	ldr X1, [sp, #16]
	cmp X1, X0
	mov X0, #0
	cset X0, lt
	lsl X0, X0, #7
	orr X0, X0, #31
	cmp X0, #31
	beq _else_1
_then_0:
	ldr X0, [sp, #24]
	b _continue_2
_else_1:
	mov X0, #8
_continue_2:
	ldp fp, lr, [sp, 16]
	add sp, sp, #32
	ret
_lisp_entry:
	sub sp, sp, #32
	stp fp, lr, [sp, #16]
	add fp, sp, #16
	mov X0, #4
	str X0, [sp, #-8]
	bl _fib
	ldp fp, lr, [sp, 16]
	add sp, sp, #32
	ret

