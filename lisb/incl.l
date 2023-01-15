(
    (std true)

    (head car)

    (tail cdr)

    (id (fn (x) (do x)))

    (div (let '((
        divRecurse
        (fn
            (d v i)
            (if
                (lt (sub d v) 1)
                (if (eq (sub d v) 0) (add i 1) i)
                (divRecurse (sub d v) v (add i 1))))))
        (fn (d v) (divRecurse d v 0))))

    (pow (let 
        '((powRecurse (fn (x y) (if (lt x 1) 1 (mul y (powRecurse (sub x 1) y))))))
        (fn (x y) (powRecurse x y))))

    (fib (fn (n) (if (eq n 0) 0 (if (eq n 1) 1 (add (fib (sub n 1)) (fib (sub n 2)))))))

    (truthy (fn (x) (if (eq x ()) false true)))

    (truthyPlus (fn (x) (if (eq x false) false (truthy x))))

    (not (fn (x) (if (truthyPlus x) false true)))

    (longOr (fn (x y) (if (truthyPlus x) x y)))

    (mul (let 
        '((mulRecurse (fn (x y) (if (lt x 1) 0 (add y (mulRecurse (sub x 1) y))))))
        (fn (x y) (if (lt x y) (mulRecurse x y) (mulRecurse y x)))))
)
