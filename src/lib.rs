pub mod api;

pub use api::{*};

#[cfg(test)]
mod tests {
    use crate::api;

    #[test]
    fn it_works() {
        let sentence = "hello world!";
        let result = api::google::translate(sentence).unwrap();
        println!("google: {}", result);
        let result = api::baidu::translate(sentence).unwrap();
        println!("baidu: {}", result);
        let result = api::mymoney::translate(sentence).unwrap();
        println!("mymoney(not stable): {}", result);
        let result = api::deepl::translate(sentence).unwrap();
        println!("deepl: {}", result);
        let result = api::chatgpt::translate(sentence).unwrap();
        println!("chatgpt: {}", result);
    }
}