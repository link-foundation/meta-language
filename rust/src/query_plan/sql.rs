use super::{
    AggregateFunction, Assignment, BinaryOperator, Projection, QueryExpression, QueryOperation,
    QueryPlanError, QueryPlanErrorKind, QuerySource, QueryValue, SortDirection, SortExpression,
    UnaryOperator,
};

pub(super) fn parse(source: &str) -> Result<QueryOperation, QueryPlanError> {
    let tokens = lex(source)?;
    let mut parser = Parser::new(tokens, source.len());
    let operation = parser.parse_statement()?;
    if parser.consume_symbol(';') && parser.consume_symbol(';') {
        return Err(parser.syntax("only one SQL statement may be lowered"));
    }
    if !parser.is_done() {
        return Err(parser.syntax("unexpected input after complete SQL statement"));
    }
    Ok(operation)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokenKind {
    Word { value: String, quoted: bool },
    Number(String),
    String(String),
    Parameter(String),
    Symbol(char),
    Operator(String),
}

fn lex(source: &str) -> Result<Vec<Token>, QueryPlanError> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            byte if byte.is_ascii_whitespace() => index += 1,
            b'-' if bytes.get(index + 1) == Some(&b'-') => {
                index += 2;
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                let start = index;
                index += 2;
                while index + 1 < bytes.len() && !(bytes[index] == b'*' && bytes[index + 1] == b'/')
                {
                    index += 1;
                }
                if index + 1 >= bytes.len() {
                    return Err(error(
                        QueryPlanErrorKind::Syntax,
                        "unterminated SQL block comment",
                        start,
                    ));
                }
                index += 2;
            }
            b'\'' => tokens.push(read_string(source, &mut index)?),
            b'"' | b'`' | b'[' => tokens.push(read_quoted_identifier(source, &mut index)?),
            byte if byte.is_ascii_digit() => {
                let start = index;
                index += 1;
                while index < bytes.len() && bytes[index].is_ascii_digit() {
                    index += 1;
                }
                if bytes.get(index) == Some(&b'.')
                    && bytes.get(index + 1).is_some_and(u8::is_ascii_digit)
                {
                    index += 1;
                    while index < bytes.len() && bytes[index].is_ascii_digit() {
                        index += 1;
                    }
                }
                tokens.push(Token {
                    kind: TokenKind::Number(source[start..index].to_string()),
                    offset: start,
                });
            }
            byte if is_identifier_start(byte) => {
                let start = index;
                index += 1;
                while index < bytes.len() && is_identifier_continue(bytes[index]) {
                    index += 1;
                }
                tokens.push(Token {
                    kind: TokenKind::Word {
                        value: source[start..index].to_string(),
                        quoted: false,
                    },
                    offset: start,
                });
            }
            b'?' => {
                tokens.push(Token {
                    kind: TokenKind::Parameter("?".to_string()),
                    offset: index,
                });
                index += 1;
            }
            b'$' | b':' | b'@' => tokens.push(read_parameter(source, &mut index)?),
            b'<' | b'>' | b'!' | b'=' => {
                let start = index;
                index += 1;
                if index < bytes.len()
                    && matches!(
                        (bytes[start], bytes[index]),
                        (b'<' | b'>' | b'!', b'=') | (b'<', b'>')
                    )
                {
                    index += 1;
                }
                tokens.push(Token {
                    kind: TokenKind::Operator(source[start..index].to_string()),
                    offset: start,
                });
            }
            byte @ (b'+' | b'-' | b'*' | b'/') => {
                tokens.push(Token {
                    kind: TokenKind::Operator(char::from(byte).to_string()),
                    offset: index,
                });
                index += 1;
            }
            byte @ (b'(' | b')' | b',' | b'.' | b';') => {
                tokens.push(Token {
                    kind: TokenKind::Symbol(char::from(byte)),
                    offset: index,
                });
                index += 1;
            }
            _ => {
                return Err(error(
                    QueryPlanErrorKind::Syntax,
                    "unsupported character in SQL statement",
                    index,
                ));
            }
        }
    }
    Ok(tokens)
}

const fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_' || byte >= 0x80
}

const fn is_identifier_continue(byte: u8) -> bool {
    is_identifier_start(byte) || byte.is_ascii_digit() || byte == b'$'
}

fn read_string(source: &str, index: &mut usize) -> Result<Token, QueryPlanError> {
    let start = *index;
    *index += 1;
    let mut value = String::new();
    while *index < source.len() {
        if source.as_bytes()[*index] == b'\'' {
            if source.as_bytes().get(*index + 1) == Some(&b'\'') {
                value.push('\'');
                *index += 2;
                continue;
            }
            *index += 1;
            return Ok(Token {
                kind: TokenKind::String(value),
                offset: start,
            });
        }
        let character = source[*index..]
            .chars()
            .next()
            .expect("string cursor is on a character boundary");
        value.push(character);
        *index += character.len_utf8();
    }
    Err(error(
        QueryPlanErrorKind::Syntax,
        "unterminated SQL string literal",
        start,
    ))
}

fn read_quoted_identifier(source: &str, index: &mut usize) -> Result<Token, QueryPlanError> {
    let start = *index;
    let opener = source.as_bytes()[*index];
    let closer = if opener == b'[' { b']' } else { opener };
    *index += 1;
    let value_start = *index;
    while *index < source.len() && source.as_bytes()[*index] != closer {
        let character = source[*index..]
            .chars()
            .next()
            .expect("identifier cursor is on a character boundary");
        *index += character.len_utf8();
    }
    if *index == source.len() {
        return Err(error(
            QueryPlanErrorKind::Syntax,
            "unterminated quoted SQL identifier",
            start,
        ));
    }
    let value = source[value_start..*index].to_string();
    *index += 1;
    Ok(Token {
        kind: TokenKind::Word {
            value,
            quoted: true,
        },
        offset: start,
    })
}

fn read_parameter(source: &str, index: &mut usize) -> Result<Token, QueryPlanError> {
    let start = *index;
    *index += 1;
    while *index < source.len() && is_identifier_continue(source.as_bytes()[*index]) {
        *index += 1;
    }
    if *index == start + 1 {
        return Err(error(
            QueryPlanErrorKind::Syntax,
            "SQL parameter marker is missing a name or index",
            start,
        ));
    }
    Ok(Token {
        kind: TokenKind::Parameter(source[start..*index].to_string()),
        offset: start,
    })
}

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    source_len: usize,
}

impl Parser {
    const fn new(tokens: Vec<Token>, source_len: usize) -> Self {
        Self {
            tokens,
            cursor: 0,
            source_len,
        }
    }

    fn parse_statement(&mut self) -> Result<QueryOperation, QueryPlanError> {
        if self.at_keyword("SELECT") {
            self.parse_select()
        } else if self.at_keyword("INSERT") {
            self.parse_insert()
        } else if self.at_keyword("UPDATE") {
            self.parse_update()
        } else if self.at_keyword("DELETE") {
            self.parse_delete()
        } else {
            Err(self.syntax("expected SELECT, INSERT, UPDATE, or DELETE"))
        }
    }

