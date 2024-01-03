#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::utils::util::werr_exit;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Col(usize),     // e.g. col(2) of data
    Literal(f64),   // e.g. 50
    Operator(char), // e.g. + - * /
}

#[derive(Debug)]
struct Parser {
    source: String, // original expr
    ch: char,       // current char
    offset: usize,  // current position
}

impl Parser {
    fn parse(source: &str) -> Vec<Token> {
        let mut toks = vec![];
        let ch = match source.chars().next() {
            Some(ch) => ch,
            None => return toks,
        };

        let mut p = Parser {
            source: source.to_owned(),
            ch,
            offset: 0,
        };

        while let Some(tok) = p.next_tok() {
            toks.push(tok);
        }

        toks
    }

    fn next_tok(&mut self) -> Option<Token> {
        // arrive at the end of expr
        if self.offset >= self.source.len() {
            return None;
        }

        // ignore white spaces
        while self.is_white_space(self.ch) && self.next_ch() {}

        match self.ch {
            '(' | ')' | '+' | '-' | '*' | '/' | '^' | '%' => {
                let tok = Token::Operator(self.ch);
                self.next_ch();
                Some(tok)
            }

            '0'..='9' => {
                let start = self.offset;
                while self.is_digit_num(self.ch) && self.next_ch() {}
                let v = self.source.get(start..self.offset).unwrap();
                let v: f64 = v.parse().unwrap();
                Some(Token::Literal(v))
            }

            '@' | 'c' => {
                let start = self.offset;
                self.next_ch();
                while self.is_digit_num(self.ch) && self.next_ch() {}
                let col = self.source.get(start + 1..self.offset).unwrap();
                let col = match col.parse::<usize>() {
                    Ok(v) => v,
                    Err(err) => werr_exit!("{}", err),
                };

                Some(Token::Col(col))
            }

            ' ' => return None,

            _ => werr_exit!("{} is not recognized in Expr.", self.ch),
        }
    }

    fn next_ch(&mut self) -> bool {
        self.offset += 1;
        match self.source.chars().nth(self.offset) {
            Some(ch) => {
                self.ch = ch;
                return true;
            }
            None => return false,
        }
    }

    fn is_white_space(&self, c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    fn is_digit_num(&self, c: char) -> bool {
        ('0' <= c && c <= '9') || c == '.' || c == '_' || c == 'e'
    }
}

trait ExprATS {
    fn to_str(&self) -> String;
    fn evaluate(&self, row: &[f64]) -> f64;
}

#[derive(Debug)]
struct NumberExprAST {
    val: f64,
}

#[derive(Debug)]
struct ColExprAST {
    col: usize,
}

struct BinaryExprAST {
    op: char,
    lhs: Box<dyn ExprATS>,
    rhs: Box<dyn ExprATS>,
}

impl ExprATS for NumberExprAST {
    fn to_str(&self) -> String {
        format!("{}", self.val)
    }

    fn evaluate(&self, _row: &[f64]) -> f64 {
        self.val
    }
}

impl ExprATS for ColExprAST {
    fn to_str(&self) -> String {
        format!("col: @{}", self.col)
    }

    fn evaluate(&self, row: &[f64]) -> f64 {
        row[self.col]
    }
}

impl ExprATS for BinaryExprAST {
    fn to_str(&self) -> String {
        format!(
            "BinaryExprAST: ({} {} {})",
            self.lhs.to_str(),
            self.op,
            self.rhs.to_str()
        )
    }

    fn evaluate(&self, row: &[f64]) -> f64 {
        let l = self.lhs.evaluate(row);
        let r = self.rhs.evaluate(row);
        match self.op {
            '+' => l + r,
            '-' => l - r,
            '*' => l * r,
            '/' => l / r,
            '%' => (l / r).floor(),
            '^' => l.powi(r as i32),
            _ => werr_exit!("<{}> is not recognized.", self.op),
        }
    }
}

struct AST {
    tokens: Vec<Token>,
    curr_tok: Option<Token>,
    curr_index: usize,
    ast: Box<dyn ExprATS>,
}

impl AST {
    fn parse(tokens: Vec<Token>) -> Self {
        if tokens.is_empty() {
            werr_exit!("no tokens!")
        }

        let mut ast = AST {
            curr_tok: tokens.get(0).copied(),
            tokens,
            curr_index: 0,
            ast: Box::new(NumberExprAST { val: 0.0 }),
        };

        match ast.parse_expression() {
            Some(expr) => ast.ast = expr,
            None => werr_exit!("parse expression error."),
        };

        ast
    }

