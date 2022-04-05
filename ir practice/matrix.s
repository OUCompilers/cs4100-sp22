setframe 0
push Lmain
call
halt
Lmain:
push undef
push 4
push 4
push 1
alloc
alloc
store 2
var 2
push 3
get
push 3
get
store 2
ret
