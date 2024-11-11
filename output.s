.global _lisp_entry
.extern _lisp_error
_lisp_entry:
	mov X0, #255
	ret
_square:
	sub sp, sp, #48
	stp fp, lr, [sp]
	mov fp, sp
	ldr X0, [sp, #-32]
	str X0, [sp, #-64]
	ldr X0, [sp, #-32]
	ldr X1, [sp, #-64]
	add X0, X0, X1
	ldp fp, lr, [sp]
	add sp, sp, #48
	ret

