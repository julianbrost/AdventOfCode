parse c = map (map read . words) $ lines c

deltas xs = [b-a | (a, b) <- zip xs $ tail xs]

findNext xs
    | all (==0) xs = 0
    | otherwise    = (last xs) + (findNext $ deltas xs)

solve1 = sum . map findNext

findPrev xs
    | all (==0) xs = 0
    | otherwise    = (head xs) - (findPrev $ deltas xs)

solve2 = sum . map findPrev

main :: IO ()
main = do
    contents <- getContents
    let input = parse contents
    putStrLn $ "Part 1: " ++ (show $ solve1 input)
    putStrLn $ "Part 2: " ++ (show $ solve2 input)
