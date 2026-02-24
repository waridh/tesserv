(define (fib n)
  (define (iter lower upper acc)
    (let ((next (+ lower upper)))
      (if (>= acc n)
          next
          (iter upper next (+ acc 1)))))
  (cond
   ((= n 0) 0)
   ((= n 1) 1)
   (else (iter 0 1 2))))
