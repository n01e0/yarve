#[macro_use]
extern crate clap;

mod emulator;

fn main() {
    let _app = clap_app!(yarve =>
        (version:       crate_version!())
        (about:         crate_description!())
        (author:        crate_authors!())
    ).get_matches();
}
