(def fib (fn (n) (if (eq n 0) 0 (if (eq n 1) 1 (add (fib (sub n 1)) (fib (sub n 2)))))))


(defmacro aif (condition consequent &optional alternate)
  `(let ((it ,condition))
      (if ,it ,consequent ,alternate)))


  (let (
    (x (cadr form))
    (y (caddr form))
    (%tmp (rename 'tmp))
    (%let (rename 'let))
    (%set! (rename 'set!))
    )
    `(,%let ((,%tmp ,x))
       (,%set! ,x ,y)
       (,%set! ,y ,%tmp)))

(let (
      x => (cadr form)
      y => (caddr form)
      %tmp => (rename 'tmp)
      ...
      )
      `(...))



(cond (macro (l) )))

x <- 

(if (truthyP l) (if (truthyP (car l)) (if (eval (car (car l))) (eval (car (cdr (car l)))) (cond (cdr l))) '()) '())

(if (truthyP l) (if (truthyP (car l)) `(if (eval ,(car (car l))) (eval ,(car (cdr (car l)))) (cond ,(cdr l))) '()) '())

l = ((c0 r0) (c1 r1) (c2 r2) ...)

`(if (eval ,(car (car l))) (eval ,(car (cdr (car l)))) (cond ,(cdr l)))

`(if (eval ,(car (car l))) (eval ,(car (cdr (car l)))) (cond ,(cdr l)))

x <- (list 'if (list 'eval (car (car l))) (list 'eval (car (cdr (car l))) (list 'cond (cdr l))))

(cond (macro (l) x))

(cond (macro (l) (list 'if (list 'eval (car (car l))) (list 'eval (car (cdr (car l))) (list 'cond (cdr l))))))

