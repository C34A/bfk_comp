
    .equ SYS_EXIT, 60
    .equ SUCCESS, 0

    .equ SYS_WRITE, 1
    .equ STDOUT, 1

    .equ SYS_READ, 0
    .equ STDIN, 0

    .global _start

    .text
_start:
    mov $arr, %r12 # Initialize the memory pointer to the start of the array.
