x = input("x: ")
y = input("y: ")
z = input("z: ")

if x >= y and x >= z:
    max_val = x
elif y >= x and y >= z:
    max_val = y
else:
    max_val = z

print(max_val)
