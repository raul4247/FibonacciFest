n = tonumber(io.read())
print("Hello HacktoberFest!")

function fibs(n) 
  return n < 2 and n or fibs(n - 1) + fibs(n - 2) 
end

for i = 0,n-1,1
do
io.write(fibs(i), " ")
end