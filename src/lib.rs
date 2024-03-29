mod blogs;
mod posts;

use self::blogs::Blog;
use self::posts::Post;
use handlebars::{handlebars_helper, Handlebars};
use serde_derive::Serialize;
use serde_json::json;
use std::convert::AsRef;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

struct Generator<'a> {
    handlebars: Handlebars<'a>,
    blogs: Vec<Blog>,
    out_directory: PathBuf,
}

#[derive(Debug, Serialize)]
struct Releases {
    releases: Vec<ReleasePost>,
    feed_updated: String,
}

#[derive(Debug, Serialize)]
struct ReleasePost {
    title: String,
    url: String,
}
handlebars_helper!(hb_month_name_helper: |month_num: u64| match month_num {
    1 => "Jan.",
    2 => "Feb.",
    3 => "Mar.",
    4 => "Apr.",
    5 => "May",
    6 => "June",
    7 => "July",
    8 => "Aug.",
    9 => "Sept.",
    10 => "Oct.",
    11 => "Nov.",
    12 => "Dec.",
    _ => "Error!",
});

impl<'a> Generator<'a> {
    fn new(
        out_directory: impl AsRef<Path>,
        posts_directory: impl AsRef<Path>,
    ) -> eyre::Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_templates_directory(".hbs", "templates")?;
        handlebars.register_helper("month_name", Box::new(hb_month_name_helper));

        Ok(Generator {
            handlebars,
            blogs: self::blogs::load(posts_directory.as_ref())?,
            out_directory: out_directory.as_ref().into(),
        })
    }

    fn render(&self) -> eyre::Result<()> {
        // make sure our output directory exists
        fs::create_dir_all(&self.out_directory)?;

        for blog in &self.blogs {
            self.render_blog(blog)?;
        }
        self.compile_sass("app");
        self.compile_sass("fonts");
        self.concat_vendor_css(vec!["skeleton", "tachyons"]);
        self.copy_static_files()?;
        Ok(())
    }

    fn compile_sass(&self, filename: &str) {
        let scss_file = format!("./src/styles/{}.scss", filename);
        let css_file = format!("./static/styles/{}.css", filename);

        let css = grass::from_path(&scss_file, &grass::Options::default())
            .unwrap_or_else(|_| panic!("couldn't compile sass: {}", &scss_file));
        let mut file =
            File::create(&css_file).unwrap_or_else(|_| panic!("couldn't make css file: {}", &css_file));
        file.write_all(&css.into_bytes())
            .unwrap_or_else(|_| panic!("couldn't write css file: {}", &css_file));
    }

    fn concat_vendor_css(&self, files: Vec<&str>) {
        let mut concatted = String::new();
        for filestem in files {
            let vendor_path = format!("./static/styles/{}.css", filestem);
            let contents = fs::read_to_string(vendor_path).expect("couldn't read vendor css");
            concatted.push_str(&contents);
        }
        fs::write("./static/styles/vendor.css", &concatted).expect("couldn't write vendor css");
    }

    fn render_blog(&self, blog: &Blog) -> eyre::Result<()> {
        std::fs::create_dir_all(self.out_directory.join(blog.prefix()))?;

        self.render_index(blog)?;
        self.render_feed(blog)?;
        self.render_releases_feed(blog)?;
        for post in blog.posts() {
            self.render_post(blog, post)?;
        }
        Ok(())
    }

    fn render_index(&self, blog: &Blog) -> eyre::Result<()> {
        let other_blogs: Vec<_> = self
            .blogs
            .iter()
            .filter(|b| b.index_title() != blog.index_title())
            .map(|other_blog| {
                json!({
                    "link_text": other_blog.link_text(),
                    "url": PathBuf::from("/").join(other_blog.prefix()).join("index.html"),
                })
            })
            .collect();

        let data = json!({
            "title": blog.index_title(),
            "parent": "layout",
            "blog": blog,
            "other_blogs": other_blogs,
        });
        self.render_template(blog.prefix().join("index.html"), "index", data)?;
        Ok(())
    }

    fn render_post(&self, blog: &Blog, post: &Post) -> eyre::Result<()> {
        let path = blog
            .prefix()
            .join(format!("{:04}", &post.year))
            .join(format!("{:02}", &post.month))
            .join(format!("{:02}", &post.day));
        fs::create_dir_all(self.out_directory.join(&path))?;

        // then, we render the page in that path
        let mut filename = PathBuf::from(&post.filename);
        filename.set_extension("html");

        let data = json!({
            "title": format!("{} | {}", post.title, blog.title()),
            "parent": "layout",
            "blog": blog,
            "post": post,
        });

        self.render_template(path.join(filename), &post.layout, data)?;
        Ok(())
    }

    fn render_feed(&self, blog: &Blog) -> eyre::Result<()> {
        let posts: Vec<_> = blog.posts().iter().take(10).collect();
        let data = json!({
            "blog": blog,
            "posts": posts,
            "feed_updated": chrono::Utc::now().to_rfc3339(),
        });

        self.render_template(blog.prefix().join("feed.xml"), "feed", data)?;
        Ok(())
    }

    fn render_releases_feed(&self, blog: &Blog) -> eyre::Result<()> {
        let posts = blog.posts().to_vec();
        let is_released: Vec<&Post> = posts.iter().filter(|post| post.release).collect();
        let releases: Vec<ReleasePost> = is_released
            .iter()
            .map(|post| ReleasePost {
                title: post.title.clone(),
                url: blog
                    .prefix()
                    .join(post.url.clone())
                    .to_string_lossy()
                    .to_string(),
            })
            .collect();
        let data = Releases {
            releases,
            feed_updated: chrono::Utc::now().to_rfc3339(),
        };
        fs::write(
            self.out_directory.join(blog.prefix()).join("releases.json"),
            serde_json::to_string(&data)?,
        )?;
        Ok(())
    }

    fn copy_static_files(&self) -> eyre::Result<()> {
        use fs_extra::dir::{self, CopyOptions};

        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.copy_inside = true;

        dir::copy("static/fonts", &self.out_directory, &options)?;
        dir::copy("static/images", &self.out_directory, &options)?;
        dir::copy("static/styles", &self.out_directory, &options)?;
        dir::copy("static/scripts", &self.out_directory, &options)?;

        Ok(())
    }

    fn render_template(
        &self,
        name: impl AsRef<Path>,
        template: &str,
        data: serde_json::Value,
    ) -> eyre::Result<()> {
        let out_file = self.out_directory.join(name.as_ref());
        let file = File::create(out_file)?;
        self.handlebars.render_to_write(template, &data, file)?;
        Ok(())
    }
}

pub fn main() -> eyre::Result<()> {
    let blog = Generator::new("site", "posts")?;

    blog.render()?;

    Ok(())
}
