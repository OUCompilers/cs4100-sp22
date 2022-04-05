setframe 0
push Lmain
call
halt
Lmain:
push 4
push 7
push Lmax
setframe 3
swap
call
ret
Lmax:
var 1
var 0
binary <
push _L1
branch
var 0
push true
push _L2
branch
_L1:
var 1
_L2:
ret
