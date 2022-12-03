(load-file "../lib/common.el")

(setq score-table #s(hash-table
                       test equal
                       size 9
                       data (
                             ;; A / X = rock, B / Y = paper, C / Z = scissors
                             ;; points: loss = 0, draw = 3, victory = 6
                             ;; points: rock = 1, paper = 2, scissors = 3
                             "A X" 4
                             "B X" 1
                             "C X" 7
                             "A Y" 8
                             "B Y" 5
                             "C Y" 2
                             "A Z" 3
                             "B Z" 9
                             "C Z" 6)))


(seq-reduce (lambda (acc rd) (+ acc (gethash rd score-table))) (read-file-lines-nonil "./input.txt") 0)
