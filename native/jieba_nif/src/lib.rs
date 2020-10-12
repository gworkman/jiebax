use jieba_rs::Jieba;
use jieba_rs::TokenizeMode;
use lazy_static::lazy_static;

lazy_static! {
    static ref JIEBA: Jieba = Jieba::new();
}

#[rustler::nif]
fn cut(sentence: &str) -> Vec<&str> {
    let result = JIEBA.cut(sentence, false);
    result
}

#[rustler::nif]
fn cut_all(sentence: &str) -> Vec<&str> {
    let result = JIEBA.cut_all(sentence);
    result
}

#[rustler::nif]
fn cut_for_search(sentence: &str) -> Vec<&str> {
    let result = JIEBA.cut_for_search(sentence, false);
    result
}

#[rustler::nif]
fn tokenize(sentence: &str) -> Vec<(&str, usize, usize)> {
    let vec = JIEBA.tokenize(sentence, TokenizeMode::Default, false);
    let result = vec
        .iter()
        .map(|token| (token.word, token.start, token.end))
        .collect::<Vec<(&str, usize, usize)>>();
    result
}

rustler::init!("Elixir.Jiebax", [cut, cut_all, cut_for_search, tokenize]);
