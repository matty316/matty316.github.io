use comrak::{markdown_to_html, ComrakOptions};
use std::fs;

fn main() {
    build_home();
    build_blog();
    build_social();
}

fn build_page(path: &str, content: &str) {
    fs::remove_file(path).unwrap();
    let header = fs::read_to_string("templates/header.html").unwrap();

    let footer = fs::read_to_string("templates/footer.html").unwrap();
    let html = format!("{header} {content} {footer}");
    fs::write(path, html).unwrap();
}

fn build_home() {
    let content = fs::read_to_string("templates/index.html").unwrap();
    build_page("index.html", &content);
}

fn build_blog() {
    let paths = fs::read_dir("posts").unwrap();

    let mut result = "<div class=\"container\"><div class=\"columns is-multiline\">".to_string();
    for path in paths {
        let name = format!("{}", path.unwrap().path().display());
        let content = fs::read_to_string(name).unwrap();
        result += &fs::read_to_string("templates/blog/post-header.html").unwrap();
        let html = markdown_to_html(&content, &ComrakOptions::default());
        result += &html;
        result += &fs::read_to_string("templates/blog/post-footer.html").unwrap();
    }
    result += "</div></div>";

    build_page("blog.html", &result);
}

fn build_social() {
    let paths = fs::read_dir("social").unwrap();
    let mut result = fs::read_to_string("templates/social/header.html").unwrap();
    for path in paths {
        let name = format!("{}", path.unwrap().path().display());
        let content = fs::read_to_string(name).unwrap();
        result += &fs::read_to_string("templates/social/post-header.html").unwrap();
        result += &content;
        result += &fs::read_to_string("templates/social/post-footer.html").unwrap();
    }
    result += "</div>";
    build_page("social.html", &result);
}

