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
