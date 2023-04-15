use comrak::{markdown_to_html, ComrakOptions};
use std::fs;

fn main() {
    build_home();
    build_blog();
}

fn build_page(path: &str, content: &str) {
    fs::remove_file(path).unwrap();
    let header = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>matty 3:16</title>
    <link rel="stylesheet" href="css/style.css">
  </head>
  <body>
    <nav class="navbar hero has-background-dark is-transparent" role="navigation" aria-label="main navigation">
      <div class="hero-body">
        <div class="navbar-brand center">
          <a class="navbar-item has-text-light" href="social.html">social</a>
          <a class="navbar-item has-text-light has-text-weight-bold is-flex is-size-1" href="/">💀 matty 3:16 ✊🏿</a>
          <a class="navbar-item has-text-light" href="blog.html">blog</a>
        </div>
      </div>
    </nav>"#;

    let footer = r#"<footer class="footer">
    <div class="content has-text-centered">
      <p>
        follow <strong>💀 matty 3:16 ✊🏿</strong> on <a href="https://www.youtube.com/channel/UCaYEz0nbJzHgaIR4QtK7Xkg">youtube</a> and <a href="https://github.com/matty316">github</a>.
      </p>
    </div>
  </footer>
  </body>
</html>"#;
    let html = format!("{header} {content} {footer}");
    fs::write(path, html).unwrap();
}

fn build_home() {
    let content = r#"<img class="main-img" src="img/main.jpeg"><p class="is-size-3 has-text-centered section has-text-weight-semibold">hi, i'm matty 🙏🏿. i like to make apps 📱, make people laugh 🤣, make powerpoints 📊, make bread 🥖🥐🍞, make clothes 🧶 and make enemies 😈.</p>"#;
    build_page("index.html", content);
}

fn build_blog() {
    let paths = fs::read_dir("posts").unwrap();

    let mut result = "<div class=\"content container\">".to_string();
    for path in paths {
        let name = format!("{}", path.unwrap().path().display());
        let content = fs::read_to_string(name).unwrap();
        let format = content;
        let html = markdown_to_html(&format, &ComrakOptions::default());
        result += &html;
    }
    result += "</div>";

    build_page("blog.html", &result);
}

