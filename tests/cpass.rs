//! Collections of `Send`-able things are `Send`

use heapless::Vec;

#[test]
fn send() {
    struct IsSend;

    unsafe impl Send for IsSend {}

    fn is_send<T>()
    where
        T: Send,
    {
    }

    // is_send::<Consumer<IsSend, consts::U4>>();
    // is_send::<Producer<IsSend, consts::U4>>();
    // is_send::<Queue<IsSend, consts::U4>>();
    is_send::<Vec<IsSend, 4>>();
    // is_send::<HistoryBuffer<IsSend, consts::U4>>();
}
