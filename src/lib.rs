use std::ffi::OsStr;
use std::os::unix::prelude::OsStrExt;
use std::{path::PathBuf, str::FromStr, ops::Deref, fmt, fs};

struct Feed {
    title: String,
    path: PathBuf,
    output: PathBuf,
    url_base: String,
    page_folder: String,
    data: String,
    description: Option<String>
}

// THIS WILL MAKE YOUR XML PARSER CRY

impl Feed {
    pub fn new<'a, T: Deref<Target=&'a str>>(title: &str, path: T, output: Option<T>, page_folder: &str, url_base: &str, description: Option<&str>) -> Self {
        let in_path = PathBuf::from_str(&path).expect("invalid path");
        let out_file = match output {
            Some(v) => PathBuf::from_str(&v).expect("invalid path"),
            None => PathBuf::from(&in_path.join("feed.xml"))
        };
        let description = match description {
            Some(v) => Some(v.to_string()),
            None => None
        };
        Feed {
            title: title.to_string(),
            path: in_path,
            output: out_file,
            url_base: url_base.to_string(),
            page_folder: page_folder.to_string(),
            data: String::new(),
            description
        }
    }

    pub fn update(&self) {}

    fn generate(&mut self) {
        
        let mut data = format!(r#"
<rss version="2.0">
    <channel>
        <title>{}</title>
        <link>{}</link>"#, self.title, self.url_base);
        match &self.description {
            Some(v) => data.push_str("
        <description>{v}</description>"),
            None => {}
        };
        data.push_str("
        <language>en-us</language>");
        for file in self.path.read_dir().unwrap() {
            if let Ok(v) = file {
                if v.path().as_path().extension().unwrap() != OsStr::from_bytes(b"html") {
                    continue;
                }
                if let Ok(x) = fs::read_to_string(v.path()) {
                    let article = format!("
            <item>
                <title>{:#?}</title>
                <guid>{}</guid>
                <pubDate>{}</pubDate>", v.file_name(), 
                format!("{}/{}/{:#?}.html", 
                        self.url_base, 
                        self.page_folder, 
                        v.file_name()),
                        "date"); // TODO DATE   
                }
            }
        }
        self.data = data.to_owned() + "\t</channel>\n</rss>";
    }

    fn write(&self) {}
}

impl std::fmt::Debug for Feed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path in: {:#?}, out: {:#?}", self.path, self.output)
    }
}
