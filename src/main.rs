use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
extern crate yaml_rust;
use yaml_rust::YamlLoader;

fn main() {
    build_home();
    build_blog();
    build_social();
}

fn build_page(path: &str, content: &str) {
    fs::remove_file(path).ok();
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
    let paths = fs::read_dir("posts/markdown").unwrap();

    let mut result = "<div class=\"container\"><div class=\"columns is-multiline\">".to_string();
    for path in paths {
        let name = format!("{}", path.unwrap().path().display());
        result += &fs::read_to_string("templates/blog/post-header.html").unwrap();
        let content = fs::read_to_string(name).unwrap();
        let yaml = content.split("---").collect::<Vec<&str>>()[1];
        let meta = YamlLoader::load_from_str(yaml).unwrap()[0].clone();
        let title = meta["title"].as_str().unwrap();
        let desc = meta["description"].as_str().unwrap();
        let slug = meta["slug"].as_str().unwrap();
        let date = meta["date"].as_str().unwrap();
        result += format!("<h1><a href=\"{}.html\">{}</a></h1><small>{}</small>", slug, title, date).as_str();
        result += format!("<p>{}</p>", desc).as_str();
        let markdown = content.split("---").collect::<Vec<&str>>()[2];
        let html = markdown_to_html(markdown, &ComrakOptions::default());
        let post_path = format!("{}.html", slug);
        fs::remove_file(post_path.clone()).ok();
        build_page(&post_path, &format!("<div class=\"content container\"><h1>{}</h1>{}</div>", title, html));

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

