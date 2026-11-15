use sigmaos_libreoffice_math::{Expr, FormulaEditor};
use std::collections::HashMap;

#[test]
fn test_formula_editor_evaluate_aaa() {
    // Arrange
    let mut editor = FormulaEditor::new();
    editor.add_formula(
        "area",
        Expr::Mul(
            Box::new(Expr::Variable("w".to_string())),
            Box::new(Expr::Variable("h".to_string())),
        ),
    );
    let mut vars = HashMap::new();
    vars.insert("w".to_string(), 5.0);
    vars.insert("h".to_string(), 4.0);

    // Act
    let result = editor.evaluate_formula("area", &vars).unwrap();

    // Assert
    assert!((result - 20.0).abs() < 1e-10);
}

#[test]
fn test_missing_formula_aaa() {
    // Arrange
    let editor = FormulaEditor::new();
    let vars = HashMap::new();

    // Act
    let result = editor.evaluate_formula("nonexistent", &vars);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_division_by_zero_aaa() {
    // Arrange
    let expr = Expr::Div(Box::new(Expr::Number(5.0)), Box::new(Expr::Number(0.0)));
    let vars = HashMap::new();

    // Act
    let result = expr.evaluate(&vars);

    // Assert
    assert!(result.is_err());
}
