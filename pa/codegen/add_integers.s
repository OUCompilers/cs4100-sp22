setframe 0
push Lmain
call
halt
Lmain:
push undef
push 4
push 10
alloc
store 2
var 2
push 3
push Lsum
setframe 3
swap
call
store 2
ret
Lsum:
push -1
var 1
binary ==
push _L1
branch
var 0
push 1
var 1
binary -
push Lsum
setframe 3
swap
call
var 0
var 1
get
binary +
push true
push _L2
branch
_L1:
push 0
_L2:
ret
