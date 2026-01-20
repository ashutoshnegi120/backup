use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let time_break = time::Duration::seconds(1_000_000_000);
    let res = start + time_break;
    res
    
    
}
