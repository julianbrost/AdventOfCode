parse c = map (map read . words) $ lines c

deltas xs = zipWith (-) (tail xs) xs

findPrev xs
    | all (==0) xs = 0
    | otherwise    = (head xs) - (findPrev $ deltas xs)

solve = sum . map findPrev

main :: IO ()
main = do
    contents <- getContents
    let input = parse contents
    putStrLn $ "Part 1: " ++ (show $ solve $ map reverse input)
    putStrLn $ "Part 2: " ++ (show $ solve input)
