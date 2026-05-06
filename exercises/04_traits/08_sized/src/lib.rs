pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.

    /*
    因為 str 是 DST（Dynamically Sized Type），大小在編譯期不知道。
     size_of::<str>()  // 錯誤！編譯期不知道多大
    size_of::<&str>() // OK，16 bytes（pointer + length）
    size_of::<String>() // OK，24 bytes（固定）
    str 代表「某段字串內容」，長度取決於實際內容：

    "hi" → 2 bytes
    "hello" → 5 bytes
    編譯期根本不知道你要存哪個，所以無法算大小。

    size_of 需要編譯期就知道大小，所以對 str 直接報錯。

    這就是為什麼你永遠不會看到裸的 str，只會看到 &str——透過 reference 把大小固定成 16 bytes（pointer + length）。

    */
    // std::mem::size_of::<str>();
}
