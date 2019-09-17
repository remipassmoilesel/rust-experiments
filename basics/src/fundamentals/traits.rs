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

}