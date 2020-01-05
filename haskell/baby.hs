--This is from the "Learn You A Haskell" book
--
doubleMe x = x + x
doubleUs x y = 2*(x+y)
doubleUs' x y = doubleMe x + doubleMe y

doubleSmallNumber x = if x > 100
                        then x
                        else x*2

doubleSmallNumber' x = (if x > 100 then x else 2*x) + 1


--[x | x <- [2..], minimum [x `mod` y | y <- [1..x-1]] /= 0]
----
----But you can do it better with
----
----[x | x <- [2..], all (/= 0) [x `mod` y | y <- [2..x-1]]]
----
----because this checks if any element is 0, so the empty
----list is okay
--
length' xs = sum [1 | a <- xs]

remUpper xs = [y | y <- xs, (y `elem` ['A'..'Z']) == False]


--[x | x <- [2..] | (all (/= 0) [x `mod` y | y <- [2..x-1]])]

addThree :: Int -> Int -> Int -> Int
addThree x y z = x + y + z

factorial :: Integer -> Integer
factorial n = product [1..n]

--sdf :: Int -> Int
--sdf x = if x < 0 then sdf (x-10000000) else x
--
--pow :: (Num a, Num t, Enum t) => a -> t -> a
--Not sure what the type declaration should be here
pow a b = product [a | _ <- [1..b]]

lucky :: (Integral a) => a -> String
lucky 7 = "GOOD GUESS MY DUDE"
lucky x = "Failure"

fact' :: (Integral a) => a -> a
fact' 0 = 1
fact' n = n * fact' (n-1)

--Remember that a is just a type
addVec :: (Num a) => (a, a) -> (a, a) -> (a, a)
addVec a b = (fst a + fst b, snd a + snd b)

--You can also do it like this to remove fst and snd stuff
addVec' :: (Num a) => (a, a) -> (a, a) -> (a, a)
addVec' (x1, y1) (x2, y2) = (x1+x2, y1+y2)

head' :: [a] -> a
head' [] = error "Can't check head of empty list"
head' (x:_) = x

tail' :: [a] -> [a]
tail' [] = error "Can't check tail of empty list"
tail' (x:xs) = xs


