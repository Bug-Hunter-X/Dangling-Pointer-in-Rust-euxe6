fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of creating mutable pointer, we can directly access and modify the elements of the vector using indexing
    v[0] = 10;
    println("Value of the first element is {}", v[0]);
}
