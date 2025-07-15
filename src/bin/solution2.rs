struct Solution2;

impl Solution2 {
    pub fn is_valid(s: String) -> bool {
        let mut pila = Vec::new();
        for i in s.chars() {
            if i == '(' || i == '[' || i == '{' {
                pila.push(i);
            } else if i == ')' || i == ']' || i == '}' {
                pila.pop();
            }
        }
        pila.is_empty()
    }
}
fn main() {
    let ejemplos = vec![("()", true), ("()[]{}", true)];
    for (s, esperado) in ejemplos {
        let resultado = Solution2::is_valid(s.to_string());
        println!(
            "Input: {:<7} | Output: {:<5} | Esperado: {}",
            s, resultado, esperado
        );
    }
}
