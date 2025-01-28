fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    //In this case, the program will crash because it is trying to access memory which may not be valid anymore
    unsafe {
        *ptr = 10;
    }
    println("Value of the first element is {}", v[0]);
}