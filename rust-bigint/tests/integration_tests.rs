mod test {
    use num_traits::Num;

    use rust_bigint::BigUint;

    #[test]
    fn parsing() {
        let get_hex = |x: &str| -> String{
            format!("{:X?}", BigUint::from_str_radix(x, 16))
        };
        let get_binary = |x: &str| -> String{
            format!("{:X?}", BigUint::from_str_radix(x, 2))
        };

        // === HEX ===
        {
            let x = "1";
            assert_eq!(get_hex(x), x);
        }
        {
            let x = "000001";
            assert_eq!(get_hex(x), x);
        }
        {
            let x = "12345";
            assert_eq!(get_hex(x), x);
        }
        {
            let x = "123456789011223344556677889900aa";
            assert_eq!(get_hex(x), x);
        }

        {
            let x = "4D3C91C579C2C6216567A5\
                241614B561ADDF76C4BB659E6FE7F65FF76\
                A918C843F0458B3EF457BCD9022D78798A2\
                9462EC99C74E6674690267D3E9844251B39D";
            assert_eq!(get_hex(x), x);
        }

        // === BIN ===
        {
            let x = "1";
            assert_eq!(get_hex(x), x);
        }
        {
            let x = "000001";
            assert_eq!(get_hex(x), x);
        }
        {
            let x = "12345";
            assert_eq!(get_hex(x), x);
        }
        {
            let x = "123456789011223344556677889900aa";
            assert_eq!(get_hex(x), x);
        }

        {
            let x = "4D3C91C579C2C6216567A5\
                241614B561ADDF76C4BB659E6FE7F65FF76\
                A918C843F0458B3EF457BCD9022D78798A2\
                9462EC99C74E6674690267D3E9844251B39D";
            assert_eq!(get_hex(x), x);
        }
    }
}
