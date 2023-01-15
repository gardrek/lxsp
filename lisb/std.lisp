'(
    (std true)

    (pair cons)

    (head car)

    (tail cdr)

    (nil ())

    (id (fn (x) x))

    (div (let '((
        divRecurse
        (fn
            (d v i)
            (if
                (lt (sub d v) 1)
                (if (eq (sub d v) 0) (add i 1) i)
                (divRecurse (sub d v) v (add i 1))))))
        (fn (d v) (divRecurse d v 0))))

    (fib (fn (n) (if (eq n 0) 0 (if (eq n 1) 1 (add (fib (sub n 1)) (fib (sub n 2)))))))

    (nilP (fn (x) (if (eq x ()) true false)))

    (not (fn (x) (if x false true)))

    (truthyP (fn (x) (if (eq x false) false (not (nilP x)))))

    (longOr (fn (x y) (if (truthyP x) x y)))

    (mul (let 
        '((mulRecurse (fn (x y) (if (lt x 1) 0 (add y (mulRecurse (sub x 1) y))))))
        (fn (x y) (if (lt x y) (mulRecurse x y) (mulRecurse y x)))))

    (pow (let 
        '((powRecurse (fn (x y) (if (lt x 1) 1 (mul y (powRecurse (sub x 1) y))))))
        (fn (x y) (powRecurse x y))))

    (firsts (fn (l) (if (truthyP l) (cons (car (car l)) (firsts (cdr l))) ())))

    (seconds (fn (l) (if (truthyP l) (cons (car (cdr (car l))) (seconds (cdr l))) ())))
)
