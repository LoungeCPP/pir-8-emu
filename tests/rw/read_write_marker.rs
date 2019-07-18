use pir_8_emu::ReadWriteMarker;


#[test]
fn new() {
    let marker = ReadWriteMarker::new();
    assert_eq!(marker.was_read(), false);
    assert_eq!(marker.was_written(), false);
}

#[test]
fn read() {
    let marker = ReadWriteMarker::new();

    marker.read();
    assert_eq!(marker.was_read(), true);
    assert_eq!(marker.was_written(), false);


    let mut marker = ReadWriteMarker::new();
    marker.written();

    marker.read();
    assert_eq!(marker.was_read(), true);
    assert_eq!(marker.was_written(), true);
}

#[test]
fn written() {
    let mut marker = ReadWriteMarker::new();

    marker.written();
    assert_eq!(marker.was_read(), false);
    assert_eq!(marker.was_written(), true);


    let mut marker = ReadWriteMarker::new();
    marker.read();

    marker.written();
    assert_eq!(marker.was_read(), true);
    assert_eq!(marker.was_written(), true);
}

#[test]
fn reset() {
    let mut marker = ReadWriteMarker::new();

    marker.reset();
    assert_eq!(marker.was_read(), false);
    assert_eq!(marker.was_written(), false);

    marker.read();
    marker.reset();
    assert_eq!(marker.was_read(), false);
    assert_eq!(marker.was_written(), false);

    marker.written();
    marker.reset();
    assert_eq!(marker.was_read(), false);
    assert_eq!(marker.was_written(), false);

    marker.read();
    marker.written();
    marker.reset();
    assert_eq!(marker.was_read(), false);
    assert_eq!(marker.was_written(), false);
}
