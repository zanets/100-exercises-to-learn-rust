#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize

        /*
        Vec 空間不夠時，不是只多一格，而是把容量翻倍：


        容量 2 → 滿了 → 新容量 4
        容量 4 → 滿了 → 新容量 8
        容量 8 → 滿了 → 新容量 16

        */
        assert_eq!(v.capacity(), 4);
    }
}
