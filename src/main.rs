use comrak::{markdown_to_html, ComrakOptions};
use std::fs;

fn main() {
    build_home();
    build_blog();
    build_social();
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
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer" />
    <link rel="stylesheet" href="css/style.css">
  </head>
  <body>
    <nav class="navbar hero has-background-dark is-transparent" role="navigation" aria-label="main navigation">
      <div class="hero-body">
        <div class="navbar-brand center">
          <a class="navbar-item has-text-light" href="social.html">social</a>
          <a class="navbar-item has-text-light has-text-weight-bold is-flex is-size-1" href="index.html">💀 matty 3:16 ✊🏿</a>
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
        let html = markdown_to_html(&content, &ComrakOptions::default());
        result += &html;
    }
    result += "</div>";

    build_page("blog.html", &result);
}

fn build_social() {
    let paths = fs::read_dir("social").unwrap();
    let mut result = r#"<div class="container"><nav class="level">
  <div class="level-item has-text-centered">
    <div>
      <p class="heading">Tweets</p>
      <p class="title">1,102,436</p>
    </div>
  </div>
  <div class="level-item has-text-centered">
    <div>
      <p class="heading">Following</p>
      <p class="title">0</p>
    </div>
  </div>
  <div class="level-item has-text-centered">
    <div>
      <p class="heading">Followers</p>
      <p class="title">1 Billion</p>
    </div>
  </div>
  <div class="level-item has-text-centered">
    <div>
      <p class="heading">Likes</p>
      <p class="title">Nobody</p>
    </div>
  </div>
</nav>"#.to_string();
    for path in paths {
        let name = format!("{}", path.unwrap().path().display());
        let content = fs::read_to_string(name).unwrap();
        let html = format!(r#"<div class="box">
  <article class="media">
    <div class="media-left">
      <figure class="image is-64x64">
        <img src="img/main.jpeg" alt="Image">
      </figure>
    </div>
    <div class="media-content">
      <div class="content">
        <p>
          <strong>💀 matty 3:16 ✊🏿</strong> <small>@matty316</small> <small>∞ ago</small>
          <br>
          {}
        </p>
      </div>
      <nav class="level is-mobile">
        <div class="level-left">
          <a class="level-item" aria-label="reply">
            <span class="icon is-small">
              <i class="fas fa-reply" aria-hidden="true"></i>
            </span>
          </a>
          <a class="level-item" aria-label="retweet">
            <span class="icon is-small">
              <i class="fas fa-retweet" aria-hidden="true"></i>
            </span>
          </a>
          <a class="level-item" aria-label="like">
            <span class="icon is-small">
              <i class="fas fa-heart" aria-hidden="true"></i>
            </span>
          </a>
        </div>
      </nav>
    </div>
  </article>
</div>"#, content);
        result += &html;
    }
    result += "</div>";
    build_page("social.html", &result);
}

