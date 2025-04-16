// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        //         分解步骤：
        // &mut t:

        // 创建一个可变引用（mutable reference）指向变量 t

        // 类型是 &mut u32

        // 这个引用有 Rust 的所有权系统保证的安全性（独占访问）

        // as *mut u32:

        // 将可变引用转换为裸指针（raw pointer）

        // 类型变为 *mut u32

        // 这一步会"抛弃"Rust 的安全保证，变成一个不安全的指针

        // 转换是显式且安全的（引用到裸指针的转换本身不需要 unsafe）

        // as usize:

        // 将裸指针转换为整数类型 usize

        // 这会得到指针指向的内存地址的数值表示

        // 在 64 位系统上通常是 64 位整数
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
