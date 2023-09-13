use ad_hoc_iterator::{iterate, iterator_from};

fn count_to(n: usize) -> impl Iterator<Item = usize> {
    let mut i = 0;
    iterate! {
        if i < n {
            i += 1;
            Some(i-1)
        } else {
            None
        }
    }
}

fn count_from_to(n: usize, m: usize) -> impl Iterator<Item = usize> {
    let mut i = n;
    iterator_from(move || {
        if i < m {
            i += 1;
            Some(i - 1)
        } else {
            None
        }
    })
}

fn main() {
    for i in count_to(10) {
        println!("{}", i);
    }

    println!("---");

    for i in count_from_to(5, 15) {
        println!("{}", i);
    }
}
