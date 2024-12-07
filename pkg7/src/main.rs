fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let largest = max(&list); let large = max2(&list);
    assert_eq!(largest, large);
    println!("Max: {}", largest);

    let list = vec!['_', 'y', 'm', 'c', 'a'];
    let largest = max(&list);
    println!("Max: {}", largest);

    let mut point = Point {x:5, y:10.333f32};
    println!("{:?}", point);

    point.set(22, 10.1);
    println!("{:?}", point.get());

    let tweet = Tweet {
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

    println!("1 new tweet: {}", tweet.summarize()); 
    print!("notify(tweet) =>"); notify(&tweet);
    println!("New article available! {}", article.summarize());
    print!("notify(article) =>"); notify(&article);

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let s: &'static str = "I have a static lifetime."; 
    // static lifetime is the entire duration of the program
    // All string literals have 'static lifetime
    // The text of this string is stored directly in the program’s binary, which is always available.
    println!("{}", s);
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String { // default implementation
        return String::from("(Read more...)")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Above is syntactic sugar for below
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content)
    }
}

#[derive(Debug)]
struct Point<X: Copy, Y: Copy>{
    x: X,
    y: Y
}

impl<X: Copy, Y: Copy> Point<X, Y> {
    fn set(&mut self, x: X, y: Y) {
        self.x = x;
        self.y = y;
    }
    fn get(&self) -> (X, Y) {
        return  (self.x, self.y)
    }
}

fn max<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut max_ = list[0];
    for item in list {
        if *item > max_ {
            max_ = *item;
        }
    }
    return max_;
}

fn max2<T>(list:&Vec<T>) -> T
where T: PartialOrd + Copy {
    let mut max_ = list[0];
    for item in list {
        if *item > max_ {
            max_ = *item;
        }
    }
    return max_;
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    } else {
        return y
    }
}