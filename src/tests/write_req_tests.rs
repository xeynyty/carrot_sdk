#[cfg(test)]
mod byte_match {
    use crate::data::Data;
    use crate::utils::Work;
    use crate::write_req::{SetData, WriteReq};

    #[test]
    fn sync_bytes() {
        let original: WriteReq = WriteReq::default();
        let bytes: Vec<u8> = original.clone().try_into().unwrap();
        let restored: WriteReq = bytes.try_into().unwrap();
        assert_eq!(original, restored)
    }

    #[test]
    fn methods() {
        let default = WriteReq::default()
            .set_data(100i64)
            .set_data(100u64)
            .set_data("&str")
            .set_data(String::from("String"))
            .set_key(Some(123))
            .set_iat(10);

        assert_eq!(Work::Write, default.work());
        assert_eq!(Data::UTF8(String::from("String")), default.data());
        assert_eq!(Some(123), default.key());
        assert_eq!(10, default.iat());
    }
}