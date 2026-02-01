; function call demo

    push 5
    call double
    puti        ; should print 10
    push 10
    putc
    halt

double:
    push 2
    mul
    ret
