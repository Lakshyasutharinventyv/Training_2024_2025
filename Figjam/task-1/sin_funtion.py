n = int(input())
x = int(input())
count = 1
i = 1
mul = 1
sum = 0
fact = 1
K = 1

while count <= n:
    K = i
    pow = 1
    fact = 1
    k = 1

    for j in range(1, i+1):
        pow *= x
        fact *= k
        k += 1

    sum += mul * (pow / fact)
    i += 2
    count += 1
    mul *= -1
    k += 1

print(sum)