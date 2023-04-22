class Foo:
    def foo():
        pass

    def bar():
        def inner():
            pass

def one():
    pass

def two():
    pass

def wrap():
    class Baz:
        class Bar:
            class Foo:
                def func1():
                    def func2():
                        pass

    def three():
        pass

def four(): pass
