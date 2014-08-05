//Test the performace of a C like fizzbuzz implementation vs a more rustic one.
//Rustic implementation stolen from:
//http://www.scriptingforsport.com/playing-with-tdd-fizzbuzz-in-rust/
//by Sami Elahmadie of http://samielahmadie.me

#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
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
    use std::rand;
    use std::rand::distributions::{IndependentSample, Range};
    use test::Bencher;

    use rustic::fizzbuzz;
    use imperative::{original, func_call, simplified};

    static BENCH_SIZE: int = 4096;

    //regex example also taken from Sami, but modified to take function
    fn regex_tester(fzbz: fn(int) -> String) {
        let between = Range::new(1i, 1000);
        let mut rng = rand::task_rng();
        let regex_fizz = regex!(r"Fizz");
        let regex_buzz = regex!(r"[Bb]uzz");
        let regex_fizzbuzz = regex!(r"Fizzbuzz");

        for _ in range(0i, 50) {
            assert!(regex_fizz.is_match(fzbz(3 * between.ind_sample(&mut rng)).as_slice()));
            assert!(regex_buzz.is_match(fzbz(5 * between.ind_sample(&mut rng)).as_slice()));
            assert!(regex_fizzbuzz.is_match(fzbz(15 * between.ind_sample(&mut rng)).as_slice()));
        }
    }

#[test]
    fn rustic() {
        regex_tester(fizzbuzz);
    }

#[test]
    fn original() {
        regex_tester(original::fizzbuzz);
    }

#[test]
    fn cfunc(){
        regex_tester(func_call::fizzbuzz);
    }

#[test]
    fn simplified() {
        regex_tester(simplified::fizzbuzz);
    }

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
