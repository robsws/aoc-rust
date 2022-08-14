use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
  let lines = read_lines(input_file_path);
  // Get the amount of lines that support TLS
  let amount = lines.iter().map(|ipv7| tls::supports_tls(ipv7) as u32).sum::<u32>();
  println!("{}", amount);
}

// pub fn part2(input_file_path: &str) {
//   println!("{}", "Not implemented");
// }

mod tls {
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
}