if exists("b:current_syntax")
    finish
endif

syntax match notesTerminal "\v\$.*"
syntax region notesBacktick start=+`+ end=+`+
syntax region notesDanger start=+#+ end=+#+
syntax region notesScript start=+@+ end=+@+


highlight link notesTerminal Keyword
highlight link notesBacktick Tag
highlight link notesDanger Todo
highlight link notesScript Macro

let b:current_syntax = "notes"
