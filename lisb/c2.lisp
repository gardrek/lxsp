(
    (cons2 (fn (x y) (fn (f) (f x y))))
    (cdr2 (fn (c) (c (fn (x y) (id y)))))
    (car2 (fn (c) (c (fn (x y) (id x)))))
)
