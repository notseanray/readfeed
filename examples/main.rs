use readfeed::Feed;

fn main() {
    let mut rss = Feed::new(
        &"Example title!",
        &"./examples",
        Some(&"output.xml"),
        &"pages",
        &"Example url!",
        Some(&"Example Description!"),
    );
    rss.generate().unwrap();
}
