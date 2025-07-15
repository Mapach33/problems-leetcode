/*A word is considered valid if:

It contains a minimum of 3 characters.
It contains only digits (0-9), and English letters (uppercase and lowercase).
It includes at least one vowel.
It includes at least one consonant.

You are given a string word.

Return true if word is valid, otherwise, return false.

Notes:

'a', 'e', 'i', 'o', 'u', and their uppercases are vowels.
A consonant is an English letter that is not a vowel.

Example 1:

Input: word = "234Adas"

Output: true

Explanation:

Enlace:
https://leetcode.com/problems/valid-word/

Casos de Prueba
"234Adas"
"b3"
"a3$e"
*/
struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let mut vocal = false;
        let mut consonat = false;
        if word.len() < 3 {
            false
        } else {
            for c in word.chars() {
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => vocal = true,
                    c if c.is_ascii_digit() => (),
                    c if c.is_ascii_alphabetic() => consonat = true,
                    _ => return false,
                }
            }
            vocal && consonat
        }
    }
}
// prueba
fn main() {
    let prueba = "a3$e".to_string();
    println!("R: {}", { Solution::is_valid(prueba) })
}
