use crate::parser::parser::Node;

fn render_heading(order: u8, txt: String) -> String {
  format!("<h{}>{}</h{}>", order, txt, order)
}

fn render_bold(txt: String) -> String {
  format!("<strong>{}</strong>", txt)
}

fn render_italic(txt: String) -> String {
 format!("<i>{}</i>", txt) 
}

pub fn render(node: Node) -> String {
  let mut html = String::new();

  for c in node.children {
    match c.ntype {
        crate::parser::parser::NodeType::Doc => {},
        crate::parser::parser::NodeType::Text(v) => html.push_str(&v),
        crate::parser::parser::NodeType::Heading(order, txt) => html.push_str(&render_heading(order, txt)),
        crate::parser::parser::NodeType::Bold(v) => html.push_str(&render_bold(v)),
        crate::parser::parser::NodeType::Italic(v) => html.push_str(&&render_italic(v)),
        crate::parser::parser::NodeType::NewLine => html.push('\n'),
        crate::parser::parser::NodeType::Character(c) => html.push(c),
    }
  }

  html
}