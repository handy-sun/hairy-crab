fn round_up(n: u32, to_align: u32) -> u32 {
    if to_align == 0 {
        return n;
    }
    let alignment = to_align - 1;
    // let original = n;
    // (n + alignment) & !alignment
    ((n + alignment) / to_align) * to_align
}
 
// fn main() {
//     let n = 13;
//     let to_align = 8;
//     println!("round_up({}, {}) = {}", n, to_align, round_up(n, to_align));

//     let n = 5;
//     let to_align = 8;
//     println!("round_up({}, {}) = {}", n, to_align, round_up(n, to_align));

//     let n = 1066;
//     let to_align = 100;
//     println!("round_up({}, {}) = {}", n, to_align, round_up(n, to_align));
// }
