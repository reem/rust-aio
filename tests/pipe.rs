extern crate aio;
extern crate syncbox;

use aio::pipe;
use aio::IoReadStream;

use syncbox::util::async::{Async, join};

#[test]
fn test_pipe() {
    let (rd, wr) = pipe::pipe().unwrap();

    let mut vec = vec![];

    join(("hello".to_string().pipe(wr), rd.pipe(&mut vec)))
        .await().unwrap();

    assert_eq!(&vec[], "hello".as_bytes())
}

