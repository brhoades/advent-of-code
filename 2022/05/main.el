(load-file "../../src/lib/common.el")

(defun read-input (f)
  "read-input splits the input into two sections: the stacks and a list of orders"
  (let*
    ((f (split-string (read-file f) "\n\n"))
     (stack (seq-map (lambda (x) (split-string x "" t)) (split-string (pop f) "\n")))
     (orders (split-string (pop f) " \n")))
     (list stack orders)
    ))

(defun pop-all (l)
  "pop-all takes a square list of lists, l, and recursively accumulates (acc) it, transposing it"
  (if (null (car l))
      nil
    (let* (
           (row (seq-map (lambda (x) (pop x)) l)) ; the row items are first
           (rem (seq-map (lambda (x) (progn (pop x) x)) l))) ; the remaining lists are last
      (append row (list (pop-all rem))))))

(defun transpose (l) (pop-all l))

(let (input (read-input "input-2.txt"))
  (transpose (pop input)))

(transpose (car-safe (read-input "input-2.txt")))

(car-safe (read-input "input-2.txt"))
