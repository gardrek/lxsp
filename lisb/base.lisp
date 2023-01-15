(def base )

fn (n b)
()


40 - 16 = 24 i = 0
24 - 16 =  8 i = 1
 8 - 16 = -8 i = 2
hex 28

16 * 16 = 256

; 550 - 256 = 294 ; i = 0 ; d = 0
; 294 - 256 =  38 ; i = 1 ; d = 0
;  38 - 256 = neg ; i = 1 ; d = 0
;  38 -  16 =  22 ; i = 2 ; d = 0
;  22 -  16 =   6 ; i = 2 ; d = 0
0x226

we need to provide the max number of digits
d is the number of digits remaining (minus 1) or more accurately, the power to which the current digit is raised
a accumulates the value of each digit

say we want to convert 570 to base 16 with a max of 4 digits
naively we can just do the subtraction and output a zero, or we can have an early out that doesn't output a digit

so for 4 digits we start with d = 3

(digits c b d a)
(digits 570 16 4 0)

stage 1
; d = 3 ; x = 16^d = 16^3 = 16 * 16 * 16 = 4096
; early out test: if the given number is smaller than 4096

; 570 - 256 =  314 ; d = 2 ; a = 0
; 314 - 256 =   58 ; d = 2 ; a = 1
;  58 - 256 = -198 ; d = 2 ; a = 2 - negative result, next digit is 2
;  58 -  16 =   42 ; d = 1 ; a = 0
;  42 -  16 =   26 ; d = 1 ; a = 1
;  26 -  16 =   10 ; d = 1 ; a = 2
;  10 -  16 =   -6 ; d = 1 ; a = 3 - negative result, next digit is 3
; ; 10 / 1 can be a trivial case



(base x b numDigits) = (digits x b (sub numDigits 1) 0)

(digits x b d a)
(digits 570 16 3 0)

y = (pow b d) ; 16 ^ 3 = 4096
(if (lt x y) 
    z = (sub x y)
    ()
(digits x b (sub d 1) a)))


(mul (fn (x y) (if (lt x 1) (y) (add y (mul (sub x 1))))))

(mul (fn (x y) (if (lt x 1) 0 (add y (mul (sub x 1) y)))))

(mul (let ((mulRecurse (fn (x y) (if (lt x 1) 0 (add y (mulRecurse (sub x 1) y)))))) (fn (x y) (if (lt x y) (mulRecurse x y) (mulRecurse y x)))))



(mul (let 
      ((mulRecurse (fn (x y) (if (lt x 1) 0 (add y (mulRecurse (sub x 1) y))))))
      (fn (x y) (if (lt x y) (mulRecurse x y) (mulRecurse y x)))))


    (pow (let 
        ((powRecurse (fn (x y) (if (lt x 1) 0 (mul y (powRecurse (sub x 1) y))))))
        (fn (x y) (if (lt x y) (powRecurse x y) (powRecurse y x)))))

todo:

- make a divrem function that returns both the quotient and the remainder
- make a function that takes a fraction/ratio (as a list of two integers) and reduces it
- approximate the golden ratio using successive fibonacci numbers and the above two functions
- make a decimal function that prints numbers after the decimal point for a fraction


    (div (let ((
        divRecurse
        (fn
            (d v i)
            (if
                (lt (sub d v) 1)
                (if (eq (sub d v) 0) (add i 1) i)
                (divRecurse (sub d v) v (add i 1))))))
        (fn (d v) (divRecurse d v 0))))


    (div (let ((
        divRecurse
        (fn
            (d v i)
            (if
                (lt (sub d v) 1)
                (if (eq (sub d v) 0) (add i 1) i)
                (divRecurse (sub d v) v (add i 1))))))
        (fn (d v) (divRecurse d v 0))))


fn divrem(top bottom) -> (q r) {
    fn divRecurse(d v i) -> (q r) {
        if (d - v) < 1 {
            if (d - v) = 0 {
                i + 1
            } else {
                i
            }
        } else {
            divRecurse((d - v) v (i + 1))
        }
    }

    divRecurse(top bottom 0)
}


