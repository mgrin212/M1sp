.global _lisp_entry
.extern _lisp_error
_lisp_entry:
	sub sp, sp, #32
	stp fp, lr, [sp, #16]
	mov fp, sp
	mov X0, #4
	str X0, [sp, #0]
	mov X0, #8
	str X0, [sp, #8]
	bl _add
	ldp fp, lr, [sp, #16]
	add sp, sp, #32
	ret
_add:
	sub sp, sp, #32
	stp fp, lr, [sp, #16]
	mov fp, sp
	ldr X0, [sp, #32]
	str X0, [sp, #0]
	ldr X0, [sp, #40]
	ldr X1, [sp, #0]
	add X0, X0, X1
	ldp fp, lr, [sp, #16]
	add sp, sp, #32
	ret

