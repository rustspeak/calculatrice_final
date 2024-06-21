// Importation des modules nécessaires
use std::io::{self, Write};
use regex::Regex;
use mathparse::eval;

// Fonction qui analyse l'entrée utilisateur et renvoie une liste de tokens
fn parse_input() -> Vec<(String, bool)> {
    // Déclaration et initialisation de la variable d'entrée
    let mut input = String::new();

    // Affichage d'un message à l'utilisateur
    print!("Entrez une expression mathématique : ");

    // Vidage du buffer de sortie standard
    io::stdout().flush().unwrap();

    // Lecture de l'entrée utilisateur
    io::stdin().read_line(&mut input).unwrap();

    // Tokenisation de l'entrée
    let tokens = tokenize(&input);

    // Validation des tokens
    validate_tokens(&tokens);

    // Renvoi des tokens
    tokens
}

// Fonction qui tokenise une expression en une liste de tokens
fn tokenize(input: &str) -> Vec<(String, bool)> {
    // Création d'une expression régulière pour détecter les tokens
    let re = Regex::new(r"[0-9\+\-\*\./\(\)]").unwrap();

    // Mapping des captures de l'expression régulière vers une liste de tokens
    re.captures_iter(input).map(|m| {
        let token = m[0].to_string();
        (token, is_number(&token))
    }).collect()
}

// Fonction qui vérifie si un token représente un nombre
fn is_number(token: &str) -> bool {
    token.parse::<f64>().is_ok()
}

// Fonction qui valide une liste de tokens
fn validate_tokens(tokens: &Vec<(String, bool)>) {
    // Vérification que l'expression commence par un nombre
    if !tokens.is_empty() && !is_number(&tokens[0].0) {
        panic!("L'expression doit commencer par un nombre");
    }

    // Vérification des symboles invalides et des opérateurs consécutifs
    for (i, (token, is_num)) in tokens.iter().enumerate().skip(1) {
        if !is_num && !is_operator(token) {
            panic!("Symbole invalide '{}' à la position {}", token, i + 1);
        }

        if is_operator(token) && !is_num && i == tokens.len() - 1 {
            panic!("L'expression doit se terminer par un nombre");
        }

        if is_operator(token) && is_operator(&tokens[i - 1].0) {
            panic!("Opérateurs consécutifs '{}' et '{}'", tokens[i - 1].0, token);
        }
    }
}

// Fonction qui vérifie si un token représente un opérateur
fn is_operator(token: &str) -> bool {
    matches!(token, "+", "-", "*", "/")
}

// Fonction qui convertit un token en notation mathématique
fn convert_to_math(token: &str) -> String {
    if is_number(token) {
        token.to_string()
    } else {
        token.to_string()
    }
}

// Fonction qui évalue une expression mathématique
fn evaluate_expression(expression: &str) -> Result<f64, String> {
    match eval(expression) {
        Ok(result) => Ok(result),
        Err(e) => Err(e.to_string()),
    }
}

// Définition d'un énumérateur pour les opérateurs
enum Operator {
    multiplication,
    division,
    addition,
    soustraction,
}

// Implémentation de la méthode prioriter pour l'énumérateur Operator
impl Operator {
    fn prioriter(&self) -> u8 {
        match self {
            Operator::multiplication | Operator::division => 2,
            Operator::addition | Operator::soustraction => 1,
        }
    }
}

// Fonction qui convertit une expression infixe en notation postfixe
fn infix_to_postfix(tokens: &Vec<(String, bool)>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    for (token, _) in tokens {
        if is_number(token) {
            output.push(token.clone());
        } else if is_operator(token) {
            while !operator_stack.is_empty() && Operator::prioriter(&operator_from_str(token)) <= Operator::prioriter(&operator_from_str(&operator_stack.last().unwrap())) {
                output.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(token.clone());
        } else if token == "(" {
            operator_stack.push(token.clone());
        } else if token == ")" {
            while operator_stack.last().unwrap() != "(" {
                output.push(operator_stack.pop().unwrap());
            }
            operator_stack.pop();
        }
    }

    while !operator_stack.is_empty() {
        output.push(operator_stack.pop().unwrap());
    }

    output
}

// Fonction qui évalue une expression postfixe
fn evaluate_postfix(postfix_expression: &Vec<String>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in postfix_expression {
        if is_number(token) {
            stack.push(token.parse::<f64>().unwrap());
        } else if is_operator(token) {
            let operand2 = stack.pop().unwrap();
            let operand1 = stack.pop().unwrap();

            match token.as_str() {
                "+" => stack.push(operand1 + operand2),
                "-" => stack.push(operand1 - operand2),
                "*" => stack.push(operand1 * operand2),
                "/" => stack.push(operand1 / operand2),
                _ => return Err("Opérateur inconnu".to_string()),
            }
        }
    }

    if stack.len() != 1 {
        return Err("Expression invalide".to_string());
    }

    Ok(stack.pop().unwrap())
}

// Fonction qui convertit une chaîne en un opérateur
fn operator_from_str(token: &str) -> Operator {
    match token {
        "*" => Operator::multiplication,
        "/" => Operator::division,
        "+" => Operator::addition,
        "-" => Operator::soustraction,
        _ => panic!("Opérateur inconnu"),
    }
}

// Fonction qui calcule une expression mathématique complète
fn calculate_expression(expression: &str) -> Result<f64, String> {
    let tokens = parse_input();
    let postfix_expression = infix_to_postfix(&tokens);
    evaluate_postfix(&postfix_expression)
}

// Fonction principale
fn main() {
    let expression = "2 + 3 * (4 - 1)";
    match calculate_expression(expression) {
        Ok(result) => println!("Résultat: {}", result),
        Err(error) => println!("Erreur: {}", error),
    }
}