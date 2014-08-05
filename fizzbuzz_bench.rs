//Test the performace of a C like fizzbuzz implementation vs a more rustic one.
//Rustic implementation stolen from:
//http://www.scriptingforsport.com/playing-with-tdd-fizzbuzz-in-rust/
//by Sami Elahmadie of http://samielahmadie.me

extern crate test;

mod rustic {
    pub fn fizzbuzz(num: int) -> String {
        match ( num % 3 , num % 5) {
            (0, 0) => format!("Fizzbuzz"),
                (0, _) => format!("Fizz"),
                (_, 0) => format!("Buzz"),
                (_, _) => num.to_string()
        }
    }
}

mod imperative {
    pub mod simplified {
        pub fn fizzbuzz(num: int) -> String {
            if num % 15 == 0 {
                format!("Fizzbuzz")
            } else if num % 3 == 0 {
                format!("Fizz")
            } else if num % 5 == 0 {
                format!("Buzz")
            } else {
                num.to_string()
            }
        }
    }

    pub mod original {
        pub fn fizzbuzz(num: int) -> String {
            if (num % 3 == 0) && (num % 5 ==  0) {
                format!("Fizzbuzz")
            } else if num % 3 == 0 {
                format!("Fizz")
            } else if num % 5 == 0 {
                format!("Buzz")
            } else {
                num.to_string()
            }
        }
    }

    pub mod func_call {
        fn div_by_three(num: int) -> bool {
            num % 3 == 0
        }

        fn div_by_five(num: int) -> bool {
            num % 5 == 0
        }

        pub fn fizzbuzz(num: int) -> String {
            if div_by_three(num) && div_by_five(num) {
                format!("Fizzbuzz")
            } else if div_by_three(num) {
                format!("Fizz")
            } else if div_by_five(num) {
                format!("Buzz")
            } else {
                num.to_string()
            }
        }
    }
}

#[main]
fn main() {
    for i in range(0i, 101) {
        println!("{}:\t{}\t{}\t{}\t{}", i,
                rustic::fizzbuzz(i),
                imperative::original::fizzbuzz(i),
                imperative::func_call::fizzbuzz(i),
                imperative::simplified::fizzbuzz(i)
                );
    }
}

#[cfg(test)]
mod testee {

    use test::Bencher;

    use rustic::fizzbuzz;
    use imperative::{original, func_call, simplified};

    static BENCH_SIZE: int = 4096;

#[bench]
    fn bench_rustic(b: &mut Bencher) {
        b.iter(|| { for i in range(0, BENCH_SIZE) { fizzbuzz(i); }});
    }
#[bench]
    fn bench_c(b: &mut Bencher) {
        b.iter(|| { for i in range(0, BENCH_SIZE) { original::fizzbuzz(i); }});
    }
#[bench]
    fn bench_cfunc(b: &mut Bencher) {
        b.iter(|| { for i in range(0, BENCH_SIZE) { func_call::fizzbuzz(i); }});
    }
#[bench]
    fn bench_cs(b: &mut Bencher) {
        b.iter(|| { for i in range(0, BENCH_SIZE) { simplified::fizzbuzz(i); }});
    }
}
