(defun read-file-lines (f)
  "splits each string twice, once by separate elves and again by calories... \n alone kills null entries"
  (with-temp-buffer
    (insert-file-contents f)
    (seq-map
     (lambda (x) (split-string x "\n"))
     (split-string
      (save-restriction
        (widen)
        (buffer-substring-no-properties
         (point-min)
         (point-max)))
      "\n\n"))))

(defun parse-elves (elves)
  "[[string]] => [int], sum all nested arrays"
  (seq-map (lambda (x) (seq-reduce (lambda (acc e) (+ (string-to-number e) acc)) x 0)) elves))


(defun max (nums)
  "returns the largest number in nums or 0 if empty"
  (seq-reduce (lambda (acc e) (if (> e acc) e acc)) nums 0))

(defun solve (f)
  "gets the maximum sum of the elves"
  (max (parse-elves (read-file-lines f))))


(solve (expand-file-name "./input.txt"))
