; factorial of 5 = 120
; uses memory addresses after the program

    ; mem[1000] = result = 1
    push 1
    push 1000
    store

    ; mem[1008] = n = 5  
    push 5
    push 1008
    store

loop:
    ; if n <= 1, done
    push 1008
    load        ; n
    push 1
    gt          ; n > 1 ?
    jz done

    ; result = result * n
    push 1000
    load        ; result
    push 1008
    load        ; n
    mul         ; result * n
    push 1000
    store       ; mem[1000] = result * n

    ; n = n - 1
    push 1008
    load
    push 1
    sub
    push 1008
    store

    jmp loop

done:
    push 1000
    load
    puti
    push 10
    putc
    halt
