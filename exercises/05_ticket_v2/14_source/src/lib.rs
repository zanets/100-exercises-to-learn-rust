/*
crate:: 就是指自己這個 crate 的根（lib.rs 或 main.rs）。


crate        ← lib.rs
└── status   ← status.rs (mod status)
use crate::status::Status 就是從根開始往下找到 status 模組裡的 Status。
*/

use crate::status::{ParseStatusError, Status};

// We've seen how to declare modules in one of the earliest exercises, but
// we haven't seen how to extract them into separate files.
// Let's fix that now!
//
// In the simplest case, when the extracted module is a single file, it is enough to
// create a new file with the same name as the module and move the module content there.
// The module file should be placed in the same directory as the file that declares the module.
// In this case, `src/lib.rs`, thus `status.rs` should be placed in the `src` directory.
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
//   When calling `source` on an error of that variant, it should return a `ParseStatusError` rather than `None`.

/*
source() 是 Error trait 的一個方法，用來表示「這個 error 是由哪個底層 error 造成的」。

例如：

TicketNewError::StatusInvalid   ← 上層 error（你回傳給呼叫者的）
    └── ParseStatusError        ← 底層 error（實際發生的原因）
呼叫 err.source() 就能拿到 ParseStatusError，讓你追蹤錯誤的根源。

這題測試就是在確認：StatusInvalid 這個 error 有沒有把 ParseStatusError 存起來當 source。
*/

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("`invalid` is not a valid status. Use one of: ToDo, InProgress, Done")]
    StatusInvalid(#[source] ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: Parse the status string into a `Status` enum.
        match Status::try_from(status) {
            Err(error) => return Err(TicketNewError::StatusInvalid(error)),
            Ok(status) => {
                return Ok(Ticket {
                    title,
                    description,
                    status,
                })
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
