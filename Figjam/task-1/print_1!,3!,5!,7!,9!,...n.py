n = int(input())
id = 3
fact = 1

while id <= (n*n)/2:
    fact = fact * id * (id-1)
    id = id + 2
    print(fact)