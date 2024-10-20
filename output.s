.global _lisp_entry
.extern _lisp_error
_lisp_entry:
	stp fp, lr, [sp, #-16]
	mov fp, sp
	mov X0, #20
	str X0, [sp, #-8]
	mov X0, #12
	ldr X1, [sp, #-8]
	add X0, X0, X1
	str X0, [sp, #-8]
	ldr X0, [sp, #-8]
	str X0, [sp, #-16]
	mov X0, #40
	ldr X1, [sp, #-16]
	cmp X1, X0
	mov X0, #0
	cset X0, lt
	lsl X0, X0, #7
	orr X0, X0, #31
	cmp X0, #31
	beq _else_1
_then_0:
	ldr X0, [sp, #-8]
	add X0, X0, #4
	b _continue_2
_else_1:
	ldr X0, [sp, #-8]
	sub X0, X0, #4
_continue_2:
	ldp fp, lr, [sp, #16]
	ret
_test:
	stp fp, lr, [sp, #-16]
	mov fp, sp
	mov X0, #20
	str X0, [sp, #-8]
	mov X0, #12
	ldr X1, [sp, #-8]
	add X0, X0, X1
	str X0, [sp, #-8]
	ldr X0, [sp, #-8]
	str X0, [sp, #-16]
	mov X0, #48
	ldr X1, [sp, #-16]
	add X0, X0, X1
	ldp fp, lr, [sp, #16]
	ret

