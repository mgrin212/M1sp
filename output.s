.global _lisp_entry
.extern _lisp_error
_lisp_entry:
	mov X0, #20
	str X0, [sp, #-8]
	mov X0, #12
	str X0, [sp, #-16]
	ldr X0, [sp, #-8]
	str X0, [sp, #-24]
	ldr X0, [sp, #-16]
	ldr X1, [sp, #-24]
	add X0, X0, X1
	ldr X0, [sp, #-8]
	str X0, [sp, #-24]
	ldr X0, [sp, #-16]
	mov X1, X0
	ldr X0, [sp, #-24]
	sub X0, X0, X1
	ret

