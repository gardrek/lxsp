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

