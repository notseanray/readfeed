use readfeed::Feed;

fn main() {
    let mut rss = Feed::new(
        &"Example title!",
        &"./examples",
        Some(&"output.xml"),
        &"pages",
        &"https://exampleWebsite.com",
        Some(&"Example Description!"),
    );
    rss.generate().unwrap();
}
