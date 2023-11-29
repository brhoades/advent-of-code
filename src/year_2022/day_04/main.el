;;; input contains two number ranges (e.g. 1-3, 5-10)
;;; Count the number of ranges where one is a superset
;;;
;;; there's algos out there for how to approach this sanely. I don't
;;; remember any at 6 AM on a Sunday. A naive approach would check
;;; the bounds of each range against the other.
;;; Doc does not say that equal sets do _not_ count, so checking for equality too.
;;; So:
;;;
;;;   for ranges like A-B,C-D, returning true if one is a superset
;;;     lambda a,b,c,d: (a <= c and b >= d) or (c <= a and d >= b)
;;;
;;;   7 operations per check!
;;;
;;; But thinking on this... my throat is itchy, my knuckles are turning white, and I'm breaking
;;; out into a cold sweat. This smells like something I've seen expressed more nastily.
;;; I suspect this is easly expressed with a non-and/or operator.
;;;
;;; sign(c-a) == sign(b-d)
;;;

(load-file "../../src/lib/common.el")

(defun rng-superset (line)
  ;; "take range strings, A-B,C-D, split them out, and return 1 if one is superset and 0 otherwise"
  (let ((ranges (seq-map 'string-to-number (split-string line "[,-]" line))))
    (let ((A (pop ranges))
          (B (pop ranges))
          (C (pop ranges))
          (D (pop ranges)))
    (if (or (and (<= A C) (>= B D)) (and (<= C A) (>= D B))) 1 0))))

(seq-reduce '+ (seq-map 'rng-superset (read-file-lines-nonil "./input.txt")) 0)
