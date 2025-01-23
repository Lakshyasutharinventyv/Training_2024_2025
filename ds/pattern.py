# n = 3
# 1 1 1 1 1 1
# 1 2 2 2 2 1
# 1 2 3 3 2 1
# 1 2 3 3 2 1
# 1 2 2 2 2 1
# 1 1 1 1 1 1

n = 2
# for i in range(n * 2):
#     for j in range(n * 2):
#         print(2 * n - max(i, j, n * 2 - i - 1, n * 2 - j - 1), end=" ")
#     print()

[[print(2 * n - max(i, j, n * 2 - i - 1, n * 2 - j - 1), end=" ") for j in range(n * 2)] and print() for i in range(n * 2)]
