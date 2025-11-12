use std::fmt::Debug;

pub fn assert_noerror<T, E: Debug>(res: Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => panic!("encountered unexpected error: {:?}", e),
    }
}

pub fn assert_error<T, E: Debug + PartialEq>(res: Result<T, E>, err: E) -> () {
    assert_eq!(Some(err), res.err())
}
