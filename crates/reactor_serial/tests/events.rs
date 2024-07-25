mod shared;
use std::{thread::sleep, time::Duration};

use bevy::prelude::*;
use reactor_serial::prelude::*;
pub use shared::*;

#[test]
fn file_reading_and_writing() {
    delete_folder("./saves/");
    sleep(Duration::from_secs(1)); // Sleep to give the OS time to catch up

    save_event();
    save_event_with_subdirectory();
    save_event_global();
    save_event_user();
    save_event_user_without_id();

    load_event();
    load_event_with_old_version();
    load_event_with_subdirectory();
}

fn save_event() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("save_event"))
        .register_save_data::<TestSaveFormatV3>()
        .add_systems(
            Update,
            |events: EventReader<SaveComplete<TestSaveFormatV3>>| {
                if events.is_empty() {
                    return;
                }
                assert!(events.len() == 1, "Did not receive SaveComplete event");
            },
        );

    let world = app.world_mut();
    let id = world.send_event(SaveRequest::new(
        "test_save",
        TestSaveFormatV3 {
            name: "test_name".to_string(),
            value: 0.0,
        },
    ));
    assert!(id.is_some(), "Failed to send event");

    let event_queue = world.get_resource::<Events<SaveRequest<TestSaveFormatV3>>>();
    assert!(event_queue.is_some(), "Failed to get event queue");
    let event_queue = event_queue.unwrap();

    let mut event_reader = event_queue.get_reader();
    assert_eq!(
        event_reader.read(event_queue).count(),
        1,
        "Failed to read event"
    );

    for _ in 0..5 {
        world.run_schedule(Main);
    }

    assert!(
        check_file_exists("./saves/save_event/default_slot/test_save"),
        "Failed to save file"
    );
}

fn save_event_global() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("save_event_global"))
        .register_save_data::<TestSaveFormatV3>();

    let world = app.world_mut();
    world.send_event(
        SaveRequest::new(
            "test_save",
            TestSaveFormatV3 {
                name: "test_name".to_string(),
                value: 0.0,
            },
        )
        .with_context(Context::Global),
    );

    for _ in 0..5 {
        world.run_schedule(Main);
    }

    assert!(
        check_file_exists("./saves/save_event_global/test_save"),
        "Failed to save global file"
    );
}

fn save_event_user() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("save_event_user"))
        .register_save_data::<TestSaveFormatV3>();

    let world = app.world_mut();
    world.set_user_id("test_id");
    world.send_event(
        SaveRequest::new(
            "test_save",
            TestSaveFormatV3 {
                name: "test_name".to_string(),
                value: 0.0,
            },
        )
        .with_context(Context::User),
    );

    for _ in 0..5 {
        world.run_schedule(Main);
    }

    assert!(
        check_file_exists("./saves/save_event_user/test_id/test_save"),
        "Failed to save user file"
    );
}

fn save_event_user_without_id() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("save_event_no_id"))
        .register_save_data::<TestSaveFormatV3>();

    let world = app.world_mut();
    world.send_event(
        SaveRequest::new(
            "test_save",
            TestSaveFormatV3 {
                name: "test_name".to_string(),
                value: 0.0,
            },
        )
        .with_context(Context::User),
    );

    for _ in 0..5 {
        world.run_schedule(Main);
    }

    assert!(
        !check_file_exists("./saves/save_event_no_id/test_id/test_save"),
        "Saved file per user when no user id set"
    );
    assert!(
        !check_file_exists("./saves/save_event_no_id/default_slot/test_save"),
        "Saved file with default slot when no user id set even though context is user"
    );
    assert!(
        check_file_exists("./saves/save_event_no_id/test_save"),
        "Failed to save user file globally with no user id set"
    );
}

fn save_event_with_subdirectory() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("sub_dir"))
        .register_save_data::<TestSaveFormatV3>();

    let world = app.world_mut();
    world.send_event(
        SaveRequest::new(
            "test_save",
            TestSaveFormatV3 {
                name: "test_name".to_string(),
                value: 0.0,
            },
        )
        .with_sub_directory("test"),
    );

    for _ in 0..5 {
        world.run_schedule(Main);
    }

    assert!(
        check_file_exists("./saves/sub_dir/default_slot/test/test_save"),
        "Failed to save file in subdirectory"
    );
}

fn load_event() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("load_event"))
        .register_save_data::<TestSaveFormatV3>()
        .add_systems(
            Update,
            |events: EventReader<LoadComplete<TestSaveFormatV3>>| {
                if events.is_empty() {
                    return;
                }
                assert!(events.len() == 1, "Did not receive LoadComplete event");
            },
        );

    let world = app.world_mut();
    world.send_event(SaveRequest::new(
        "test_save",
        TestSaveFormatV3 {
            name: "test_name".to_string(),
            value: 0.0,
        },
    ));
    world.run_schedule(Main);

    world.send_event(LoadRequest::<TestSaveFormatV3>::new("test_save"));
    for _ in 0..5 {
        world.run_schedule(Main);
    }
    assert_eq!(world.entities().len(), 1, "Failed to load save");
}

fn load_event_with_old_version() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("load_event"))
        .register_save_data::<TestSaveFormatV3>();

    let world = app.world_mut();
    world.send_event(SaveRequest::new(
        "test_save",
        TestSaveFormatV1 {
            name: "test_name".to_string(),
        },
    ));
    world.run_schedule(Main);

    world.send_event(LoadRequest::<TestSaveFormatV3>::new("test_save"));
    while world.entities().is_empty() {
        world.run_schedule(Main);
    }
    assert_eq!(
        world.entities().len(),
        1,
        "Failed to load save with old version"
    );
}

fn load_event_with_subdirectory() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(ReactorSerialPlugin::new("load_event_sub_dir"))
        .register_save_data::<TestSaveFormatV3>();

    let world = app.world_mut();
    world.send_event(
        SaveRequest::new(
            "test_save",
            TestSaveFormatV3 {
                name: "test_name".to_string(),
                value: 0.0,
            },
        )
        .with_sub_directory("test"),
    );
    world.run_schedule(Main);

    world.send_event(LoadRequest::<TestSaveFormatV3>::new("test_save").with_sub_directory("test"));

    while world.entities().is_empty() {
        world.run_schedule(Main);
    }
    assert_eq!(
        world.entities().len(),
        1,
        "Failed to load save from subdirectory"
    );
}