    fn parse_select(&mut self) -> Result<QueryOperation, QueryPlanError> {
        self.expect_keyword("SELECT")?;
        let distinct = self.consume_keyword("DISTINCT");
        let mut projection = Vec::new();
        loop {
            if self.is_done() || self.at_keyword("FROM") {
                return Err(self.syntax("SELECT projection must not be empty"));
            }
            let expression = self.parse_expression(0)?;
            let alias = self.parse_optional_alias()?;
            projection.push(Projection { expression, alias });
            if !self.consume_symbol(',') {
                break;
            }
        }

        let source = if self.consume_keyword("FROM") {
            Some(self.parse_source(true)?)
        } else {
            None
        };
        let predicate = if self.consume_keyword("WHERE") {
            Some(self.parse_expression(0)?)
        } else {
            None
        };
        let group_by = if self.consume_keyword("GROUP") {
            self.expect_keyword("BY")?;
            self.parse_expression_list()?
        } else {
            Vec::new()
        };
        let order_by = if self.consume_keyword("ORDER") {
            self.expect_keyword("BY")?;
            self.parse_order_list()?
        } else {
            Vec::new()
        };
        let limit = if self.consume_keyword("LIMIT") {
            Some(self.parse_nonnegative_integer("LIMIT")?)
        } else {
            None
        };
        let offset = if self.consume_keyword("OFFSET") {
            Some(self.parse_nonnegative_integer("OFFSET")?)
        } else {
            None
        };
        Ok(QueryOperation::Select {
            distinct,
            projection,
            source,
            predicate,
            group_by,
            order_by,
            limit,
            offset,
        })
    }

    fn parse_insert(&mut self) -> Result<QueryOperation, QueryPlanError> {
        self.expect_keyword("INSERT")?;
        self.expect_keyword("INTO")?;
        let into = self.parse_source(false)?;
        let columns = if self.consume_symbol('(') {
            let values = self.parse_identifier_list()?;
            self.expect_symbol(')')?;
            values
        } else {
            Vec::new()
        };
        self.expect_keyword("VALUES")?;
        let mut rows = Vec::new();
        loop {
            self.expect_symbol('(')?;
            let row = self.parse_expression_list()?;
            self.expect_symbol(')')?;
            if row.is_empty() {
                return Err(self.semantic("INSERT rows must not be empty"));
            }
            if !columns.is_empty() && row.len() != columns.len() {
                return Err(
                    self.semantic("INSERT row value count must match the declared column count")
                );
            }
            rows.push(row);
            if !self.consume_symbol(',') {
                break;
            }
        }
        Ok(QueryOperation::Insert {
            into,
            columns,
            rows,
        })
    }

    fn parse_update(&mut self) -> Result<QueryOperation, QueryPlanError> {
        self.expect_keyword("UPDATE")?;
        let table = self.parse_source(true)?;
        self.expect_keyword("SET")?;
        let mut assignments = Vec::new();
        loop {
            if self.at_keyword("WHERE") || self.is_done() {
                return Err(self.syntax("UPDATE SET must contain an assignment"));
            }
            let column = self.parse_identifier_path()?;
            self.expect_operator("=")?;
            let value = self.parse_expression(0)?;
            assignments.push(Assignment { column, value });
            if !self.consume_symbol(',') {
                break;
            }
        }
        let predicate = if self.consume_keyword("WHERE") {
            Some(self.parse_expression(0)?)
        } else {
            None
        };
        Ok(QueryOperation::Update {
            table,
            assignments,
            predicate,
        })
    }

    fn parse_delete(&mut self) -> Result<QueryOperation, QueryPlanError> {
        self.expect_keyword("DELETE")?;
        self.expect_keyword("FROM")?;
        let source = self.parse_source(true)?;
        let predicate = if self.consume_keyword("WHERE") {
            Some(self.parse_expression(0)?)
        } else {
            None
        };
        Ok(QueryOperation::Delete { source, predicate })
    }

    fn parse_source(&mut self, allow_alias: bool) -> Result<QuerySource, QueryPlanError> {
        let path = self.parse_identifier_path()?;
        let alias = if allow_alias {
            self.parse_optional_alias()?
        } else {
            None
        };
        Ok(QuerySource { path, alias })
    }

    fn parse_expression_list(&mut self) -> Result<Vec<QueryExpression>, QueryPlanError> {
        let mut expressions = vec![self.parse_expression(0)?];
        while self.consume_symbol(',') {
            expressions.push(self.parse_expression(0)?);
        }
        Ok(expressions)
    }

