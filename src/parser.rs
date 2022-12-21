use crate::token::{Token, TokenValue};

/// Conditional struct, used for calculating if a conditional executions should be performed.
pub struct Condition {
    condition: Vec<Node>
}

impl Condition {
    pub fn new(condition: Vec<Node>) -> Condition {
        return Condition {
            condition
        }
    }

    pub fn calculate(&mut self) -> bool {
        if self.condition.len() as i32 == 1 {
            match &self.condition[0] {
                Node::Conditional(_) => {}
                Node::Invocation(_) => {}
                Node::Assign(_) => {}
                Node::Var(var_node) => {
                    match &var_node.value.value {
                        TokenValue::Identifier(_) => {}
                        TokenValue::Character(_) => {}
                        TokenValue::String(_) => {}
                        TokenValue::Char(_) => {}
                        TokenValue::Integer(integer_value) => {
                            return if integer_value == 0 {
                                false
                            } else {
                                true
                            };
                        }
                        TokenValue::Float(_) => {}
                        TokenValue::Boolean(boolean_value) => {
                            return boolean_value.to_owned();
                        }
                        TokenValue::EOF => {}
                    }
                }
                _ => {}
            }
        }

        return false;
    }
}

// Node types.
/// Conditional Node.
pub struct ConditionalNode {
    condition: Condition,
    body: Vec<Node>
}

impl ConditionalNode {
    pub fn new(condition: Condition, body: Vec<Node>) -> ConditionalNode {
        return ConditionalNode {
            condition,
            body
        };
    }
}

/// Function Invocation Node.
pub struct InvocationNode {
    method_name: String,
    parameters: Vec<Node>
}

impl InvocationNode {
    pub fn new(method_name: String, parameters: Vec<Node>) -> InvocationNode {
        return InvocationNode {
            method_name,
            parameters
        };
    }
}

/// Assign Node.
pub struct AssignNode {
    var_name: VarNode,
    value: Token,
    first_time: bool
}

impl AssignNode {
    pub fn new(var_name: VarNode, value: Token, first_time: bool) -> AssignNode {
        return AssignNode {
            var_name,
            value,
            first_time
        };
    }
}

/// Variable Node.
pub struct VarNode {
    var_name: String
}

impl VarNode {
    pub fn new(var_name: String) -> VarNode {
        return VarNode {
            var_name
        };
    }
}

/// Parameters Node.
pub struct ParamsNode {
    parameters: Vec<Node>
}

impl ParamsNode {
    pub fn new(parameters: Vec<Node>) -> ParamsNode {
        return ParamsNode {
            parameters
        };
    }
}

/// Function Node.
pub struct FuncNode {
    name: String,
    return_type: String,
    parameters: ParamsNode,
    body: Vec<Node>
}

impl FuncNode {
    pub fn new(name: String, return_type: String, parameters: ParamsNode, body: Vec<Node>)
        -> FuncNode {
        return FuncNode {
            name,
            return_type,
            parameters,
            body
        };
    }
}

// Node enum, to group all the nodes in a single vector.
pub enum Node {
    Conditional(ConditionalNode),
    Invocation(InvocationNode),
    Assign(AssignNode),
    Var(VarNode),
    Parameters(ParamsNode),
    Function(FuncNode)
}

// The parser takes the list of tokens, and transforms it into an AST (Abstract Syntax Tree), which
// is then used later for execution.
pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    token_id: i32
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let first_token: Token = tokens[0].into();

        return Parser {
            tokens,
            current_token: first_token,
            token_id: 0
        };
    }

    fn advance(&mut self) {
        self.token_id += 1;

        if self.token_id >= self.tokens.len() as i32 {
            self.current_token = self.tokens[self.tokens.len() - 1].into();
        }
        else {
            self.current_token = self.tokens[self.token_id];
        }
    }

    fn get_parameters(&mut self, open_char: char) -> Vec<Token> {
        let mut parameters: Vec<Token> = Vec::new();
        let close_char: char = match open_char {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            '<' => '>',
            _ => {
                panic!("Could not find matching closing character for: `{}`.", open_char);
            }
        };

        loop {
            match self.current_token.value {
                TokenValue::Character(character) => {
                    if character == close_char {
                        break;
                    }

                    parameters.append(self.current_token.into());
                }
                TokenValue::EOF => {
                    panic!("Could not get parameters, reached EOF!");
                }
                _ => {
                    parameters.append(self.current_token.into());
                }
            }

            self.advance();
        }

        self.advance();

        return parameters;
    }

    /// Get a list of tokens, starting at the current token, and going until it reaches an EOS (End
    /// of Statement) token (';').
    fn get_until_eos(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        self.advance();

        loop {
            match self.current_token.value {
                TokenValue::Character(character) => {
                    if character == ';' {
                        self.advance();
                        break;
                    }

                    tokens.push(self.current_token.into());
                }
                _ => {
                    tokens.push(self.current_token.into());
                }
            }

            self.advance();
        }

        return tokens;
    }

    /// Walk a given tree of tokens, recursively.
    fn walk(&mut self, tree: Vec<Token>) -> Vec<Node> {
        // The node tree (the AST).
        let mut node_tree: Vec<Node> = Vec::new();
        let mut current_token: Token = tree[0].into();
        let mut position: i32 = 0;

        let advance = || {
            position += 1;

            if position >= tree.len() as i32 {
                current_token = tree[tree.len() as i32 - 1];
            }
            else {
                current_token = tree[position];
            }
        };

        loop {
            // Go through each token and match them to grammatical patterns, then transform them
            // into Nodes, and add them to the node tree (the AST).

            match self.current_token.value.into() {
                TokenValue::Identifier(identifier) => {
                    let first_identifier: String = identifier;
                    self.advance();

                    match self.current_token.value.into() {
                        TokenValue::Identifier(identifier_2) => {
                            let second_identifier: String = identifier_2;

                            self.advance();

                            match self.current_token.value.into() {
                                TokenValue::Character(character) => {
                                    if character == '(' {
                                        // Function declaration.
                                        self.advance();
                                        let parameters: Vec<Node> = self.walk(
                                            self.get_parameters('(')
                                        );

                                        match self.current_token.value.into() {
                                            TokenValue::Character(character) => {
                                                if character == '{' {
                                                    self.advance();
                                                    let body: Vec<Node> = self.walk(
                                                        self.get_parameters('{')
                                                    );

                                                    let parameters: ParamsNode = ParamsNode::new(
                                                        parameters
                                                    );

                                                    let node: FuncNode = FuncNode::new(
                                                        name: identifier,
                                                        return_type: identifier2,
                                                        parameters,
                                                        body
                                                    );

                                                    node_tree.push(Node::Function(node));
                                                }
                                                else {
                                                    panic!("Expected: '{{'!");
                                                }
                                            }
                                        }
                                    }
                                    else {
                                        panic!("Unexpected character: {}", character);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return node_tree;
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut ast = self.walk(self.tokens.to_vec());

        return ast;
    }
}