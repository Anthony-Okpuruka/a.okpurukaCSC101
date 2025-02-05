fn main() {

    // mutable array
    let mut colors = ["red","green","yelow","white",];

    println!("\nOriginal array = {:?}",colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}", sliced_colors);

    // change the value of the original slice at the end of the index
    sliced_colors[1] = "purple";

    println!("changed slice {:?}", sliced_colors);
}
