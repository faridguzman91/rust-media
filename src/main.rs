#[derive(Debug)]

enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

//rust will not let you create different selfs without their types
// description's self will be of type Media
impl Media {
    fn description(&self) -> String {
        // //if self is of enum type Book, give access to title and author.
        // if let Media::Book { title, author} = self {
        //     format!("Book: {} {}", title, author)
        //     // if self is of enum type Movie, give access to title and director
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        //     // if self is of enum type Audiobook, give access to title
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        //the same can be achieved with less code using match:
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}


impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    //add a mutable referenc to self
    //give ownership to the media
    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

// fn print_book(book: Book) {}
// fn print_movie(movie: Movie) {}
//
// do instead:
fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("an audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad book"),
        author: String::from("bad author"),
    };

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    // print_media(good_movie);
    // print_media(bad_book);
    // print_media(audiobook);
    //
    //
    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);

    println!("{:#?}", catalog);
}
