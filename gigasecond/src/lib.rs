use time::PrimitiveDateTime as DateTime;
use time::Duration;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration(1000000000)
}
