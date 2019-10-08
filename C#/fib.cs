//Rextester.Program.Main is the entry point for your code. Don't change it.
//Compiler version 4.0.30319.17929 for Microsoft (R) .NET Framework 4.5

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace Rextester
{
    public class Program
    {
        public static void Main(string[] args)
        {
                        
            int n, fib1 = 0, fib2 = 1;
            n = Convert.ToInt32(Console.ReadLine());

            Console.WriteLine ("Hello HacktoberFest!\n");

            for (int i = 0; i < n; i++)
            {
                int tmp;
                Console.WriteLine ("" + fib1);

                tmp = fib1;
                fib1 = fib2;
                fib2 += tmp;
            }
            Console.WriteLine ("\n");

        }
    }
}