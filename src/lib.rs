mod reddit;

use reddit::RedditClient;


pub fn run() {
    let listings = RedditClient::new("/r/all").limit(3).get();

    for listing in listings {
        println!("\n\n'{}' has {} comments.\n\t\thttps://reddit.com{}\n\t ",
                 listing.link.data.children[0].data.title,
                 listing.comments.data.children.len(),
                 listing.link.data.children[0].data.permalink,
        );
    }
}
