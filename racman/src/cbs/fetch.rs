use alpm::FetchCbReturn;

pub fn fetchcb(_url: &str, _path: &str, _force: bool) -> FetchCbReturn {
    FetchCbReturn::Ok
}