p = putStrLn . show

-- Take the contents of input, split them , group them up by empty newline,
-- and finally take the maximum of sums.
solve :: String -> Int
solve = foldl max 0 . sumLn [] . lines
  -- return an array of ints. sum numbers until we get a newline, then begin a new sum
  where sumLn acc []                = acc
        sumLn [] (x:xs)             = sumLn [read x] xs
        sumLn acc ("":xs)           = sumLn (0 : acc) xs
        sumLn (a:as) (x:xs)         = sumLn ((read x + a) : as) xs

main = p =<< solve <$> readFile "./input.txt"
