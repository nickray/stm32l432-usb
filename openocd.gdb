target remote :3333

set print asm-demangle on
set print pretty on

# detect unhandled exceptions, hard faults and panics
#break DefaultHandler
#break UserHardFault
#break rust_begin_unwind

monitor reset halt

monitor arm semihosting enable

# # send captured ITM to the file itm.fifo
# # (the microcontroller SWO pin must be connected to the programmer SWO pin)
# # 8000000 must match the core clock frequency
monitor tpiu config internal itm.txt uart off 8000000

# # OR: make the microcontroller SWO pin output compatible with UART (8N1)
# # 8000000 must match the core clock frequency
# # 2000000 is the frequency of the SWO pin
# monitor tpiu config external uart off 8000000 2000000

# # enable ITM port 0
monitor itm port 0 on

load

set confirm off

# *try* to stop at the user entry point (it might be gone due to inlining)
break main
continue
