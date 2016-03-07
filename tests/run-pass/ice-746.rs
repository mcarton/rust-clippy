#![feature(plugin)]
#![plugin(clippy)]
#![deny(clippy)]

#![allow(no_effect)]

static UNZIGZAG: [usize; 4] = [ 0,  1,  8, 16 ];

fn main() {
    for j in 0..4 {
        UNZIGZAG[j];
    }
}
