; C language function declarations
(declaration
    [
        (function_declarator
            declarator: (identifier) @fn.name
        )
        (init_declarator
            (_
                (function_declarator
                    (_ (_ declarator: (identifier) @fn.name))
                )
            )
        )
    ]
)

; C language function definitions
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
