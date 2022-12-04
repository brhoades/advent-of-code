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

maxSum :: [[Int]] -> Int
maxSum = foldl max 0 . map sum

main = do
  file <- readFile "./input.txt"
  putStrLn $ show $ maxSum $ parse file

-- import Prelude hiding (readFile, unwords, lines)
-- import Data.Text hiding (length, map, foldl)
-- import Data.Text.IO
-- import Data.Text.Read

  -- trace ("called splitOn" ++ show $ length elves) (elves)

-- mmap = (map . map)

--parseElves :: Text -> [[Text]]
--parseElves = (map lines) . (splitOn (pack "\n\n"))
--
--main = do
--  file <- readFile "./input.txt"
--  debug $ foldl (+) $ mmap fst $ map rights $ mmap decimal $ parseElves file
--  return ""
--
--  -- trace ("called splitOn" ++ show $ length elves) (elves)
