pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // Comment out the following line and move on to the next exercise.
    // std::mem::size_of::<str>();

    // str, as we just saw, is not Sized.
    // &str is Sized though! We know its size at compile time: two usizes, 
    // - one for the pointer 
    // - one for the length.
}
