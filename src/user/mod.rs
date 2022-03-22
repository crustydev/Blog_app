mod writer;
mod user;
mod reader;
mod traits;

enum User {
    Writer(Writer),
    Reader(Reader)
}