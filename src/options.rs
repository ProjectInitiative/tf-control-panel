extern crate clap;
use self::clap::{App, Arg};

// import constants from root import
use crate::constants::*;

pub fn get_matches() -> clap::ArgMatches<'static> {
    return App::new(APP_NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT_DESC)
        .arg(
            Arg::with_name(ARG_VERBOSE_NAME)
                .short(ARG_VERBOSE_SHORT)
                // .long(ARG_VERBOSE_LONG)
                .multiple(true)
                .help(ARG_VERBOSE_HELP),
        )
        .arg(
            Arg::with_name(ARG_COLOR_NAME)
                .short(ARG_COLOR_SHORT)
                .long(ARG_COLOR_LONG)
                .help(ARG_COLOR_HELP),
        )
        .arg(
            Arg::with_name("hid")
                .short("j")
                .long("hid")
                .help("print out HID USB info"),
        )
        .get_matches();
}
