" Vim syntax file
" Language: Bevel
" Maintainer: Matt Torrence

if exists("b:current_syntax")
	finish
endif

syn region syntaxElementRegion start="{" end="}" fold transparent

syn match   bvNumber		 "\<\(0[bB][0-1]\+\|0[0-7]*\|0[xX]\x\+\|\d\(\d\|_\d\)*\)[lL]\=\>"
syn match   bvNumber		 "\<\d\(\d\|_\d\)*\([eE][-+]\=\d\(\d\|_\d\)*\)\=[fFdD]\>"
syn match   bvNumber		 "\<\d\(\d\|_\d\)*[eE][-+]\=\d\(\d\|_\d\)*[fFdD]\=\>"
syn match   bvNumber		 "\(\<\d\(\d\|_\d\)*\.\(\d\(\d\|_\d\)*\)\=\|\.\d\(\d\|_\d\)*\)\([eE][-+]\=\d\(\d\|_\d\)*\)\=[fFdD]\="
syn match bvComment /%.*$/
syn match bvAtom "'\w"
syn match bvProc "\w\(\w\)*("he=e-1,me=e-1
syn match bvOperator display "\%(+\|-\|/\|*\|=\|>\|<\|\~\)=\?"

syn keyword bvKeyword relate

let b:current_syntax = "bv"

hi def link bvKeyword Keyword
hi def link bvNumber Constant
hi def link bvAtom Constant
hi def link bvProc Function
hi def link bvComment Comment
hi def link bvOperator Operator

set comments="\%"
set commentstring="\% %s"
