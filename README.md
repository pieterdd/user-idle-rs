# Get the idle time of a user

| OS              | Supported |
| --------------- | --------- |
| Linux           | ✔️*       |
| Windows         | ✔️        |
| MacOS           | ✔️        |

\* The Linux implementation will do the following:
1. Try to get the idle time from Mutter via DBus. This should work on GNOME desktops with Wayland or X11.
2. Try to get the idle time from X11. This will not work on Wayland.
3. As a last resort, try to get the screensaver's idle time via DBus. Note that the screensaver may report a value of 0ns when it's not active.

### Example

```rust
use user_idle::UserIdle;

let idle = UserIdle::get_time().unwrap();

let idle_seconds = idle.as_seconds();
let idle_minutes = idle.as_minutes();
```

Check the documentation for more methods.
