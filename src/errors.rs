

pub fn general(desc: &str) -> String { format!("{{type: \"GEN\", description: \"{}\"}}", desc) }

pub fn no_active_connection(comment: Option<String>) -> String { 
  let cmt = match comment {
    Some(c) => c,
    None => String::from("null")
  };

  format!("{{type: \"CONN\", description: \"No connection has previously been established\", comment: \"{}\"}}", cmt) 
}