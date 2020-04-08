mod reddit;

use reddit::RedditClient;


pub fn run() {
    let listings = RedditClient::new("/r/programming").limit(30).get();

    for listing in listings {
        println!("{}\n\thttps://reddit.com{}\n",
                 listing.link.data.children[0].data.title,
                 listing.link.data.children[0].data.permalink
        );
    }
}
