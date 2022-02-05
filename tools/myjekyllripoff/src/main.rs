use std::{env, fs};
use std::error::Error;
use std::fs::read_dir;
use std::io::{Read, Write};
use comrak::{Arena, ComrakOptions, format_html, parse_document};
use comrak::nodes::NodeValue;
use regex::Regex;

const FRONT_MATTER_DELIMITER: &str = "---";

#[derive(Debug)]
struct Date {
    day: i32,
    month: i32,
    year: i32,
}

impl Date {
    fn parse(date: &str) -> Date {
        let parts: Vec<&str> = date.trim().split('-').collect();

        Date {
            day: parts[0].parse::<i32>().unwrap(),
            month: parts[1].parse::<i32>().unwrap(),
            year: parts[2].parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct PostMeta {
    date: Option<Date>,
    draft: bool,
    slug: Option<String>,
    title: Option<String>,
}

impl PostMeta {
    fn new() -> PostMeta {
        PostMeta {
            date: None,
            draft: false,
            slug: None,
            title: None,
        }
    }

    fn parse(matter: &str) -> PostMeta {
        let lines = matter.split('\n');

        let mut post_meta = PostMeta::new();

        for row in lines {
            if row != FRONT_MATTER_DELIMITER && !row.is_empty() {
                let key_value: Vec<&str> = row.splitn(2, ':').collect();

                let key = key_value[0];
                let value = key_value[1];

                match key {
                    "title" => post_meta.title = Some(String::from(value)),
                    "slug" => post_meta.slug = Some(String::from(value)),
                    "draft" => post_meta.draft = value == "true",
                    "date" => post_meta.date = Some(Date::parse(value)),
                    _ => ()
                }
            }
        }

        post_meta
    }
}

#[derive(Debug)]
struct Post {
    raw_content: Option<String>,
    meta: Option<PostMeta>,
}

struct BuildInstance {
    posts: Vec<Post>,
}

impl BuildInstance {
    fn new() -> BuildInstance {
        BuildInstance {
            posts: vec![],
        }
    }

    fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        // fetch all posts and derive their front matter
        let directory_content = read_dir(env::current_dir()?)?;
        let md_file_regex = Regex::new(r".+\.md");

        for path in directory_content {
            let dir_entry = path.expect("Invalid directory entry");
            let metadata = dir_entry.metadata()?;

            let file_name = dir_entry.file_name();

            if metadata.is_file() && md_file_regex.as_ref().unwrap()
                .is_match(file_name.to_str().unwrap()) {
                let mut file = fs::File::options().read(true).open(file_name)?;

                let mut md_content: String = String::new();
                file.read_to_string(&mut md_content)?;

                let arena = Arena::new();

                let mut comrak_options = ComrakOptions::default();
                comrak_options.extension.front_matter_delimiter = Some(String::from(FRONT_MATTER_DELIMITER));
                comrak_options.render.unsafe_ = true;

                let root = parse_document(&arena,
                                          md_content.as_str(),
                                          &comrak_options);

                let mut post = Post {
                    meta: None,
                    raw_content: None,
                };

                for node in root.children() {
                    if let &mut NodeValue::FrontMatter(ref mut front_matter)
                    = &mut node.data.borrow_mut().value {
                        post.meta = Some(PostMeta::parse(&String::from_utf8(front_matter.to_owned())?));
                    }
                }

                let mut html = vec![];

                format_html(root, &comrak_options, &mut html)?;

                post.raw_content = Some(String::from_utf8(html)?);

                self.posts.push(post);
            }
        }

        Ok(())
    }

    fn build_homepage(&self) {}

    fn build_posts(&self) -> Result<(), Box<dyn Error>> {
        let mut template_file = fs::File::options().read(true).open("post.liquid")?;
        let mut template_str = String::new();

        template_file.read_to_string(&mut template_str)?;

        let template = liquid::ParserBuilder::with_stdlib().build().unwrap().parse(template_str.as_str()).unwrap();

        for post in self.posts.iter() {
            let globals = liquid::object!({
                "title": post.meta.as_ref().unwrap().title,
                "content": post.raw_content.as_ref().unwrap(),
            });

            let output = template.render(&globals).unwrap();

            let file_name = format!("{}.html", post.meta.as_ref().unwrap().slug.as_ref().unwrap());
            let mut out_file = fs::File::options().write(true).create(true).truncate(true).open(String::from(&file_name))?;

            out_file.write_all(output.as_bytes())?;

            println!("📒 Written {}", file_name);
        }

        Ok(())
    }
}

fn transform_to_html() -> String {
    String::from("Hello")
}

fn main() {
    let mut build_instance = BuildInstance::new();
    build_instance.initialize();
    // build_instance.build_homepage();
    build_instance.build_posts();

    println!("\n✅  Done");
}
