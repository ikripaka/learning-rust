use num_traits::Num;
use rust_bigint::BigUint;

mod test {
    use num_traits::Num;
    use rust_bigint::BigUint;

    #[test]
    fn parsing() {
        // === HEX ===

        let mut x = "1";
        assert_eq!(get_lower_hex(x), trim_zeros(x));

        x = "";
        assert_eq!(get_lower_hex(x), "0");

        x = "000001";
        assert_eq!(get_lower_hex(x), trim_zeros("1"));

        x = "12345";
        assert_eq!(get_lower_hex(x), trim_zeros(x));

        x = "123456789011223344556677889900aa";
        assert_eq!(get_lower_hex(x), trim_zeros(x));

        x = "4D3C91C579C2C6216567A5\
                241614B561ADDF76C4BB659E6FE7F65FF76\
                A918C843F0458B3EF457BCD9022D78798A2\
                9462EC99C74E6674690267D3E9844251B39D";
        assert_eq!(get_upper_hex(x), trim_zeros(x));

        // === BIN ===

        x = "1";
        assert_eq!(get_binary(x), trim_zeros(x));

        x = "00100000001";
        assert_eq!(get_binary(x), trim_zeros(x));

        // 0x12345
        x = "10010001101000101";
        assert_eq!(get_binary(x), trim_zeros(x));

        // 0x123456789011223344556677889900aa
        x = "10010001101000101011001111000100100000001000100100010001100110100010001010101011001100111011110001000100110010000000010101010";
        assert_eq!(get_binary(x), trim_zeros(x));

        x = "4D3C91C579C2C6216567A5241614B561ADDF76C4BB659E6FE7F65FF76A918C843F0458B3EF457BCD9022D78798A29462EC99C74E6674690267D3E9844251B39D";
        assert_eq!(get_binary_from_hex(x), trim_zeros("01001101001111001001000111000101011110011100001011000110001000010110010101100111101001010010010000010110000101001011010101100001101011011101111101110110110001001011101101100101100111100110111111100111111101100101111111110111011010101001000110001100100001000011111100000100010110001011001111101111010001010111101111001101100100000010001011010111100001111001100010100010100101000110001011101100100110011100011101001110011001100111010001101001000000100110011111010011111010011000010001000010010100011011001110011101"));
    }

    fn get_binary_from_hex(x: &str) -> String {
        format!(
            "{}",
            BigUint::from_str_radix(x, 16).unwrap().to_binary_string()
        )
    }
    fn get_lower_hex(x: &str) -> String {
        format!(
            "{}",
            BigUint::from_str_radix(x, 16)
                .unwrap()
                .to_lower_hex_string()
        )
    }

    fn get_upper_hex(x: &str) -> String {
        format!(
            "{}",
            BigUint::from_str_radix(x, 16)
                .unwrap()
                .to_upper_hex_string()
        )
    }

    fn get_binary(x: &str) -> String {
        format!(
            "{}",
            BigUint::from_str_radix(x, 2).unwrap().to_binary_string()
        )
    }

    #[inline]
    fn trim_zeros<'a>(x: &'a str) -> &'a str {
        x.trim_start_matches("0")
    }
}
