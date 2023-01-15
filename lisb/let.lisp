(fn (bindings) (
                (car (car bindings)) => 'x i think
                (eval (car (cdr bindings))) => 5
))

(let ((x 5) (y 10)) (add x y))

S = ^x.^y.^z.x(z)(y(z))
K = ^x.^y.x
U = ^x.x(S)(K)


S = ^xyz.x(z)(y(z))
K = ^xy.x
U = ^x.x(S)(K)

