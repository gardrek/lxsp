Axioms
    (quote x)
        Returns the argument without evaluating it.
    (if cond then else)
        if `cond` evaluates true, `then` is evaluated and returned. Otherwise `else` is evaluated and returned.
    (car list)   (first list)
        Returns the first item of a list or the first character of a symbol.
    (cdr list)   (rest list)
        Returns all but the first item of a list, or all but the first character of a symbol.
    (cons first rest)
        Returns a list or symbol where the first element is the first argument and the remainder is the second argument. (cons ’a ’(b c)) will return (a b c), and (cons ’a ’bc) will return abc.
    (atom x)
        Returns the argument if the argument is an atom, otherwise the empty list

arithmetic quasi-axioms
    (sub a b)
        Returns `a` minus `b`.
    (lt a b)
        Returns truth if `a` is less than `b`.

NOTE:
(eq a b) = (not (or (lt a b) (lt b a)))
(not x) = (if x () 't)
(or a b) = (if a a b)

NOTE: Should symbols be encoded as a list of bytes of UTF-8 encoding, or a list of Unicode codepoints? In the ASCII realm these are equivalent so for now we remain ASCII-only and delay the decision, with the codepoints being generic integers rather than specifically 8-bit or 32-bit. Although, since they are limited to ASCII they are essentially 7-bit integers. Actually another detail, instead of linked lists they should probably be vectors. So rather than carving out a special case for list manipulation functions over *symbols* we have one over *vectors*.

I/O
    tbd


Types
    List
        A generic list expression, eg. (a b c).
    Symbol
        A symbol or string. This can include any character that is not a read macro, a bracket or a space.
        NOTE: Why can't the type hold those characters? Of course they would need some special syntax to embed directly in source, but that's not necessary. If the characters are internally integers then any character can be inserted using a bit of math.
    Number
        An integer or float.
        NOTE: We don't necessarily need to implement floats immediately.
    Function
        A built-in Lisp function which is defined internally as a function in the host language.
    Lambda
        An anonymous function, which is applied by creating a closure and performing search and replace rules on the function’s body.
    Macro
        Very similar to a lambda, but the body is interpreted (macroexpanded) in a dynamic environment rather than a lexical one with unevaluated arguments, and the result is then interpreted again in the current environment.




(def cons (fn (x y) (fn (f) (f x y))))
(def car (fn (c) (c (fn (x y) (id x))))
(def cdr (fn (c) (c (fn (x y) (id y))))

cons ← λxy.λf.f(xy)
car ← λc.c(λxy.x)
cdr ← λc.c(λxy.y)


