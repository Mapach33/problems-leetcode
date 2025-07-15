struct Solution1;

impl Solution1 {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        if strs.is_empty() {
            return prefix;
        }
        let first_str = &strs[0];
        for (i, c) in first_str.chars().enumerate() {
            for s in &strs[1..] {
                if i >= s.len() || s.chars().nth(i) != Some(c) {
                    return prefix;
                }
            }
            prefix.push(c);
        }
        prefix
    }
}
fn main() {
    let palabras = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let resultado = Solution1::longest_common_prefix(palabras);
    println!("Prefijo comun mas largo: {resultado}")
}
