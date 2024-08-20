闭包：在 Rust 中，即使一个闭包没有捕获任何外部变量，它仍然被视为闭包。这是因为 Rust 的闭包设计基于它们的语法和潜在能力，而不仅仅是它们当前的行为

不如叫 闭包函数

# into 类型推断

1. 上下文类型推断：
   Rust 编译器通常从上下文中推断 into() 的目标类型。
   `let s: String = "hello".into();`
   在这个例子中，String 类型注解明确告诉编译器 into() 应该返回 String。
2. 变量的后续使用
   ```rust
   let x = "5".into();
   let y: u32 = x + 3;
   ```
   这里，x 的类型会被推断为 u32，因为它后续被用于与 u32 相加。
3. 函数参数

   ```rust
   fn process_string(s: String) {
       // ...
   }

   process_string("hello".into());
   ```

   into() 会被推断为 String，因为函数参数要求 String 类型。

4. 使用 turbofish 语法
   你可以显式指定 into() 的目标类型
   ```rust
   let n = "5".into::<i32>();
   ```
5. 查看实现
   你可以查看类型的 From trait 实现来了解可能的转换
   ```rust
   impl From<&str> for String {
    // ...
   }
   ```
   这表明 &str 可以转换为 String
6. 使用 dbg! 宏：
   `let x = dbg!("hello".into());`
   这会在运行时打印出 x 的类型。
