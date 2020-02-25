use std::io::Read;

use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| help());
    let buffer = &mut Vec::new();
    std::fs::File::open(&input)
        .unwrap()
        .read_to_end(buffer)
        .unwrap();

    let _document = parse_html(&buffer);
}

// We callgrind only this function by using --toggle-collect=parse_html
#[no_mangle]
#[inline(never)]
pub fn parse_html(input: &[u8]) -> markup5ever_rcdom::RcDom {
    html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .one(input)
}

fn help() -> ! {
    eprintln!("Usage: {} <input>", std::env::args().nth(0).unwrap());
    std::process::exit(1);
}
