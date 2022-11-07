use comrak::{markdown_to_html, ComrakOptions};
use handlebars::Handlebars;
use serde_json::json;
use slug::slugify;
use std::fs;

struct PostTitle {
    title: String,
    slug: String,
}

fn main() {
    let post_markdown_files = fs::read_dir("./posts")
        .expect("The directory posts does not exist.")
        .filter(|entry| match entry.as_ref().unwrap().path().extension() {
            Some(extension) => extension == "md",
            None => false,
        });

    fs::create_dir_all("./dist").unwrap();

    let mut posts: Vec<PostTitle> = Vec::new();

    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();

    for post in post_markdown_files {
        let unwrapped_post = post.expect("There was an error reading directory entries.");

        let post_path = unwrapped_post.path();

        let content = fs::read_to_string(&post_path)
            .expect(format!("There was an error reading {:?}", &post_path).as_str());

        let mut post_title_chars = content.lines().next().expect("Post was empty.").chars();

        // Skip two iterator steps to remove the "# "
        post_title_chars.next();
        post_title_chars.next();

        let post_title = post_title_chars.as_str();

        let post_content_html = markdown_to_html(content.as_str(), &ComrakOptions::default());

        let post_html = handlebars
            .render("post", &json!({ "content": post_content_html }))
            .unwrap();

        let post_slug = slugify(post_title);

        posts.push(PostTitle {
            title: String::from(post_title),
            slug: post_slug.clone(),
        });

        let post_path = format!("./dist/{}.html", post_slug);

        fs::write(post_path, post_html).unwrap();
    }

    let mut posts_html = String::new();

    for post_title in posts {
        let post_item_html = format!(
            "<li><a href=\"./{}.html\">{}</a></li>",
            post_title.slug, post_title.title
        );

        posts_html = format!("{}{}", posts_html, post_item_html);
    }

    let home_content = handlebars
        .render("home", &json!({ "posts": posts_html }))
        .unwrap();

    fs::write("./dist/index.html", home_content).unwrap();
}
