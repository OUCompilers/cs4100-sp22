setframe 0
push Lmain
call
halt
Lmain:
push undef
push undef
push 3
store 3
push 2
push 2
binary +
store 2
var 2
var 2
binary *
store 2
pop
ret
