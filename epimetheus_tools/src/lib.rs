pub mod util;
#[cfg(test)]
mod tests {
    use crate::util::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn do_info(){
        sunny_log::info("测试".to_string());
    }
}
