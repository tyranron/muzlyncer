#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::web::{
    document,
};
use stdweb::web::event::ClickEvent;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
    stdweb::initialize();

    let h1 = document().query_selector("h1").unwrap().unwrap();
    h1.add_event_listener( enclose!( (h1) move |_ :ClickEvent| {
        h1.set_text_content("Hello, 世界!");
    }) );

    stdweb::event_loop();
}
