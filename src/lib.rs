//! Formula and equation editor for SigmaOS.
//!
//! Provides a recursive-descent expression parser, evaluator, named formula
//! registry, and CLI-friendly API. Deterministic and dependency-free.

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
            Expr::Pow(base, exp) => format!("({} ^ {})", base.to_string_repr(), exp.to_string_repr()),
        }
    }
}

/// Parse errors for expression parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedChar(char),
    UnexpectedEof,
    ExpectedNumber,
    ExpectedVariable,
    ExpectedRParen,
    EmptyExpression,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedChar(c) => write!(f, "unexpected character: {c}"),
            Self::UnexpectedEof => write!(f, "unexpected end of expression"),
            Self::ExpectedNumber => write!(f, "expected number"),
            Self::ExpectedVariable => write!(f, "expected variable"),
            Self::ExpectedRParen => write!(f, "expected ')'"),
            Self::EmptyExpression => write!(f, "empty expression"),
        }
    }
}

impl std::error::Error for ParseError {}

struct Parser<'a> {
    chars: Vec<char>,
    pos: usize,
    _src: &'a str,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self { chars: src.chars().collect(), pos: 0, _src: src }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        Some(c)
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some('+') => {
                    self.next();
                    let right = self.parse_multiplicative()?;
                    left = Expr::Add(Box::new(left), Box::new(right));
                }
                Some('-') => {
                    self.next();
                    let right = self.parse_multiplicative()?;
                    left = Expr::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_power()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some('*') => {
                    self.next();
                    let right = self.parse_power()?;
                    left = Expr::Mul(Box::new(left), Box::new(right));
                }
                Some('/') => {
                    self.next();
                    let right = self.parse_power()?;
                    left = Expr::Div(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let base = self.parse_unary()?;
        self.skip_ws();
        if self.peek() == Some('^') {
            self.next();
            let exp = self.parse_unary()?;
            return Ok(Expr::Pow(Box::new(base), Box::new(exp)));
        }
        Ok(base)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        self.skip_ws();
        if self.peek() == Some('-') {
            self.next();
            let inner = self.parse_unary()?;
            return Ok(Expr::Sub(Box::new(Expr::Number(0.0)), Box::new(inner)));
        }
        if self.peek() == Some('+') {
            self.next();
            return self.parse_unary();
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        self.skip_ws();
        match self.peek() {
            None => Err(ParseError::UnexpectedEof),
            Some('(') => {
                self.next();
                let expr = self.parse_expression()?;
                self.skip_ws();
                if self.next() != Some(')') {
                    return Err(ParseError::ExpectedRParen);
                }
                Ok(expr)
            }
            Some(c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(c) if c.is_alphabetic() || c == '_' => self.parse_variable(),
            Some(c) => Err(ParseError::UnexpectedChar(c)),
        }
    }

    fn parse_number(&mut self) -> Result<Expr, ParseError> {
        let start = self.pos;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '.') {
            self.pos += 1;
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        text.parse::<f64>().map(Expr::Number).map_err(|_| ParseError::ExpectedNumber)
    }

    fn parse_variable(&mut self) -> Result<Expr, ParseError> {
        let start = self.pos;
        while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
            self.pos += 1;
        }
        if start == self.pos {
            return Err(ParseError::ExpectedVariable);
        }
        Ok(Expr::Variable(self.chars[start..self.pos].iter().collect()))
    }
}

/// Parse a mathematical expression string into an [`Expr`].
pub fn parse_expr(input: &str) -> Result<Expr, ParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseError::EmptyExpression);
    }
    let mut parser = Parser::new(trimmed);
    let expr = parser.parse_expression()?;
    parser.skip_ws();
    if parser.pos != parser.chars.len() {
        return Err(ParseError::UnexpectedChar(parser.chars[parser.pos]));
    }
    Ok(expr)
}

/// Parse and evaluate an expression in one step.
pub fn evaluate_expr(input: &str, vars: &HashMap<String, f64>) -> Result<f64, String> {
    parse_expr(input)
        .map_err(|e| e.to_string())
        .and_then(|expr| expr.evaluate(vars))
}

/// Named formula registry with parse/evaluate support.
pub struct FormulaEditor {
    formulas: Vec<(String, Expr)>,
}

impl FormulaEditor {
    pub fn new() -> Self {
        Self { formulas: Vec::new() }
    }

    pub fn add_formula(&mut self, name: &str, expr: Expr) {
        self.formulas.push((name.to_string(), expr));
    }

    pub fn add_formula_str(&mut self, name: &str, source: &str) -> Result<(), ParseError> {
        let expr = parse_expr(source)?;
        self.add_formula(name, expr);
        Ok(())
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

    fn vars(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn test_parse_precedence() {
        let expr = parse_expr("1 + 2 * 3").unwrap();
        assert_eq!(expr.evaluate(&HashMap::new()).unwrap(), 7.0);
    }

    #[test]
    fn test_parse_power_and_parens() {
        let expr = parse_expr("(1 + 2) ^ 2").unwrap();
        assert_eq!(expr.evaluate(&HashMap::new()).unwrap(), 9.0);
    }

    #[test]
    fn test_parse_unary_and_division() {
        let expr = parse_expr("-6 / 2").unwrap();
        assert_eq!(expr.evaluate(&HashMap::new()).unwrap(), -3.0);
        assert!(evaluate_expr("1 / 0", &HashMap::new()).is_err());
    }

    #[test]
    fn test_variables() {
        let expr = parse_expr("x * y + 1").unwrap();
        assert_eq!(expr.evaluate(&vars(&[("x", 3.0), ("y", 4.0)])).unwrap(), 13.0);
        assert!(expr.evaluate(&HashMap::new()).is_err());
    }

    #[test]
    fn test_parse_errors() {
        assert!(parse_expr("").is_err());
        assert!(parse_expr("1 +").is_err());
        assert!(parse_expr("(1").is_err());
        assert!(parse_expr("@").is_err());
    }

    #[test]
    fn test_formula_editor_str() {
        let mut editor = FormulaEditor::new();
        editor.add_formula_str("double", "x * 2").unwrap();
        assert_eq!(editor.evaluate_formula("double", &vars(&[("x", 5.0)])).unwrap(), 10.0);
        assert!(editor.add_formula_str("bad", "1 +").is_err());
        assert_eq!(editor.formula_count(), 1);
    }
}
