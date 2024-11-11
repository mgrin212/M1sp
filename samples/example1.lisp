(define (myPlus x y) (+ x (+ y 1)))
(define (fib n) 
  (if (< n 2) 
    n
    (+ (fib (- n 1)) (fib (- n 2)))))

(let ((x 7)) (fib x))
