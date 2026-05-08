use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
//   You want to *delegate* the iteration to the `Vec<Ticket>` field in `TicketStore`.
//   Look at the standard library documentation for `Vec` to find the right type
//   to return from `into_iter`.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

/*
IntoIterator 是一個 trait，你只需要實作它規定的方法，告訴 Rust「怎麼產生一個 iterator」。

Rust 的 for loop 其實是語法糖，自動幫你呼叫 into_iter()：

// 你寫的
for ticket in store { ... }

// Rust 展開成
let mut iter = store.into_iter();
while let Some(ticket) = iter.next() { ... }
*/

impl IntoIterator for TicketStore {
    // type Item = 每次迭代產出的元素型別
    type Item = Ticket;

    // type IntoIter = 實際的 iterator 型別，直接用 Vec 的
    type IntoIter = std::vec::IntoIter<Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
