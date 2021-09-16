pub mod dir;

#[cfg(test)]
mod tests {
    use super::dir;
    #[test]
    fn it_works() {
        let dir = String::from("경로 입력");
        dir::file_of_dir(dir);
    }
}