use renderer::renderer::render;
use tokenizer::tokenizer::tokenize;
use wasm_bindgen::prelude::*;

pub mod parser;
pub mod tokenizer;
pub mod renderer;

#[wasm_bindgen]
pub struct ParseResult {
    html: String,
    ast: String,
}

#[wasm_bindgen]
impl ParseResult {
    #[wasm_bindgen(constructor)]
    pub fn new(html: String, ast: String) -> ParseResult {
        ParseResult {html, ast }
    }

    #[wasm_bindgen(getter)]
    pub fn html(&self) -> String {
        self.html.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn ast(&self) -> String {
        self.ast.clone()
    }
}

#[wasm_bindgen]
pub fn parse(input: &str) -> ParseResult {
    let mut tokens = tokenize(input);

    tokens.reverse();

    let node = parser::parser::parse(tokens);

    let ast = node.to_pretty_string(0);
    let html = render(node);
  
    ParseResult { html, ast }
}