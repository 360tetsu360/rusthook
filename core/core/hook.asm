.code

; [.data]
message_str db 'Assembly', 0
format_str db 'Hello World from %s!', 0

; [.text]
extrn printf:proc

helloworld proc

lea rdx, message_str
lea rcx, format_str
jmp printf

helloworld endp

end