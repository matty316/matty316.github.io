use comrak::{markdown_to_html, ComrakOptions};
use std::fs;

fn main() {
    let contents = fs::read_to_string("posts/post.md").unwrap();
    let html = markdown_to_html(&contents, &ComrakOptions::default());
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
    print!("{html:?}");
}

fn build_pages(header: &str, footer: &str) {
    let pages = vec![
        "index",
        "social",
        "blog",
    ];

    for p in pages {}
}

fn build_home(header: &str, footer: &str) {
    let content = r#"<img class="main-img" src="img/main.jpeg"><p class="is-size-3 has-text-centered section has-text-weight-semibold">hi, i'm matty 🙏🏿. i like to make apps 📱, make people laugh 🤣, make powerpoints 📊, make bread 🥖🥐🍞, make clothes 🧶 and make enemies 😈.</p>"#;

}


