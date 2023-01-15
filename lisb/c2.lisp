(if (truthy
(def cons2 (fn (x y) (fn (f) (f x y)))))
never

(if (truthy
(def cdr2 (fn (c) (c (fn (x y) (id y))))))
never

(def car2 (fn (c) (c (fn (x y) (id x)))))
)))