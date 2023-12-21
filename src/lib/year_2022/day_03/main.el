(load-file "../../src/lib/common.el")

(defun string-intersection (l r)
    "returns the single char in common in two strings. O(n) w/ hashmap"
    ;; table is for l, used in lookups to r
    (let ((table (make-hash-table
                         :test 'equal
                         :size (length l))))
    (seq-map (lambda (x) (puthash x t table)) (split-string l "" t))
    ;; finally, find the first element and return it
    (seq-find (lambda (x) (equal t (gethash x table))) (split-string r "" t))))

(defun score-char (c)
  (if (equal (downcase c) c)
      (+ 1 (- (string-to-char c) (string-to-char "a")))
      (+ 27 (- (string-to-char c) (string-to-char "A")))))

(defun halve-string (l)
  "halves the string and returns both halves in a list"
  (let ((midpoint (/ (length l) 2)))
    (list (substring l 0 midpoint) (substring l midpoint))))

(defun bag-score (l)
  (let ((halves (halve-string l)))
  (score-char (string-intersection (pop halves) (pop halves)))))


(seq-reduce (lambda (acc rd) (+ acc (bag-score rd))) (read-file-lines-nonil "./input.txt") 0)
