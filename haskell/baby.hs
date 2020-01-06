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

--Given that the argument is printable and a list, return a string
tell :: (Show a) => [a] -> String
tell [] = "Empty"
tell (x:y:[]) = "Theres 2 elements, " ++ show x ++ " and " ++ show y
tell (x:[]) = "Theres one element, " ++ show x
tell (x:y:_) = "Theres a lot of elemts"

length'' :: (Num b) => [a] -> b
length'' [] = 0 
length'' (_:xs) = 1 + length'' xs

--foo :: (Num a, Eq a) => a -> String
--foo ( 3 ) = "You picked 3"
--foo (x>3) = "more than 3"
--foo (x<3) = "more than 3"

foo :: (Integral a, Show a) => a -> String
foo number
    | number == 3 = "That was a three!"
    | number <= 3 = "That was lower than three. THat was a " ++ show number
    | number >= 3 = "That was higher than three."
    | otherwise   = "What???"

capital :: String -> String
capital "" = "Empty str LUL"
capital all@(x:xs) = "First char of " ++ all ++ " is " ++ [x]

----Guards
--bmiTell :: (RealFloat a) => a -> String
--bmiTell bmi
--    | bmi <= 18.5 = "Underweight"
--    | bmi <= 25.0 = "Normal"
--    | bmi <= 30.0 = "Large"
--    | otherwise   = "Very Large"

myComp :: (Ord a) => a -> a -> Ordering
a `myComp` b
    | a > b   = GT
    | a == b  = EQ
    | otherwise = LT

----Guards with where
bmiTell :: (RealFloat a) => a -> a -> String
bmiTell weight height
    | bmi <= 18.5 = "Underweight"
    | bmi <= 25.0 = "Normal"
    | bmi <= 30.0 = "Large"
    | otherwise   = "Very Large"
    where bmi = weight / height ^ 2


cylinder :: (RealFloat a) => a -> a -> a
cylinder r h =
    let sideArea = 2 * pi * r * h
        topArea = pi * r^2
    in  sideArea + 2 * topArea

--Note that `let` bindings are expressions themselves
-- You can use them to define local functions like
--  show [let s x = x*x in (s 5, s 3)] ++ show (s 4)

-- (let f a = a^a in f 4)
--  works too
--  Let bindings are not perfect though because
--  their scope is smallish

--primes = sieve [2..]
--    where sieve (p:xs) = p:sieve [x | x <- xsm x mod p > 0]
--

--There are also case expressions which seem like match

collatz :: (Integral a) => a -> a
collatz n = collatz_help n 0

collatz_help :: (Integral a) => a -> a -> a
collatz_help n i
    | n == 1         = i
    | n `mod` 2 == 0 = collatz_help (n `div` 2) (i+1)
    | n `mod` 2 == 1 = collatz_help (3*n+1) (i+1)

describeList :: [a] -> String
describeList xs = "The list is " ++
    case xs of [] -> "empty."
               [x] -> "A singelton."
	       xs -> "Longer"

