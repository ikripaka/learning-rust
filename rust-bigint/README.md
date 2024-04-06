# CryptoLibRust

Minimal Rust version -- 1.77.1

## Щоб імпортувати бібліотеку:
1. Копіюєте теку `/rust-bigint` і вставляєте у корінь вашого проєкту поряд із `src`.  <br />
   Тобто повинно вийти ось так:
    ```console
        user@device:~/PathToProjectFolder$ ls
            src
            rust-bigint
            Cargo.toml
   ```
    + можна [ось тут](https://blog.pnkfx.org/blog/2022/05/12/linking-rust-crates/) глянути як саме бібліотеки взаємопов'язуються між собою (тут використовуємо `static rlib linked with Rust` рис. 2 починаючи згори).

2. У `Cargo.toml` до вашого проєкту треба додати до `[dependencies]` залежностей <br />`rust-bigint = {path = "назва_підпапки де знаходиться бібліотека"}`, <br /> але у нашому випадку це:
    ```TOML
        [dependencies]
        cryptolib = {path = "/rust-biguint"}
    ```
   
time spent: 3 + 4 + 1 + 1.5 + 1.5
