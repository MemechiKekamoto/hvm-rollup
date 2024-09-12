#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Storage;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        Storage {}
        impl Storage {
            pub fn new() -> Self;
            pub fn store_program(&self, data: Vec<u8>) -> Result<(), String>;
        }
    }

    #[test]
    fn test_upload_bend_program() {
        let mut mock_storage = MockStorage::new();
        mock_storage.expect_store_program()
                    .times(1)
                    .returning(|_| Ok(()));
        let result = upload_bend_program("valid_program.wasm");
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_batch() {
        let result = process_batch();
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_and_submit() {
        let result = verify_and_submit();
        assert!(result.is_ok());
    }
}