.global _lisp_entry
.extern _lisp_error
_lisp_entry:
	mov X0, #159
	cmp X0, #31
	beq _else_1
_then_0:
	mov X0, #8
	b _continue_2
_else_1:
	mov X0, #12
_continue_2:
	ret
