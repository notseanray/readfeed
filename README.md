### readfeed
<a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>

Extremely simple RSS generator for my website/blog. To determine the publication date the latest modified timestamp is used from the file metadata then converted to rfc2822 date format.

```toml
[dependencies]
readfeed = { git = "https://github.com/notseanray/readfeed" } 

```

##### Example usage:
```rust
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
```
###### output.xml
```
<rss version="2.0">
    <channel>
        <title>Example title!</title>
        <link>https://exampleWebsite.com</link>
        <description>Example Description!</description>
        <language>en-us</language>
        <item>
            <title>hello</title>
            <guid>https://exampleWebsite.com/pages/hello.html</guid>
            <pubDate>Thu, 24 Mar 2022 22:53:51 GMT</pubDate>
        </item>
        <item>
            <title>article</title>
            <guid>https://exampleWebsite.com/pages/article.html</guid>
            <pubDate>Thu, 24 Mar 2022 22:53:51 GMT</pubDate>
        </item>
        <item>
            <title>text</title>
            <guid>https://exampleWebsite.com/pages/text.html</guid>
            <pubDate>Thu, 24 Mar 2022 22:53:51 GMT</pubDate>
        </item>
    </channel>
</rss>
```

##### contributing
Any suggestions, improvements, or pull request are welcome. I am relatively new to rust so my quality of code is not great but I'm looking to improve!
