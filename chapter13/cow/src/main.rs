use std::{borrow::Cow, path::PathBuf};

enum Error {
    OutOfMemory,
    StackOverflow,
    FileNotFound(PathBuf),
}

// Stringを返すようにしても、問題ない。ただし、Stringのコピーが発生するので、それを避けたい場合には、Cowを使う。
fn describe(error: &Error) -> Cow<'static, str> {
    match *error {
        Error::OutOfMemory => "out of memory".into(),
        Error::StackOverflow => "stack overflow".into(),
        Error::FileNotFound(ref path) => format!("file not found: {}", path.display()).into(),
    }
}

fn generate_random_pathbuf() -> PathBuf {
    let mut path = PathBuf::new();
    path.push("/tmp");
    path.push(format!("{}", rand::random::<u64>()));
    path
}

fn main() {
    println!("Hello, world!");
    let error = Error::OutOfMemory;
    println!("Disaster has stack: {}", describe(&error)); // Cowを&strとして使う。

    let mut log: Vec<String> = Vec::new();

    let err1 = Error::StackOverflow;
    log.push(describe(&err1).into());
    let err2 = Error::FileNotFound(generate_random_pathbuf());
    log.push(describe(&err2).into_owned()); // 値を所有する
    let err3 = Error::FileNotFound(generate_random_pathbuf());
    log.push(describe(&err3).into());
    println!("Log: {:?}", log);
}