    fn parse_order_list(&mut self) -> Result<Vec<SortExpression>, QueryPlanError> {
        let mut expressions = Vec::new();
        loop {
            let expression = self.parse_expression(0)?;
            let direction = if self.consume_keyword("DESC") {
                SortDirection::Descending
            } else {
                self.consume_keyword("ASC");
                SortDirection::Ascending
            };
            expressions.push(SortExpression {
                expression,
                direction,
            });
            if !self.consume_symbol(',') {
                break;
            }
        }
        Ok(expressions)
    }

    fn parse_expression(
        &mut self,
        minimum_precedence: u8,
    ) -> Result<QueryExpression, QueryPlanError> {
        let mut left = self.parse_prefix()?;
        while let Some((operator, precedence, token_count)) = self.infix_operator() {
            if precedence < minimum_precedence {
                break;
            }
            self.cursor += token_count;
            let right = self.parse_expression(precedence + 1)?;
            left = QueryExpression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<QueryExpression, QueryPlanError> {
        if self.consume_keyword("NOT") {
            return Ok(QueryExpression::Unary {
                operator: UnaryOperator::Not,
                operand: Box::new(self.parse_expression(6)?),
            });
        }
        if self.consume_operator("-") {
            return Ok(QueryExpression::Unary {
                operator: UnaryOperator::Negate,
                operand: Box::new(self.parse_expression(6)?),
            });
        }
        if self.consume_operator("+") {
            return Ok(QueryExpression::Unary {
                operator: UnaryOperator::Positive,
                operand: Box::new(self.parse_expression(6)?),
            });
        }
        if self.consume_symbol('(') {
            let expression = self.parse_expression(0)?;
            self.expect_symbol(')')?;
            return Ok(expression);
        }

        let Some(token) = self.advance().cloned() else {
            return Err(self.syntax("expected SQL expression"));
        };
        match token.kind {
            TokenKind::Number(value) => Ok(QueryExpression::Literal {
                value: QueryValue::Number { value },
            }),
            TokenKind::String(value) => Ok(QueryExpression::Literal {
                value: QueryValue::String { value },
            }),
            TokenKind::Parameter(name) => Ok(QueryExpression::Parameter { name }),
            TokenKind::Operator(operator) if operator == "*" => Ok(QueryExpression::Wildcard),
            TokenKind::Word { value, quoted } => {
                if !quoted && value.eq_ignore_ascii_case("NULL") {
                    return Ok(QueryExpression::Literal {
                        value: QueryValue::Null,
                    });
                }
                if !quoted && value.eq_ignore_ascii_case("TRUE") {
                    return Ok(QueryExpression::Literal {
                        value: QueryValue::Boolean { value: true },
                    });
                }
                if !quoted && value.eq_ignore_ascii_case("FALSE") {
                    return Ok(QueryExpression::Literal {
                        value: QueryValue::Boolean { value: false },
                    });
                }
                let identifier = canonical_identifier(&value, quoted);
                if self.consume_symbol('(') {
                    self.parse_function(identifier)
                } else {
                    let mut path = vec![identifier];
                    while self.consume_symbol('.') {
                        path.push(self.expect_identifier()?);
                    }
                    Ok(QueryExpression::Column { path })
                }
            }
            _ => Err(error(
                QueryPlanErrorKind::Syntax,
                "expected SQL expression",
                token.offset,
            )),
        }
    }

    fn parse_function(&mut self, name: String) -> Result<QueryExpression, QueryPlanError> {
        let distinct = self.consume_keyword("DISTINCT");
        let mut arguments = Vec::new();
        if !self.consume_symbol(')') {
            arguments = self.parse_expression_list()?;
            self.expect_symbol(')')?;
        }
        if let Some(function) = aggregate_function(&name) {
            if arguments.len() != 1 {
                return Err(self.semantic("aggregate functions require exactly one argument"));
            }
            if function != AggregateFunction::Count
                && matches!(arguments[0], QueryExpression::Wildcard)
            {
                return Err(self.semantic("only COUNT accepts a wildcard argument"));
            }
            return Ok(QueryExpression::Aggregate {
                function,
                expression: Box::new(arguments.remove(0)),
                distinct,
            });
        }
        if distinct {
            return Err(self.semantic(
                "DISTINCT function arguments are only canonicalized for supported aggregates",
            ));
        }
        Ok(QueryExpression::Function { name, arguments })
    }

    fn infix_operator(&self) -> Option<(BinaryOperator, u8, usize)> {
        if self.at_keyword("OR") {
            Some((BinaryOperator::Or, 1, 1))
        } else if self.at_keyword("AND") {
            Some((BinaryOperator::And, 2, 1))
        } else if self.at_keyword("IS") && self.peek_keyword(1, "NOT") {
            Some((BinaryOperator::IsNot, 3, 2))
        } else if self.at_keyword("IS") {
            Some((BinaryOperator::Is, 3, 1))
        } else if self.at_keyword("NOT") && self.peek_keyword(1, "LIKE") {
            Some((BinaryOperator::NotLike, 3, 2))
        } else if self.at_keyword("LIKE") {
            Some((BinaryOperator::Like, 3, 1))
        } else {
            match self.current_operator()? {
                "=" => Some((BinaryOperator::Equal, 3, 1)),
                "!=" | "<>" => Some((BinaryOperator::NotEqual, 3, 1)),
                "<" => Some((BinaryOperator::LessThan, 3, 1)),
                "<=" => Some((BinaryOperator::LessThanOrEqual, 3, 1)),
                ">" => Some((BinaryOperator::GreaterThan, 3, 1)),
                ">=" => Some((BinaryOperator::GreaterThanOrEqual, 3, 1)),
                "+" => Some((BinaryOperator::Add, 4, 1)),
                "-" => Some((BinaryOperator::Subtract, 4, 1)),
                "*" => Some((BinaryOperator::Multiply, 5, 1)),
                "/" => Some((BinaryOperator::Divide, 5, 1)),
                _ => None,
            }
        }
    }

    fn parse_optional_alias(&mut self) -> Result<Option<String>, QueryPlanError> {
        if self.consume_keyword("AS") {
            return self.expect_identifier().map(Some);
        }
        let Some(Token {
            kind: TokenKind::Word { value, quoted },
            ..
        }) = self.current()
        else {
            return Ok(None);
        };
        if !*quoted && is_clause_keyword(value) {
            return Ok(None);
        }
        let value = canonical_identifier(value, *quoted);
        self.cursor += 1;
        Ok(Some(value))
    }

    fn parse_identifier_list(&mut self) -> Result<Vec<String>, QueryPlanError> {
        let mut values = vec![self.expect_identifier()?];
        while self.consume_symbol(',') {
            values.push(self.expect_identifier()?);
        }
        Ok(values)
    }

    fn parse_identifier_path(&mut self) -> Result<Vec<String>, QueryPlanError> {
        let mut values = vec![self.expect_identifier()?];
        while self.consume_symbol('.') {
            values.push(self.expect_identifier()?);
        }
        Ok(values)
    }

    fn parse_nonnegative_integer(&mut self, clause: &str) -> Result<u64, QueryPlanError> {
        let Some(token) = self.advance().cloned() else {
            return Err(self.syntax(&format!("{clause} requires an integer")));
        };
        let TokenKind::Number(value) = token.kind else {
            return Err(error(
                QueryPlanErrorKind::Semantic,
                format!("{clause} requires a non-negative integer"),
                token.offset,
            ));
        };
        if value.contains('.') {
            return Err(error(
                QueryPlanErrorKind::Semantic,
                format!("{clause} requires a non-negative integer"),
                token.offset,
            ));
        }
        value.parse().map_err(|_| {
            error(
                QueryPlanErrorKind::Semantic,
                format!("{clause} integer is out of range"),
                token.offset,
            )
        })
    }

    fn expect_identifier(&mut self) -> Result<String, QueryPlanError> {
        let Some(token) = self.advance().cloned() else {
            return Err(self.syntax("expected SQL identifier"));
        };
        if let TokenKind::Word { value, quoted } = token.kind {
            Ok(canonical_identifier(&value, quoted))
        } else {
            Err(error(
                QueryPlanErrorKind::Syntax,
                "expected SQL identifier",
                token.offset,
            ))
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), QueryPlanError> {
        if self.consume_keyword(keyword) {
            Ok(())
        } else {
            Err(self.syntax(&format!("expected {keyword}")))
        }
    }

    fn expect_symbol(&mut self, symbol: char) -> Result<(), QueryPlanError> {
        if self.consume_symbol(symbol) {
            Ok(())
        } else {
            Err(self.syntax(&format!("expected `{symbol}`")))
        }
    }

    fn expect_operator(&mut self, operator: &str) -> Result<(), QueryPlanError> {
        if self.consume_operator(operator) {
            Ok(())
        } else {
            Err(self.syntax(&format!("expected `{operator}`")))
        }
    }

    fn consume_keyword(&mut self, keyword: &str) -> bool {
        if self.at_keyword(keyword) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn consume_symbol(&mut self, symbol: char) -> bool {
        if matches!(self.current().map(|token| &token.kind), Some(TokenKind::Symbol(value)) if *value == symbol)
        {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn consume_operator(&mut self, operator: &str) -> bool {
        if self.current_operator() == Some(operator) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn at_keyword(&self, keyword: &str) -> bool {
        self.peek_keyword(0, keyword)
    }

    fn peek_keyword(&self, lookahead: usize, keyword: &str) -> bool {
        matches!(
            self.tokens.get(self.cursor + lookahead).map(|token| &token.kind),
            Some(TokenKind::Word { value, quoted: false }) if value.eq_ignore_ascii_case(keyword)
        )
    }

    fn current_operator(&self) -> Option<&str> {
        match self.current().map(|token| &token.kind) {
            Some(TokenKind::Operator(value)) => Some(value),
            _ => None,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.cursor);
        self.cursor += usize::from(token.is_some());
        token
    }

    fn is_done(&self) -> bool {
        self.cursor == self.tokens.len()
    }

    fn syntax(&self, message: &str) -> QueryPlanError {
        error(
            QueryPlanErrorKind::Syntax,
            message,
            self.current().map_or(self.source_len, |token| token.offset),
        )
    }

    fn semantic(&self, message: &str) -> QueryPlanError {
        error(
            QueryPlanErrorKind::Semantic,
            message,
            self.current().map_or(self.source_len, |token| token.offset),
        )
    }
}

fn canonical_identifier(value: &str, quoted: bool) -> String {
    if quoted {
        value.to_string()
    } else {
        value.to_ascii_lowercase()
    }
}

fn is_clause_keyword(value: &str) -> bool {
    [
        "FROM", "WHERE", "GROUP", "ORDER", "LIMIT", "OFFSET", "ASC", "DESC", "SET", "VALUES",
        "AND", "OR", "IS", "NOT", "LIKE",
    ]
    .iter()
    .any(|keyword| value.eq_ignore_ascii_case(keyword))
}

fn aggregate_function(name: &str) -> Option<AggregateFunction> {
    match name.to_ascii_uppercase().as_str() {
        "COUNT" => Some(AggregateFunction::Count),
        "SUM" => Some(AggregateFunction::Sum),
        "AVG" => Some(AggregateFunction::Avg),
        "MIN" => Some(AggregateFunction::Min),
        "MAX" => Some(AggregateFunction::Max),
        "VAR_POP" | "VARIANCE_POP" => Some(AggregateFunction::VariancePopulation),
        "STDDEV_POP" => Some(AggregateFunction::StandardDeviationPopulation),
        _ => None,
    }
}

fn error(kind: QueryPlanErrorKind, message: impl Into<String>, offset: usize) -> QueryPlanError {
    QueryPlanError::new(kind, message, Some(offset))
}
