use std::fmt::{Display, Debug};

pub fn main() {
    pub trait Summary {
        // Simple method declaration
        fn count_chars(&self) -> usize;

        // With default implem
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // Implement trait name for struct
    impl Summary for NewsArticle {
        fn count_chars(&self) -> usize {
            self.content.len()
        }

        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn count_chars(&self) -> usize {
            self.content.len()
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    // trait as parameter, the simple way
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // trait as parameter, more complex but more powerful (Trait Bound Syntax)

    pub fn notify_with_same_type_args<T: Summary>(item1: &T, item2: &T) {
        println!("Breaking news! {} {}", item1.summarize(), item2.summarize(), );
    }

    pub fn notify_with_diff_types_args<T: Summary, I: Summary>(item1: &T, item2: &I) {
        println!("Breaking news! {} {}", item1.summarize(), item2.summarize(), );
    }

    // won't compile
    // notify_with_same_type_args(tweet, article)
    notify_with_same_type_args(&tweet, &tweet2);
    notify_with_diff_types_args(&tweet, &article);

    // Multiple traits are needed here
    pub fn notify_multiple_traits(item: impl Summary + Display) {}

    // Where clause to get better method signatures
    fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug {
        1
    }

    // return a trait, but we can return a single type only due to compiler limitations
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}