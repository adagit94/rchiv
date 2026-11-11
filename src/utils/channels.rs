use std::sync::mpsc::{Receiver, TryRecvError};

macro_rules! select_rotate {
    ($func_name: ident, $(($val_type_param_name: ident, $recvr_param_name: ident, $handler_param_name: ident)),+) => {
      /// Function starts a select loop that calls try_recv() in ordered way based on the order of passed arguments.
      ///
      /// In case receive from the channel is successfull, related handler passed get's called with received value. In case unsuccessfull receive indicates that channel is empty, next receiver will be tried.
      ///
      /// It's possible to indicate potential intention to terminate the whole select loop via returned boolean from the handler (true for intended termination). Whole receive loop also terminates in case of unsuccessfull receive indicating disconnection of the sender related to any receiver - in this scenario, all passed receivers will be dropped, channels closed and whole communication flow should be terminated.
      /// ```
      ///
      /// let (s1, r1) = mpsc::channel::<String>();
      /// let (s2, r2) = mpsc::channel::<u8>();
      /// let (s3, r3) = mpsc::channel::<u8>();
      ///
      /// thread::spawn(move || {
      ///   // Order of arguments determines order of attempts to receive a value (try_recv()). In case channel isn't empty, handler get's called. Receivers are tried in predictable order without relation to their content being present - in case there isn't termination flag returned from a handler. After the last receiver, select loop iterates again in same order.
      ///   select_rotate_3((r1, |_v| {
      ///     // Handler triggered as first in case r1 isn't empty.
      ///     false // This handler never causes termination of the select loop.
      ///   }), (r2, |v| {
      ///     // Always tried as second. Handler triggered as second in case r1 wasn't empty, or as the first one in case r1 was empty.
      ///     v == 0 // Termination of whole select loop when v == 0.
      ///   }), (r3, |v| {
      ///     // Always tried as third. Handler triggered as third in case r1 and r2 weren't empty, or as the first or second one based on state of preceding receivers. After this receiver, select loop repeats - in case it wasn't terminated via true value beign returned from a handler.
      ///     v == 1 // Termination of whole select loop when v == 1.
      ///   }));
      /// });
      /// ```
      pub fn $func_name<$($val_type_param_name),+>(
            $(($recvr_param_name, $handler_param_name): (Receiver<$val_type_param_name>, impl Fn($val_type_param_name) -> bool)),+
        ) {
            loop {
                $(
                    match $recvr_param_name.try_recv() {
                        Ok(v) => {
                            if $handler_param_name(v) {
                                return;
                            }
                        },

                        Err(err) => match err {
                            TryRecvError::Disconnected => return,
                            TryRecvError::Empty => {},
                        },
                    };
              )+
            }
        }
    };
}

select_rotate!(select_rotate_2, (V1, r1, f1), (V2, r2, f2));
select_rotate!(select_rotate_3, (V1, r1, f1), (V2, r2, f2), (V3, r3, f3));
select_rotate!(
    select_rotate_4,
    (V1, r1, f1),
    (V2, r2, f2),
    (V3, r3, f3),
    (V4, r4, f4)
);
select_rotate!(
    select_rotate_5,
    (V1, r1, f1),
    (V2, r2, f2),
    (V3, r3, f3),
    (V4, r4, f4),
    (V5, r5, f5)
);

