# Crossbeam RX Channel Test

This is a small test to check if a Crossbeam channel can be duplicated to receive a message in two different threads simultaneously. 

### Outcome:
It turns out that it's not possible, as the first thread to reach the `try_recv()` will get the message, leaving the other thread with nothing. This behavior was expected.

### Alternative Test:
Alternatively tested the async crate `async-broadcast`, without utilizing async methods. Surprisingly, it works as long as methods returning futures are avoided. For example, `recv()` should be replaced with `try_recv()`, and similarly, `broadcast()` should be replaced with `try_broadcast()`.

### Motivation for Upload:
This test is being uploaded as there wasn't much information available online regarding this behavior. Hopefully, it will be helpful to someone encountering similar challenges.
