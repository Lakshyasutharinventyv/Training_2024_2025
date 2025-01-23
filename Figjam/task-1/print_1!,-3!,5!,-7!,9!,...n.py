n = int(input("Enter a number: "))
id = 3
fact = 1

print(fact)

while id <= (n * n) / 2:
    fact = fact * id * (id - 1) * (-1)
    id = id + 2
    print(fact)