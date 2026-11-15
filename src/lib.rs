//! Formula and equation editor for SigmaOS (LibreOffice Math equivalent).

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self, vars: &HashMap<String, f64>) -> Result<f64, String> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Variable(v) => vars
                .get(v)
                .copied()
                .ok_or_else(|| format!("Undefined variable: {}", v)),
            Expr::Add(a, b) => Ok(a.evaluate(vars)? + b.evaluate(vars)?),
            Expr::Sub(a, b) => Ok(a.evaluate(vars)? - b.evaluate(vars)?),
            Expr::Mul(a, b) => Ok(a.evaluate(vars)? * b.evaluate(vars)?),
            Expr::Div(a, b) => {
                let divisor = b.evaluate(vars)?;
                if divisor == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(a.evaluate(vars)? / divisor)
            }
            Expr::Pow(base, exp) => Ok(base.evaluate(vars)?.powf(exp.evaluate(vars)?)),
        }
    }

    pub fn to_string_repr(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Variable(v) => v.clone(),
            Expr::Add(a, b) => format!("({} + {})", a.to_string_repr(), b.to_string_repr()),
            Expr::Sub(a, b) => format!("({} - {})", a.to_string_repr(), b.to_string_repr()),
            Expr::Mul(a, b) => format!("({} * {})", a.to_string_repr(), b.to_string_repr()),
            Expr::Div(a, b) => format!("({} / {})", a.to_string_repr(), b.to_string_repr()),
            Expr::Pow(base, exp) => {
                format!("({} ^ {})", base.to_string_repr(), exp.to_string_repr())
            }
        }
    }
}

pub struct FormulaEditor {
    formulas: Vec<(String, Expr)>,
}

impl FormulaEditor {
    pub fn new() -> Self {
        Self {
            formulas: Vec::new(),
        }
    }

    pub fn add_formula(&mut self, name: &str, expr: Expr) {
        self.formulas.push((name.to_string(), expr));
    }

    pub fn evaluate_formula(&self, name: &str, vars: &HashMap<String, f64>) -> Result<f64, String> {
        self.formulas
            .iter()
            .find(|(n, _)| n == name)
            .ok_or_else(|| format!("Formula '{}' not found", name))
            .and_then(|(_, expr)| expr.evaluate(vars))
    }

    pub fn formula_count(&self) -> usize {
        self.formulas.len()
    }
}

impl Default for FormulaEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_expression_aaa() {
        // Arrange
        let expr = Expr::Add(
            Box::new(Expr::Number(3.0)),
            Box::new(Expr::Mul(
                Box::new(Expr::Number(2.0)),
                Box::new(Expr::Number(4.0)),
            )),
        );
        let vars = HashMap::new();

        // Act
        let result = expr.evaluate(&vars).unwrap();

        // Assert
        assert!((result - 11.0).abs() < 1e-10);
    }

    #[test]
    fn test_add_formula_aaa() {
        // Arrange
        let mut editor = FormulaEditor::new();
        let expr = Expr::Mul(
            Box::new(Expr::Variable("x".to_string())),
            Box::new(Expr::Number(2.0)),
        );

        // Act
        editor.add_formula("double_x", expr);

        // Assert
        assert_eq!(editor.formula_count(), 1);
    }

    #[test]
    fn test_nested_expression_aaa() {
        // Arrange
        // (x^2 + y) / 2
        let expr = Expr::Div(
            Box::new(Expr::Add(
                Box::new(Expr::Pow(
                    Box::new(Expr::Variable("x".to_string())),
                    Box::new(Expr::Number(2.0)),
                )),
                Box::new(Expr::Variable("y".to_string())),
            )),
            Box::new(Expr::Number(2.0)),
        );
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 3.0);
        vars.insert("y".to_string(), 7.0);

        // Act
        let result = expr.evaluate(&vars).unwrap();

        // Assert
        // (9 + 7) / 2 = 8.0
        assert!((result - 8.0).abs() < 1e-10);
    }
}
