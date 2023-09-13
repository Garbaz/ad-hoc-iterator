use ad_hoc_iterator::iterate;

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

fn main() {
    for i in count_to(10) {
        println!("{}", i);
    }
}
