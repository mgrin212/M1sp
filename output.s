.global _lisp_entry
.extern _lisp_error
_lisp_entry:
	mov X0, #168
	str X0, [sp, #-8]
	mov X0, #400
	str X0, [sp, #-16]
	mov X0, #159
	str X0, [sp, #-24]
	ldr X0, [sp, #-8]
	add X0, X0, #4
	ldr X0, [sp, #-16]
	sub X0, X0, #4
	mov X0, #0
	cmp X0, #0
	mov X0, #0
	cset X0, eq
	lsl X0, X0, #7
	orr X0, X0, #31
	ldr X0, [sp, #-8]
	and X0, X0, #3
	cmp X0, #0
	mov X0, #0
	cset X0, eq
	lsl X0, X0, #7
	orr X0, X0, #31
	ldr X0, [sp, #-24]
	cmp X0, #31
	mov X0, #0
	cset X0, eq
	lsl X0, X0, #7
	orr X0, X0, #31
	ldr X0, [sp, #-8]
	str X0, [sp, #-32]
	ldr X0, [sp, #-16]
	ldr X1, [sp, #-32]
	add X0, X0, X1
	ldr X0, [sp, #-8]
	str X0, [sp, #-32]
	ldr X0, [sp, #-16]
	mov X1, X0
	ldr X0, [sp, #-32]
	sub X0, X0, X1
	ldr X0, [sp, #-8]
	str X0, [sp, #-32]
	ldr X0, [sp, #-16]
	ldr X1, [sp, #-32]
	cmp X1, X0
	mov X0, #0
	cset X0, eq
	lsl X0, X0, #7
	orr X0, X0, #31
	ldr X0, [sp, #-8]
	str X0, [sp, #-32]
	ldr X0, [sp, #-16]
	ldr X1, [sp, #-32]
	cmp X1, X0
	mov X0, #0
	cset X0, lt
	lsl X0, X0, #7
	orr X0, X0, #31
	ret

