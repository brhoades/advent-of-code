(defun read-file-lines-nonil (f)
  "split file by new line and remove null lines"
  (with-temp-buffer
    (insert-file-contents f)
    (split-string
     (save-restriction
       (widen)
       (buffer-substring-no-properties
        (point-min)
        (point-max)))
     "\n" t)))

(defun read-file (f)
  "read and return a file's contents"
  (with-temp-buffer
    (insert-file-contents f)
     (save-restriction
       (widen)
       (buffer-substring-no-properties
        (point-min)
        (point-max)))))