    fn tok_precedence(&self, op: char) -> i32 {
        match op {
            '+' => 20,
            '-' => 20,
            '*' => 40,
            '/' => 40,
            '%' => 40,
            '^' => 60,
            _ => -1,
        }
    }

    fn next_tok(&mut self) -> Option<Token> {
        self.curr_index += 1;
        self.curr_tok = self.tokens.get(self.curr_index).copied();
        self.curr_tok
    }

    fn get_tok_precedence(&mut self) -> i32 {
        let Some(Token::Operator(op)) = self.curr_tok else {
            return -1;
        };
        self.tok_precedence(op)
    }

    fn parse_expression(&mut self) -> Option<Box<dyn ExprATS>> {
        let lhs = self.parse_primary();
        if lhs.is_none() {
            return None;
        }
        self.parse_bin_op_rhs(0, lhs.unwrap())
    }

    fn parse_primary(&mut self) -> Option<Box<dyn ExprATS>> {
        let expr = match self.curr_tok {
            Some(Token::Col(col)) => Some(Box::new(ColExprAST { col }) as Box<dyn ExprATS>),
            Some(Token::Literal(val)) => Some(Box::new(NumberExprAST { val }) as Box<dyn ExprATS>),
            Some(Token::Operator(op)) => {
                if op != '(' {
                    werr_exit!("start operation <{}> is not recognized", op)
                }
                self.next_tok();
                self.parse_expression()
            }
            None => return None,
        };

        self.next_tok();
        expr
    }

    fn parse_bin_op_rhs(
        &mut self,
        exec_prec: i32,
        mut lhs: Box<dyn ExprATS>,
    ) -> Option<Box<dyn ExprATS>> {
        loop {
            let tok_prec = self.get_tok_precedence();
            if tok_prec < exec_prec {
                return Some(lhs);
            }
            let op = match self.curr_tok {
                Some(Token::Operator(op)) => op,
                _ => return None,
            };
            if self.next_tok().is_none() {
                return Some(lhs);
            }
            let mut rhs = self.parse_primary();
            if rhs.is_none() {
                return None;
            }
            let next_prec = self.get_tok_precedence();
            if tok_prec < next_prec {
                rhs = self.parse_bin_op_rhs(exec_prec, rhs.unwrap());
                if rhs.is_none() {
                    return None;
                }
            }
            lhs = Box::new(BinaryExprAST {
                op,
                lhs,
                rhs: rhs.unwrap(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_expr_parser() {
        let p = Parser::parse("");
        assert_eq!(p.len(), 0);

        let p = Parser::parse(" ");
        assert_eq!(p.len(), 0);

        let p = Parser::parse("  @1  + ( 1.05 + @2 ^ 2 )");
        println!("{:?}", p);
        assert_eq!(p[0], Token::Col(1));
        assert_eq!(p[1], Token::Operator('+'));
        assert_eq!(p[2], Token::Operator('('));
        assert_eq!(p[3], Token::Literal(1.05));
        assert_eq!(p[4], Token::Operator('+'));
        assert_eq!(p[5], Token::Col(2));
        assert_eq!(p[6], Token::Operator('^'));
        assert_eq!(p[7], Token::Literal(2.0));
        assert_eq!(p[8], Token::Operator(')'));
    }

    #[test]
    fn test_ast() {
        let p = Parser::parse("  @0  + (1.05 + @1 ^ 2 * 2)");
        let ar = AST::parse(p);
        let t = "BinaryExprAST: (col: @0 + BinaryExprAST: (1.05 + BinaryExprAST: (BinaryExprAST: (col: @1 ^ 2) * 2)))";
        assert_eq!(ar.ast.to_str(), t);

        assert_eq!(ar.ast.evaluate(&vec![1.0, 1.0]), 4.05);
        assert_eq!(ar.ast.evaluate(&vec![2.0, 2.0]), 11.05);
        assert_eq!(ar.ast.evaluate(&vec![1.0, 3.0]), 20.05);
        assert_eq!(ar.ast.evaluate(&vec![2.0, 4.0]), 35.05);
    }
}
