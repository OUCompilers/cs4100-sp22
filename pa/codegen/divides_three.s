setframe 0
push Lmain
call
halt
Lmain:
push 9
push Ldivisiblebythree
setframe 2
swap
call
ret
Ldivisiblebythree:
var 0
push 3
push 3
var 0
binary /
binary *
binary ==
push _L1
branch
push false
push true
push _L2
branch
_L1:
push true
_L2:
ret
