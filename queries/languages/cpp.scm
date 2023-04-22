; C++ language function declarations
(declaration
    [
        (function_declarator
            declarator: (identifier) @fn.declaration.name
        )
        (_
            (function_declarator
                declarator: (identifier) @fn.declaration.name
            )
        )
    ]
)

; C++ language function pointer declarations
(declaration
    [
        (init_declarator
            (function_declarator
                (_ (_ declarator: (identifier) @fn.pointer.declaration.name))
            )
        )
        (init_declarator
            (_
                (function_declarator
                    (_ (_ declarator: (identifier) @fn.pointer.declaration.name))
                )
            )
        )
    ]
)

; C++ language function definitions
(function_definition
    [
        (function_declarator
            declarator: (identifier) @fn.name
        )
        (_
            (function_declarator
                declarator: (identifier) @fn.name
            )
        )
    ]
    body: (_) @fn.scope
)
