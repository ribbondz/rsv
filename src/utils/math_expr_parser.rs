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
    tokens: Vec<Token>,
    ch: char,      // current char
    offset: usize, // current position
    used_columns: Vec<usize>,
    max_column: usize,
}

impl Parser {
    fn parse(source: &str) -> Self {
        let mut p = Parser {
            source: source.replace(" ", "").to_owned(),
            tokens: vec![],
            ch: '0',
            offset: 0,
            used_columns: vec![],
            max_column: 0,
        };
        match p.source.chars().next() {
            Some(ch) => p.ch = ch,
            None => return p,
        };

        while let Some(tok) = p.next_tok() {
            p.tokens.push(tok);
        }

        p
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

                self.used_columns.push(col);
                self.max_column = self.max_column.max(col);
                Some(Token::Col(col))
            }
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

#[derive(Debug)]
pub struct CompiledExpr {
    node: Node,
    used_columns: Vec<usize>,
    max_column: usize,
}

impl CompiledExpr {
    pub fn new() -> Self {
        CompiledExpr {
            node: Node {
                token: Token::Literal(0.0),
                node_type: NodeType::Fixed,
                val: 0.0,
                calculated: true,
                lhs: None,
                rhs: None,
            },
            used_columns: vec![],
            max_column: 0,
        }
    }

    pub fn evaluate(&self, cols: Option<&[f64]>) -> f64 {
        match self.node.calculated {
            true => self.node.val,
            false => self.node.evaluate(cols),
        }
    }

    pub fn contains_column(&self, col: &usize) -> bool {
        self.used_columns.contains(col)
    }

    pub fn max_column(&self) -> usize {
        self.max_column
    }
}

#[derive(Debug)]
enum NodeType {
    Fixed,
    ColumnRelated,
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    val: f64,
    token: Token,
    calculated: bool,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn from_col(col: usize) -> Node {
        Node {
            token: Token::Col(col),
            node_type: NodeType::ColumnRelated,
            val: 0.0,
            calculated: false,
            lhs: None,
            rhs: None,
        }
    }

    fn from_number(val: f64) -> Node {
        Node {
            token: Token::Literal(val),
            node_type: NodeType::Fixed,
            val,
            calculated: true,
            lhs: None,
            rhs: None,
        }
    }

    fn evaluate(&self, cols: Option<&[f64]>) -> f64 {
        if self.calculated {
            return self.val;
        }

        match self.token {
            Token::Col(col) => cols.unwrap()[col],
            Token::Operator(op) => {
                let l = self.lhs.as_ref().unwrap().evaluate(cols);
                let r = self.rhs.as_ref().unwrap().evaluate(cols);
                match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    '%' => (l / r).floor(),
                    '^' => l.powi(r as i32),
                    _ => werr_exit!("Node evaluate error."),
                }
            }
            Token::Literal(v) => v,
        }
    }
}

pub struct AST {
    tokens: Vec<Token>,
    curr_tok: Option<Token>,
    curr_index: usize,
}

impl AST {
    pub fn parse(source: &str) -> CompiledExpr {
        let p = Parser::parse(source);

        if p.tokens.is_empty() {
            return CompiledExpr::new();
        }

        let mut ast = AST {
            curr_tok: p.tokens.get(0).copied(),
            tokens: p.tokens,
            curr_index: 0,
        };

        let node = ast.parse_expression().unwrap();
        CompiledExpr {
            node,
            used_columns: p.used_columns,
            max_column: p.max_column,
        }
    }

    fn parse_expression(&mut self) -> Option<Node> {
        let lhs = self.parse_primary();
        if lhs.is_none() {
            return None;
        }
        self.parse_bin_op_rhs(0, lhs.unwrap())
    }

    fn parse_primary(&mut self) -> Option<Node> {
        let expr = match self.curr_tok {
            Some(Token::Col(col)) => Some(Node::from_col(col)),
            Some(Token::Literal(val)) => Some(Node::from_number(val)),
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

    fn parse_bin_op_rhs(&mut self, exec_prec: i32, mut lhs: Node) -> Option<Node> {
        loop {
            let tok_prec = self.get_tok_precedence();
            if tok_prec < exec_prec {
                return Some(lhs);
            }
            let op = match self.curr_tok {
                Some(Token::Operator(op)) => op,
                _ => werr_exit!("AST parse_bin_op_rhs error."),
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
                // 递归，将当前优先级+1
                rhs = self.parse_bin_op_rhs(exec_prec + 1, rhs.unwrap());
                if rhs.is_none() {
                    return None;
                }
            }

            let rhs = rhs.unwrap();
            lhs = Node {
                token: Token::Operator(op),
                node_type: match (&lhs.node_type, &rhs.node_type) {
                    (NodeType::ColumnRelated, _) => NodeType::ColumnRelated,
                    (_, NodeType::ColumnRelated) => NodeType::ColumnRelated,
                    (_, _) => NodeType::Fixed,
                },
                val: 0.0,
                calculated: false,
                lhs: Some(Box::new(lhs)),
                rhs: Some(Box::new(rhs)),
            };

            // update fixed value for this node
            if !matches!(lhs.node_type, NodeType::ColumnRelated) {
                lhs.val = lhs.evaluate(None);
                lhs.calculated = true
            };
        }
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
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_expr_parser() {
        let p = Parser::parse("").tokens;
        assert_eq!(p.len(), 0);

        let p = Parser::parse(" ").tokens;
        assert_eq!(p.len(), 0);

        let p = Parser::parse("  @1  + ( 1.05 + @2 ^ 2 )").tokens;
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
        let p = AST::parse("4+2");
        // dbg!(&p);
        assert_eq!(p.evaluate(None), 6.0);

        let p = AST::parse("  @0  + (1.05 + @1 ^ 2 * 2)");
        // dbg!(&p);
        assert_eq!(p.evaluate(Some(&vec![1.0, 1.0])), 4.05);
        assert_eq!(p.evaluate(Some(&vec![2.0, 2.0])), 11.05);
        assert_eq!(p.evaluate(Some(&vec![1.0, 3.0])), 20.05);
        assert_eq!(p.evaluate(Some(&vec![2.0, 4.0])), 35.05);

        let p = AST::parse("  @0 + (1.05 + (@1 ^ 2) * 2)");
        // dbg!(&p);
        assert_eq!(p.evaluate(Some(&vec![1.0, 1.0])), 4.05);
        assert_eq!(p.evaluate(Some(&vec![2.0, 2.0])), 11.05);
        assert_eq!(p.evaluate(Some(&vec![1.0, 3.0])), 20.05);
        assert_eq!(p.evaluate(Some(&vec![2.0, 4.0])), 35.05);
    }
}
