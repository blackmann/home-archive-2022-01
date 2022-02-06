use std::{fs};
use std::error::Error;
use std::fs::read_dir;
use std::io::{Read, Write};
use std::path::{Path};
use std::sync::mpsc::channel;
use std::time::Duration;
use comrak::{Arena, ComrakOptions, format_html, parse_document};
use comrak::nodes::NodeValue;
use notify::{DebouncedEvent, RecursiveMode, watcher, Watcher};
use regex::Regex;
use rsass::{compile_scss_path};
use serde::{Deserialize, Serialize};

const FRONT_MATTER_DELIMITER: &str = "---";
const POSTS_LAYOUT: &str = "./layouts/post.liquid";
const HOME_LAYOUT: &str = "./layouts/home.liquid";
const MAIN_LAYOUT: &str = "./layouts/main.liquid";

#[derive(Debug, Serialize, Deserialize)]
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

    fn to_human_str(&self) -> String {
        let months = vec!["January", "February", "March", "April",
                         "May", "June", "July", "August", "September", "October", "November", "December"];

        let month_post: usize = (self.month - 1) as usize;
        return format!("{} {} {}", self.day, months[month_post], self.year);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PostMeta {
    date: Option<Date>,
    draft: bool,
    next: Option<String>,
    slug: Option<String>,
    title: Option<String>,
    title_meta: Option<String>,
}

impl PostMeta {
    fn new() -> PostMeta {
        PostMeta {
            date: None,
            draft: false,
            next: None,
            slug: None,
            title: None,
            title_meta: None,
        }
    }

    fn parse(matter: &str) -> PostMeta {
        let lines = matter.split('\n');

        let mut post_meta = PostMeta::new();

        for row in lines {
            if row != FRONT_MATTER_DELIMITER && !row.is_empty() {
                let key_value: Vec<&str> = row.splitn(2, ':').collect();

                let key = key_value[0];
                let value = key_value[1].trim();

                match key {
                    "title" => post_meta.title = Some(String::from(value)),
                    "slug" => post_meta.slug = Some(String::from(value)),
                    "draft" => post_meta.draft = value == "true",
                    "date" => post_meta.date = Some(Date::parse(value)),
                    "title_meta" => post_meta.title_meta = Some(String::from(value)),
                    "next" => post_meta.next = Some(String::from(value)),
                    _ => ()
                }
            }
        }

        post_meta
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
        self.posts.clear();

        // fetch all posts and derive their front matter
        let directory_content = read_dir("./posts")?;
        let md_file_regex = Regex::new(r".+\.md")?;

        for path in directory_content {
            let dir_entry = path.expect("Invalid directory entry");
            let metadata = dir_entry.metadata()?;

            let file_name = dir_entry.file_name();

            if metadata.is_file() && md_file_regex.is_match(file_name.to_str().unwrap()) {
                let mut file = fs::File::options().read(true).open(dir_entry.path())?;

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

                format_html(root, &comrak_options, &mut html).expect("Failed to convert to html");

                post.raw_content = Some(String::from_utf8(html)?);

                self.posts.push(post);
            }
        }

        Ok(())
    }

    fn build_homepage(&self) -> Result<(), Box<dyn Error>> {
        println!("   Building home");
        let template_str = read_file_to_string(HOME_LAYOUT)?;
        let final_template_str = read_file_to_string(MAIN_LAYOUT)?;

        let main_template = liquid::ParserBuilder::with_stdlib().build()
            .unwrap().parse(final_template_str.as_str()).unwrap();

        let content_template = liquid::ParserBuilder::with_stdlib().build()
            .unwrap().parse(template_str.as_str()).unwrap();

        let globals = liquid::object!({
            "posts": self.posts
        });

        let content_output = content_template.render(&globals).unwrap();

        let main_globals = liquid::object!({
            "content": content_output,
            "description": "",
            "title": "Welcome",
        });

        let final_output = main_template.render(&main_globals).unwrap();

        fs::create_dir_all("dist")?;

        let mut out_file = fs::File::options().write(true)
            .truncate(true).create(true).open("dist/index.html")?;

        out_file.write_all(final_output.as_bytes())?;

        println!("🏡 Written home");

        Ok(())
    }

    fn build_posts(&self) -> Result<(), Box<dyn Error>> {
        println!("   Building posts");

        let template_str = read_file_to_string(POSTS_LAYOUT)?;

        let final_template_str = read_file_to_string(MAIN_LAYOUT)?;

        let post_template = liquid::ParserBuilder::with_stdlib()
            .build().unwrap().parse(template_str.as_str()).unwrap();

        let final_template = liquid::ParserBuilder::with_stdlib()
            .build().unwrap().parse(final_template_str.as_str()).unwrap();

        for post in self.posts.iter() {
            let title = post.meta.as_ref().unwrap().title.as_ref().unwrap();
            let slug = post.meta.as_ref().unwrap().slug.as_ref().unwrap();
            let date = post.meta.as_ref().unwrap().date.as_ref().unwrap().to_human_str();

            let post_globals = liquid::object!({
                "date": date,
                "title": title,
                "content": post.raw_content.as_ref().unwrap(),
                "posts": self.posts,
                "slug": slug
            });

            let post_output = post_template.render(&post_globals).unwrap();

            let final_globals = liquid::object!({
                "content": post_output,
                "description": "",
                "title": title
            });

            let final_output = final_template.render(&final_globals).unwrap();

            fs::create_dir_all("dist/posts/")?;

            let file_name = format!("dist/posts/{}.html",
                                    post.meta.as_ref().unwrap().slug.as_ref().unwrap());

            let mut out_file = fs::File::options().write(true).create(true)
                .truncate(true).open(String::from(&file_name))?;

            out_file.write_all(final_output.as_bytes())?;

            println!("📒 Written {}", file_name);
        }

        Ok(())
    }

    fn process_asset(path: &Path, sub_folder: &str) -> Result<(), Box<dyn Error>> {
        let scss_regex = Regex::new(r".+\.scss")?;

        let sub_folder_dir = format!("dist/static/{}", sub_folder);

        fs::create_dir_all(sub_folder_dir.as_str())?;

        if scss_regex.is_match(path.file_name().unwrap().to_str().unwrap()) {
            let css = compile_scss_path(path, Default::default())?;

            let splits: Vec<&str> = path.file_name().unwrap().to_str().to_owned().unwrap().split('.').collect();
            let css_file_name = format!("{}/{}.css", sub_folder_dir, splits[0]);

            let mut css_file = fs::File::options().write(true)
                .truncate(true).create(true).open(css_file_name)?;

            css_file.write_all(&css)?;
        } else {
            let file_name = format!("{}/{}",
                                    &sub_folder_dir, path.file_name().unwrap().to_str().unwrap());
            fs::copy(path, file_name)?;
        }

        Ok(())
    }

    fn pack_assets(&self) -> Result<(), Box<dyn Error>> {
        fn traverse(dir: &Path, sub_folder_ref: &str) -> Result<(), Box<dyn Error>> {
            if dir.is_dir() {
                let entries = read_dir(dir)?;

                for entry in entries {
                    let entry = entry?;
                    let file_name = entry.file_name().to_str().unwrap().to_owned();

                    if entry.path().is_dir() {
                        let sub_folder = format!("{}/{}", sub_folder_ref, file_name);
                        traverse(&entry.path(), sub_folder.as_str())?;
                    } else {
                        BuildInstance::process_asset(&entry.path(), sub_folder_ref)?;
                    }
                }
            }

            Ok(())
        }

        traverse(Path::new("./assets"), "")?;

        Ok(())
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.initialize()?;
        self.build_homepage()?;
        self.build_posts()?;
        self.pack_assets()?;

        Ok(())
    }
}

fn read_file_to_string(path: &str) -> Result<String, Box<dyn Error>> {
    let mut final_template = fs::File::options().read(true).open(path)?;
    let mut res = String::new();

    final_template.read_to_string(&mut res)?;

    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut build_instance = BuildInstance::new();
    build_instance.run()?;

    println!("Running in watch mode by default");
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(5))?;
    watcher.watch(".", RecursiveMode::Recursive)?;

    let accept_pattern = Regex::new(r"(posts|layouts|assets)/")?;

    // adding this to force dist from being watched because dist contains a
    // posts/ dir which matches the accept_pattern
    let ignore_pattern = Regex::new(r"(dist)")?;

    loop {
        match rx.recv() {
            Ok(event) => {
                if let DebouncedEvent::NoticeWrite(w) = event {
                    let path = w.to_str().unwrap();
                    if accept_pattern.is_match(path) && !ignore_pattern.is_match(path) {
                        println!("File updated {:?}. Rebuilding...", w);

                        build_instance.run()?;

                        println!("\n✅  Done\n");
                    }
                }
            }

            Err(e) => {
                println!("An error occurred: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
