// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

/*
Drop 是 Rust 的解構子（destructor）。

當一個值的生命週期結束（離開 scope、被 move 走、手動 drop()）時，Rust 會自動呼叫 Drop::drop()。


{
    let bomb = DropBomb::new();
} // ← 這裡 scope 結束，自動呼叫 drop()
常見用途是清理資源，例如：

File 被 drop 時自動關閉檔案
MutexGuard 被 drop 時自動釋放鎖
Vec 被 drop 時釋放 heap 記憶體
這就是 Rust 不需要 GC 也能自動管理資源的機制，叫做 RAII（Resource Acquisition Is Initialization）。

*/

pub struct DropBomb {
    fuse: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { fuse: true }
    }

    pub fn defuse(&mut self) {
        self.fuse = false;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.fuse {
            panic!("drop")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb: DropBomb = DropBomb::new();
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
    }
}
