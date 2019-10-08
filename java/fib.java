import java.util.Scanner;

public class fib{
	public static void main(String[]args){
		int n, fib1=0,fib2=1;
		Scanner sc = new Scanner(System.in);
		System.out.println("Hello Hacktoberfest");
		System.out.println("Digite o valor de n");
		n = sc.nextInt();
		for(int i=0;i<n;i++){
			int tmp;
			System.out.print(fib1+" ");
			
			tmp=fib1;
			fib1=fib2;
			fib2+=tmp;
		}
		
	}
}