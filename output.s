.global _lisp_entry
.extern _lisp_error
_lisp_entry:
	sub sp, sp, #48
	mov X0, #4
	str X0, [sp, #-16]
	mov X0, #8
	str X0, [sp, #-32]
	sub sp, sp, #64
	stp fp, lr, [sp]
	ldr X0, [sp, #-16]
	str X0, [sp, #-48]
	ldr X0, [sp, #-32]
	str X0, [sp, #-64]
	mov X0, #12
	str X0, [sp, #-80]
	bl _add
	ldp fp, lr, [sp]
	add sp, sp, #64
	add sp, sp, #48
	ret
_add:
	sub sp, sp, #48
	stp fp, lr, [sp]
	mov fp, sp
	ldr X0, [sp, #-32]
	str X0, [sp, #-96]
	ldr X0, [sp, #-48]
	str X0, [sp, #-128]
	ldr X0, [sp, #-64]
	ldr X1, [sp, #-128]
	add X0, X0, X1
	ldr X1, [sp, #-96]
	add X0, X0, X1
	ldp fp, lr, [sp]
	add sp, sp, #48
	ret

