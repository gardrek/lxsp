(
    (lam true)

    (pair (fn (x y) (fn (f) (f x y))))

    (head (fn (c) (fn (x y) x)))

    (tail (fn (c) (fn (x y) y)))

    (nil ())

    (zero (fn (n) ()))

    (one (fn (n) zero))
)
