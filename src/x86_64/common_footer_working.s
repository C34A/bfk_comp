
    mov $SYS_EXIT, %rax # Exit 
    mov $SUCCESS, %rdi # with code 0 (success)
    syscall

    .bss
arr .lcomm 30000 # The memory array
