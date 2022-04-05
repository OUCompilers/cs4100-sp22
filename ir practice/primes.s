setframe 0
push Lmain
call
halt
Lmain:
push 17
push 16
push Lisprime
setframe 3
swap
call
ret
Lisprime:
push 1
var 1
binary ==
push _L1
branch
var 0
var 1
var 1
var 0
binary /
binary *
binary ==
push _L3
branch
var 0
push 1
var 1
binary -
push Lisprime
setframe 3
swap
call
push true
push _L4
branch
_L3:
push false
_L4:
push true
push _L2
branch
_L1:
push true
_L2:
ret
