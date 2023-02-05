/* Traits allow to define a Set of Methods that are shared between difference Types */
pub trait Summary {
  fn summarize(&self) -> String;
  /* Default Implementation will be overridden by a new Implementation if there is one */
  fn summarize_with_default(&self) -> String {
    String::from("Read more..")
  }
}

pub struct NewsArticle {
  pub author: String,
  pub headline: String,
  pub content: String
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {}", self.headline, self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

fn main() {
  let tweet: Tweet = Tweet {
    username: String::from("Michael"),
    content: String::from("Bruno"),
    reply: false,
    retweet: false
  };
  println!("Tweet Summary: {}", tweet.summarize());

  let article: NewsArticle = NewsArticle {
    author: String::from("Michael"),
    headline: String::from("Bruno"),
    content: String::from("Bud")
  };
  println!("Article Summary: {}", article.summarize());
  println!("Article Summary: {}", article.summarize_with_default());

  notify(&article);
  notify_with_trait_bound(&tweet);

  println!("Summarizeable Article: {}", return_summarizeable().summarize());
}

fn notify(item: &impl Summary) {
  println!("Breaking News: {}", item.summarize())
}

/* Alternative with Trait Bound */
fn notify_with_trait_bound<T: Summary>(item: &T) {
  println!("Breaking News: {}", item.summarize())
}

fn return_summarizeable() -> impl Summary {
  NewsArticle {
    author: String::from("Marie"),
    headline: String::from("Bruno"),
    content: String::from("Bud")
  }
}