#![cfg(test)]

use crate::application;
use trillium_testing::prelude::*;

#[test]
fn says_hello_world() {
    assert_ok!(get("/trillium-template/").on(&application()), "hello world!")
}

#[test]
fn says_hello_name() {
    assert_ok!(
        get("/trillium-template/hello/trillium").on(&application()),
        "hello, trillium!"
    );
    assert_ok!(get("/trillium-template/hello/rust").on(&application()), "hello, rust!");
}

#[test]
fn other_routes_are_not_found() {
    assert_response!(
        get("/not/found").on(&application()),
        404,
        "Uh oh, I don't have a route for /not/found"
    )
}
