print("Hello HacktoberFest!")

n = int(input("Digite o valor desejado: "))

fib1 = 0
fib2 = 1

for i in range (0, n):
    print(fib1 , end = " ")
    
    tmp = fib1
    fib1 = fib2
    fib2 += tmp
    
print("\nFim do programa.")