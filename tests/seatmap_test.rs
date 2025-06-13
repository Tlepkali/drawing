use drawing::seat_map::SeatMap;
use drawing::geometrical_shapes::Point;

#[test]
fn seatmap_dimensions() {
    let sm = SeatMap::new(3, 4, &Point::new(0,0), 10, 2);
    assert_eq!(sm.rows, 3);
    assert_eq!(sm.cols, 4);
    assert_eq!(sm.seats.len(), 12);
}
