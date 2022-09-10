extern crate buscaminas;

mod reader_test {
    use buscaminas::reader::read_file;

    #[test]
    pub(crate) fn open_non_existing_file() {
        let result = read_file("no_existe.txt".to_string());
        assert!(result.is_err());
    }

    #[test]
    pub(crate) fn open_existing_file() {
        let result = read_file("files/field.txt".to_string());
        assert!(result.is_ok());
    }

    #[test]
    pub(crate) fn open_invalid_file() {
        let result = read_file("files/invalid_field.txt".to_string());
        assert!(result.is_err());
    }

    #[test]
    pub(crate) fn open_empty_file() {
        let result = read_file("files/empty_field.txt".to_string());
        assert!(result.is_ok());
    }
}
