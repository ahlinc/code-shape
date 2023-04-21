[
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
        body: (_
            "{" @fn.begin
            "}" @fn.end
        )
    )
]
