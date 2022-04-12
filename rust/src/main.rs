use num_integer::Roots;
use std::collections::BinaryHeap;
use std::thread;

// Mersenne primes
// doable
pub const M_31: u128 = (1 << 31) - 1;
pub const M_61: u128 = (1 << 61) - 1;
// not doable
pub const M_89: u128 = (1 << 89) - 1;
pub const M_107: u128 = (1 << 107) - 1;
pub const M_127: u128 = (1 << 127) - 1;

// Just a large number (is not prime because even)
pub const U128_MAX_1: u128 = u128::MAX - 1;

fn print_help() {
    println!("just set your cpu on fire, tries to prime decompose a large mersenne prime (this will probably never succeed)");
    println!("first argument is the amount of threads to spawn");
}

fn is_help(args: &[String]) -> bool {
    args.len() == 0
        || args.iter().any(|x| ["help", "h", "--help", "-h"].contains(&x.as_ref()))
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();

    if is_help(&args) {
        print_help();
        return;
    }

    if let Ok(integer) = args[0].parse::<u8>() {
        let mut handles = Vec::new();
        for _ in 0..integer {
            let handle = thread::spawn(inner);
            handles.push(handle)
        }

        for handle in handles {
            handle.join().unwrap()
        }
    } else {
        print_help();
    }
    
}

fn inner() {
    let result = expand(M_127);
    println!("result {:?}", result);
}

fn expand(n: u128) -> BinaryHeap<u128> {
    let mut list = BinaryHeap::new();
    for i in 2..=(n.sqrt() + 1) {
        if n % i == 0 {
            list.push(i);
            list.extend(expand(n / i));
            return list;
        }
    }

    if n != 1 {
        list.push(n)
    }

    list
}

#[test]
fn expand_works_16() {
    let expected = vec![2, 2, 2, 2];
    let a = expand(16).into_vec();
    assert_eq!(expected, a);
}

#[test]
fn expand_works_120() {
    let expected = vec![5, 3, 2, 2, 2];
    let a = expand(120).into_vec();
    assert_eq!(expected, a);
}

#[test]
fn expand_works_83() {
    let expected = vec![83];
    let a = expand(83).into_vec();
    assert_eq!(expected, a);
}

#[test]
fn expand_works_99() {
    let expected = vec![11, 3, 3];
    let a = expand(99).into_vec();
    assert_eq!(expected, a);
}
