def print_series(n):
    for i in range(1, n + 1):
        # Ascending part
        for j in range(1, i + 1):
            print(j, end=" ")
        # Descending part
        for j in range(i - 1, 0, -1):
            print(j, end=" ")
  
print_series(4)
