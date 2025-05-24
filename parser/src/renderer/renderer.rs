use crate::parser::parser::{Node, NodeType};

fn render_heading(order: u8, txt: String) -> String {
  format!("<h{}>{}</h{}>", order, txt, order)
}

fn render_bold(txt: String) -> String {
  format!("<strong>{}</strong>", txt)
}

fn render_italic(txt: String) -> String {
 format!("<i>{}</i>", txt) 
}

fn render_list_item(txt: String) -> String {
  format!("<li>{}</li>", txt)
}

fn render_list(list: Node) -> String {
  let mut html = String::from("<ul>");

  for i in list.children {
    match i.ntype {
      NodeType::ListItem(txt) => html.push_str(&render_list_item(txt)),
      _ => {}
    };
  }

  html.push_str("</ul>");

  html
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
        crate::parser::parser::NodeType::NewLine => html.push_str("<br />"),
        crate::parser::parser::NodeType::Character(c) => html.push(c),
        crate::parser::parser::NodeType::List => html.push_str(&render_list(c)),
        _ => {}
    }
  }

  html
}