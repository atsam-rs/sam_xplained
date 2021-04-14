# OpenOCD
target extended-remote :3333

# JLink
# target remote :2331
#monitor semihosting enable
#monitor semihosting IOClient 2

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

monitor arm semihosting enable

load

# Continue running
continue

# Immediately step and halt the processor
#stepi   