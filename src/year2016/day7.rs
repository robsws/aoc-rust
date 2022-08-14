use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
  let lines = read_lines(input_file_path);
  // Get the amount of lines that support TLS
  let amount = lines.iter().map(|ipv7| ipv7::supports_tls(ipv7) as u32).sum::<u32>();
  println!("{}", amount);
}

pub fn part2(input_file_path: &str) {
  let lines = read_lines(input_file_path);
  let amount = lines.iter().map(|ipv7| ipv7::supports_ssl(ipv7) as u32).sum::<u32>();
  println!("{}", amount);
}

mod ipv7 {
    /// IPv7 string supports TLS if it has an ABBA outside square
    /// brackets and does not have one inside square brackets.
    /// An ABBA is a four-character sequence which is a palindrome,
    /// but the interior characters must not be the same as the exterior.
    /// Assumes ASCII input.
    pub fn supports_tls(ipv7: &str) -> bool {
        let mut in_brackets = false;
        let mut result = false;
        // Assumes ASCII
        let line_bytes = ipv7.as_bytes();
        for i in 0 .. ipv7.len() - 3 {
            let c = line_bytes[i] as char;
            // Keep track of whether cursor is inside or outside
            // square brackets
            match c {
                '[' => { in_brackets = true; continue },
                ']' => { in_brackets = false; continue },
                _ => ()
            }
            // Check for ABBA
            let abba = &line_bytes[i .. i+4];
            if
                abba[0] == abba[3] &&
                abba[1] == abba[2] &&
                abba[0] != abba[1]
            {
                if in_brackets {
                    // If ABBA is inside brackets, string cannot
                    // be TLS compliant
                    return false;
                } else {
                    result = true;
                    // Continue to check for ABBAs inside square
                    // brackets.
                }
            }
        }
        result
    }

    #[test]
    fn supports_tls_abba_outside_brackets() {
        assert!(supports_tls("abba[mnop]qrst"))
    }

    #[test]
    fn supports_tls_abba_outside_brackets_and_inside() {
        assert!(!supports_tls("abcd[bddb]xyyxad"))
    }

    #[test]
    fn supports_tls_abba_interior_characters() {
        assert!(!supports_tls("aaaa[qwer]tyui"))
    }

    #[test]
    fn supports_tls_abba_across_brackets() {
        assert!(!supports_tls("aaab[baer]tyui"))
    }

    #[test]
    fn supports_tls_long_string() {
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"))
    }

    #[test]
    fn supports_tls_at_beginning() {
        assert!(supports_tls("abbadd[asdfgh]zxcvbn"))
    }

    #[test]
    fn supports_tls_at_end() {
        assert!(supports_tls("dd[asdfgh]xxyyx"))
    }

    #[test]
    fn supports_tls_two_abbas() {
        assert!(supports_tls("abbadd[asdfgh]xxyyxbn"))
    }

    #[test]
    fn supports_tls_two_abbas_and_inside() {
        assert!(!supports_tls("abbadd[abba]xxyyxbn"))
    }

    /// Must have an ABA outside square brackets and a
    /// corresponding BAB inside square brackets.
    /// ABA is the same character twice with a different one in the middle.
    /// BAB is the same as ABA but with positions reversed.
    pub fn supports_ssl(ipv7: &str) -> bool {
        let mut in_brackets = false;
        let mut abas = Vec::<[char; 3]>::new();
        let mut babs = Vec::<[char; 3]>::new();
        // Assumes ASCII
        let line_bytes = ipv7.as_bytes();
        for i in 0 .. ipv7.len() - 2 {
            let c = line_bytes[i] as char;
            // Keep track of whether cursor is inside or outside
            // square brackets
            match c {
                '[' => { in_brackets = true; continue },
                ']' => { in_brackets = false; continue },
                _ => ()
            }
            // Check for ABBA
            let aba: [char; 3] = [line_bytes[i] as char, line_bytes[i+1] as char, line_bytes[i+2] as char];
            if
                aba[0] == aba[2] &&
                aba[0] != aba[1]
            {
                if in_brackets {
                    babs.push(aba);
                } else {
                    abas.push(aba);
                }
            }
        }
        // Make sure that there is at least one aba that has a matching bab
        for aba in abas {
            let bab = [aba[1], aba[0], aba[1]];
            if babs.contains(&bab) {
                return true;
            }
        }
        false
    }

    #[test]
    fn supports_ssl_normal() {
        assert!(supports_ssl("aba[bab]xyz"))
    }

    #[test]
    fn supports_ssl_no_corresponding_bab() {
        assert!(!supports_ssl("xyx[xyx]xyx"))
    }

    #[test]
    fn supports_ssl_triple_character() {
        assert!(!supports_ssl("aaa[aaa]eke"))
    }

    #[test]
    fn supports_ssl_triple_character_other_aba() {
        assert!(supports_ssl("aaa[kek]eke"))
    }

    #[test]
    fn supports_ssl_overlapping_abas() {
        assert!(supports_ssl("zazbz[bzb]cdb"))
    }
}