use mini_natural_sort::NaturalSortKey;

fn main() {
    println!(
        "{}",
        NaturalSortKey::from("0000001") < NaturalSortKey::from("2")
    );
}
