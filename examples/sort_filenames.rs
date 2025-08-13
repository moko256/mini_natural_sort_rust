use mini_natural_sort::NaturalSortKey;

fn main() {
    let mut filenames = [
        "b001.txt",
        "b1.txt",
        "b000002.txt",
        "A.txt",
        "a.txt",
        "c.txt",
        "c_copy (1).txt",
    ];
    println!("Before: {:?}", filenames);

    filenames.sort_by_key(|v| NaturalSortKey::from_str(v));
    println!("After: {:?}", filenames);
}
