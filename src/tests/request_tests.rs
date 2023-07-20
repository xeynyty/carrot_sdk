#[cfg(test)]
mod byte_match {
    use crate::data::Data;
    use crate::utils::Work;
    use crate::request::{SetData, Request};

    #[test]
    fn sync_bytes() {
        let original: Request = Request::default();
        let bytes: Vec<u8> = original.clone().try_into().unwrap();
        let restored: Request = bytes.try_into().unwrap();
        assert_eq!(original, restored)
    }

    #[test]
    fn methods() {
        let default = Request::default()
            .set_data(100i64)
            .set_data(100u64)
            .set_data("&str")
            .set_data(String::from("String"))
            .set_key(Some(123))
            .set_iat(10)
            .write();

        assert_eq!(Work::Write, default.work());
        assert_eq!(Data::UTF8(String::from("String")), default.data());
        assert_eq!(Some(123), default.key());
        assert_eq!(10, default.iat());
    }
}