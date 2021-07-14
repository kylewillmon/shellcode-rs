.code

public tail_call_asm
tail_call_asm:
    jmp rcx

public ret_call_asm
ret_call_asm:
    pop rax
    push rcx
    ret

end