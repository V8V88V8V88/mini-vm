; count from 1 to 5

    push 1      ; counter

loop:
    dup
    puti        ; print counter
    push 10
    putc        ; newline

    push 1
    add         ; counter++

    dup
    push 6
    lt          ; counter < 6 ?
    jnz loop

    pop
    halt
