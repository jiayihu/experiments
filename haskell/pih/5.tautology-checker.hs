{-
  Tautology checker
-}

data Prop = Const Bool
  | Var Char
  | Not Prop
  | And Prop Prop
  | Imply Prop Prop

p1 :: Prop
p1 = (Var 'A') `And` (Not (Var 'A'))

p2 :: Prop
p2 = ((Var 'A') `And` (Var 'B')) `Imply` (Var 'A')

p3 :: Prop
p3 = (Var 'A') `Imply` ((Var 'A') `And` (Var 'B'))

p4 :: Prop
p4 = ((Var 'A') `And` ((Var 'A') `Imply` (Var 'B'))) `Imply` (Var 'B')

type Assoc k v = [(k, v)]

type Subst = Assoc Char Bool

find :: Char -> Subst -> Bool
find x s = head [v| (k, v) <- s, x == k]

eval :: Subst -> Prop -> Bool
eval _ (Const b) = b
eval s (Var x) = find x s
eval s (Not p) = not (eval s p)
eval s (And p q) = eval s p && eval s q
eval s (Imply p q) = eval s p <= eval s q

vars :: Prop -> [Char]
vars (Const _) = []
vars (Var x) = [x]
vars (Not p) = vars p
vars (And p q) = vars p ++ vars q
vars (Imply p q) = vars p ++ vars q

rmdups :: Eq a => [a] -> [a]
rmdups [] = []
rmdups (x:xs) = x : filter (/= x) (rmdups xs)

bools :: Int -> [[Bool]]
bools 0 = [[]]
bools n = map (False:) bss ++ map (True:) bss
  where bss = bools (n - 1)

substs :: Prop -> [Subst]
substs p = map (zip vs) (bools (length vs))
  where vs = rmdups (vars p)

isTaut :: Prop -> Bool
isTaut p = and [eval s p | s <- substs p]
