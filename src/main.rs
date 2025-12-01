

mod content;
use content::media::Media;
use content::catalog::Catalog;


fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Media book"),
    };

    let movie = Media::Movie {
        title:String::from("Jurassic Park"),
        director:String::from("Stephen Spielberg"),
    };

    let book = Media::Book {
        title:String::from("Little Women"),
        author:String::from("John Smith"),
    };

    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    //println!("{}", audiobook.description());
    //println!("{}", movie.description());
    //println!("{}", book.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);
    //print_media(audiobook);
    //print_media(movie);
    //print_media(book);
    //println!("{:#?}", catalog.items.get(100));
    match catalog.items.get(100) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("No Item");
        }
    }

    let item = catalog.get_by_index(0);
    let placeholder = Media::Placeholder;
    println!("{:#?}",item.unwrap_or(&placeholder));
    /*println!("Item: {:#?}", item);
    match catalog.get_by_index(0) {
        Some(value) => {
            println!("ThereIsaValue: {:#?}", value);
        }
        None => {
            println!("NoValueAvailable");
        }
    }

    if let Some(value) = catalog.get_by_index(1) {
        println!("ThereIsaValue: {:#?}", value);
    } else {
        println!("NoValueAvailable");
    }*/

}