macro_rules! select_repeat_by_priority {
    ($func_name: ident, $(($val_type_param_name: ident, $recvr_param_name: ident, $handler_param_name: ident)),+) => {
      /// Function starts a select loop that calls try_recv() in ordered way by priority based on the order of passed arguments.
      ///
      /// In case receive from the channel is successfull, related handler passed get's called with received value and loop will start from the beginning again - be mindful that in case channel with higher priority will be continuously filled, channels with lower priority can potentially never yield a value. In case unsuccessfull receive indicates that channel is empty, next receiver by priority will be tried.
      ///
      /// It's possible to indicate potential intention to terminate the whole select loop via returned boolean from the handler (true for intended termination). Whole receive loop also terminates in case of unsuccessfull receive indicating disconnection of the sender related to any receiver - in this scenario, all passed receivers will be dropped, channels closed and whole communication flow should be terminated.
      /// ```
      ///
      /// let (s1, r1) = mpsc::channel::<String>();
      /// let (s2, r2) = mpsc::sync_channel::<u8>(10);
      /// let (s3, r3) = mpsc::sync_channel::<u8>(5);
      ///
      /// thread::spawn(move || {
      ///   // Order of arguments determines priority of attempts to receive a value (try_recv()). In case channel isn't empty, handler get's called and loop will start over again.
      ///   select_rotate_3((r1, |_v| {
      ///     // Handler triggered in case r1 isn't empty. In such case loop wil start over again.
      ///     false // This handler never causes termination of the select loop.
      ///   }), (r2, |v| {
      ///     // Tried as second in case r1 was empty. In case of successfull receive (and no intended temination) loop wil start over again.
      ///     v == 0 // Termination of whole select loop when v == 0.
      ///   }), (r3, |v| {
      ///     // Tried as third in case both r1 and r2 were empty.
      ///     v == 1 // Termination of whole select loop when v == 1.
      ///   }));
      /// });
      /// ```
      pub fn $func_name<$($val_type_param_name),+>(
            $(($recvr_param_name, $handler_param_name): (Receiver<$val_type_param_name>, impl Fn($val_type_param_name) -> bool)),+
        ) {
            loop {
                $(
                    match $recvr_param_name.try_recv() {
                        Ok(v) => {
                            if $handler_param_name(v) {
                                return;
                            }

                            continue;
                        },

                        Err(err) => match err {
                            TryRecvError::Disconnected => return,
                            TryRecvError::Empty => {},
                        },
                    };
              )+
            }
        }
    };
}

select_repeat_by_priority!(select_repeat_by_priority_2, (V1, r1, f1), (V2, r2, f2));
select_repeat_by_priority!(
    select_repeat_by_priority_3,
    (V1, r1, f1),
    (V2, r2, f2),
    (V3, r3, f3)
);
select_repeat_by_priority!(
    select_repeat_by_priority_4,
    (V1, r1, f1),
    (V2, r2, f2),
    (V3, r3, f3),
    (V4, r4, f4)
);
select_repeat_by_priority!(
    select_repeat_by_priority_5,
    (V1, r1, f1),
    (V2, r2, f2),
    (V3, r3, f3),
    (V4, r4, f4),
    (V5, r5, f5)
);

macro_rules! select_once_by_priority {
    ($func_name: ident, $(($val_type_param_name: ident, $recvr_param_name: ident, $handler_param_name: ident)),+) => {
      /// Function calls try_recv() in ordered way by priority based on the order of passed arguments and triggers first handler of successfull receive and returns.
      ///
      /// Unsuccessfull receive from any channel indicating sender disconnection will cause early return from a function without any handler being triggered.
      pub fn $func_name<$($val_type_param_name),+>(
            $(($recvr_param_name, $handler_param_name): (Receiver<$val_type_param_name>, impl Fn($val_type_param_name))),+
        ) {
            loop {
                $(
                    match $recvr_param_name.try_recv() {
                        Ok(v) => {
                            $handler_param_name(v);
                            return;
                        },
                        Err(err) => match err {
                            TryRecvError::Disconnected => return,
                            TryRecvError::Empty => {},
                        },
                    };
                )+
            }
        }
    };
}

select_once_by_priority!(select_once_by_priority_2, (V1, r1, f1), (V2, r2, f2));
select_once_by_priority!(
    select_once_by_priority_3,
    (V1, r1, f1),
    (V2, r2, f2),
    (V3, r3, f3)
);
select_once_by_priority!(
    select_once_by_priority_4,
    (V1, r1, f1),
    (V2, r2, f2),
    (V3, r3, f3),
    (V4, r4, f4)
);
select_once_by_priority!(
    select_once_by_priority_5,
    (V1, r1, f1),
    (V2, r2, f2),
    (V3, r3, f3),
    (V4, r4, f4),
    (V5, r5, f5)
);
