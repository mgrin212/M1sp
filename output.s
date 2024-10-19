.global _lisp_entry
.extern _lisp_error
_lisp_entry:
	mov X0, #8
	str X0, [sp, #-8]
	mov X0, #12
	ldr X1, [sp, #-8]
	add X0, X0, X1
	ret
