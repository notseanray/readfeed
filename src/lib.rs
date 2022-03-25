use chrono::{DateTime, Utc};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::os::unix::prelude::OsStrExt;
use std::time::SystemTime;
use std::{fmt, ops::Deref, path::PathBuf, str::FromStr};

pub struct Feed {
    title: String,
    path: PathBuf,
    output: PathBuf,
    url_base: String,
    page_folder: String,
    data: String,
    description: Option<String>,
}

// THIS WILL MAKE YOUR XML PARSER CRY

impl Feed {
    pub fn new<'a, T: Deref<Target = &'a str>>(
        title: &str,
        path: T,
        output: Option<T>,
        page_folder: &str,
        url_base: &str,
        description: Option<&str>,
    ) -> Self {
        let in_path = PathBuf::from_str(&path).expect("invalid path");
        let out_file = match output {
            Some(v) => PathBuf::from_str(&v).expect("invalid path"),
            None => PathBuf::from(&in_path.join("rss.xml")),
        };
        let description = match description {
            Some(v) => Some(v.to_string()),
            None => None,
        };
        Feed {
            title: title.to_string(),
            path: in_path,
            output: out_file,
            url_base: url_base.to_string(),
            page_folder: page_folder.to_string(),
            data: String::new(),
            description,
        }
    }

    pub fn generate(&mut self) -> std::io::Result<()> {
        let mut data = format!(
            r#"<rss version="2.0">
    <channel>
        <title>{}</title>
        <link>{}</link>"#,
            self.title, self.url_base
        );
        match &self.description {
            Some(v) => data.push_str(&format!(
                "
        <description>{v}</description>"
            )),
            None => {}
        };
        data.push_str(
            "
        <language>en-us</language>",
        );
        for file in self.path.join(&self.page_folder).read_dir()? {
            if let Ok(v) = file {
                let path = v.path();
                let extension = match path.as_path().extension() {
                    Some(v) => v,
                    None => continue,
                };
                if extension != OsStr::from_bytes(b"html") {
                    continue;
                }
                let date = match v.metadata() {
                    Ok(v) => v.modified().unwrap(),
                    Err(_) => SystemTime::now(),
                };
                let pub_date = DateTime::<Utc>::from(date).to_rfc2822();
                let fname = v.file_name().to_string_lossy().to_string();
                let article = format!(
                    "
        <item>
            <title>{}</title>
            <guid>{}</guid>
            <pubDate>{}</pubDate>
        </item>",
                    fname.split(".").nth(0).unwrap().replace("_", " "),
                    format!("{}/{}/{}", self.url_base, self.page_folder, fname),
                    format!("{} GMT", &pub_date[..pub_date.len() - 6])
                );
                data.push_str(&article);
            }
        }
        self.data = format!(
            "{data}
    </channel>\n</rss>"
        );
        self.write()?;
        Ok(())
    }

    fn write(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.output)?;
        writeln!(file, "{}", self.data)?;
        Ok(())
    }
}

impl std::fmt::Debug for Feed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path in: {:#?}, out: {:#?}", self.path, self.output)
    }
}
