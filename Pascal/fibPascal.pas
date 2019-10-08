program recursiveFibonacci;
var
   i, N: integer;
function fibonacci(n: integer): integer;

begin
   if n=1 then
      fibonacci := 0
   
   else if n=2 then
      fibonacci := 1
   
   else
      fibonacci := fibonacci(n-1) + fibonacci(n-2);
end; 

begin
   Readln (N);
   for i:= 1 to N do
   
   write(fibonacci (i), '  ');
end.