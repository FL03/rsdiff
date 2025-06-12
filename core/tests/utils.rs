/*
    Appellation: utils <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rsdiff_core as rsdiff;

#[cfg(feature = "alloc")]
#[test]
fn test_nested() {
    use alloc::vec::Vec;
    
    use rsdiff::nested;
    let a = Vec::from_iter(0..9);
    let b = Vec::from_iter(0..9);
    let mut res = Vec::new();
    nested!(i in 0..9 => j in 0..9 => { res.push((i, j)) });
    assert_eq!(
        res,
        a.iter()
            .flat_map(|&i| b.iter().map(move |&j| (i, j)))
            .collect::<Vec<_>>()
    );
}
