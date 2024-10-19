.global _lisp_entry

.text
_lisp_entry:
    // Save the link register
    stp     x29, x30, [sp, #-16]!
    mov     x29, sp

    // Load the address of the hello world string
    adrp    x0, hello_str@PAGE
    add     x0, x0, hello_str@PAGEOFF

    // Call printf
    bl      _printf

    // Return 0
    mov     x0, #0

    // Restore the link register and return
    ldp     x29, x30, [sp], #16
    ret

.data
hello_str:
    .asciz  "Hello, World!\n"
