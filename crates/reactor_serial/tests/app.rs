mod shared;
use bevy::prelude::*;
use reactor_serial::prelude::*;
use shared::*;
use std::path::PathBuf;

#[test]
fn root_save_path() {
    let mut app = App::new();
    app.add_plugins(ReactorSerialPlugin::new("test_game"));
    let world = app.world_mut();

    let root_save_path = world.get_resource::<RootSavePath>().unwrap();
    let mut test_path = PathBuf::from("./saves");
    test_path.push("test_game");
    assert_eq!(**root_save_path, test_path.display().to_string());
}

#[test]
fn register_save_data() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("test_game"))
        .register_save_data::<TestSaveFormatV3>();

    let world = app.world_mut();
    let id = world.send_event(SaveRequest::new(
        "test_save",
        TestSaveFormatV3 {
            name: "test_name".to_string(),
            value: 0.0,
        },
    ));
    assert!(id.is_some());

    let event_queue = world.get_resource::<Events<SaveRequest<TestSaveFormatV3>>>();
    assert!(event_queue.is_some());
    let event_queue = event_queue.unwrap();

    let mut event_reader = event_queue.get_reader();
    assert_eq!(event_reader.read(event_queue).count(), 1);
}
