--
-- Alternative implementation which uses maybes and seems a bit
-- less convoluted
--

maxSum :: [[Int]] -> Int
maxSum = foldl max 0 . map sum

parseLine :: String -> Maybe Int
parseLine "" = Nothing
parseLine x = Just $ read x

-- make list elements separated by Nothing into nested lists
group :: [[Int]] -> [Maybe Int] -> [[Int]]
group acc []                = acc
group [] (Just x:xs)        = group [[x]] xs
group (a:as) (Just x:xs)    = group ((x : a) : as) xs
group acc (Nothing:xs)      = group ([] : acc) xs

-- parse input file into lists of ints
parse :: String -> [[Int]]
parse = group [] . map parseLine . lines
