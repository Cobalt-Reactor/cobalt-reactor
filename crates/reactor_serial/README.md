# `reactor_serial`

A crate for handling saving and loading for the Bevy game engine, designed for use with `reactor_proto`

## Usage

* Add `SerialPlugin` to your app with a root save path (relative to the users data dir).
* Register any types you want to use as save data with `app.register_save_data`.
* If your app is multi user set the user using `commands.set_user_id()`.
* Setup a save slot using `commands.set_save_slot()`
* If you are using `reactor_proto` don't register `ProtoPlugin`  as `reactor_serial` will register it for you.

To request a save send a `SaveRequest` event.
To request a load send a `LoadRequest` event.

## Testing Note

Be aware that file IO testing has a lot of sleeps in it, this is to let the OS actually do the file IO. Don't be worried if test the events.rs file takes some time. I'd recommend not running `cargo test`, and instead running `cargo test --test <filename>` if you're doing testing in general.

Sorry about that, I couldn't find another good way to ensure that files were deleted/written without sleeping or looping in a way that would cause the test to infinite loop on a failure.
