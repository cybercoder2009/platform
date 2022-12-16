use skia_safe::FontMgr;

fn main() {
    let fm: FontMgr = FontMgr::default();
    let count: usize = fm.count_families();
    println!("families count = {}", count);
    for i in 0 .. count {
        println!("{} {}", i, fm.family_name(i));
    }
}
