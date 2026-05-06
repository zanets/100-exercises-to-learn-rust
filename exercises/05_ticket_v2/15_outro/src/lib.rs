// TODO: you have something to do in each of the modules in this crate!
/*
mod 宣告通常寫在 lib.rs（或 main.rs），作為整個 crate 的模組樹入口：


lib.rs
├── mod status;      → src/status.rs
├── mod ticket;      → src/ticket.rs
└── mod utils;       → src/utils.rs
子模組也可以再宣告自己的子模組，但根都從 lib.rs 開始。

lib.rs 就是這個 crate 的公開介面，負責：

宣告有哪些模組（mod status;）
決定哪些東西對外可見（pub mod、pub use）
外部使用這個 crate 的人，只能看到 lib.rs 暴露出來的東西。


// lib.rs
mod internal;        // 載入但不公開，外部看不到
pub mod status;      // 公開，外部可以用
pub use status::Status;  // 把 Status 直接提到根層級，外部用起來更方便
*/

mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// and then re-export the public parts of those modules at the root of the crate.
//
// This hides the internal structure of the crate from your users, while still
// allowing you to organize your code however you like.
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// We no longer need to make the fields private!
// Since each field encapsulates its own validation logic, there is no risk of
// a user of `Ticket` modifying the fields in a way that would break the
// invariants of the struct.
//
// Careful though: if you had any invariants that spanned multiple fields, you
// would need to ensure that those invariants are still maintained and go back
// to making the fields private.
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
