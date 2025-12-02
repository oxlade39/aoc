use std::time::Duration;

pub fn format_elapsed_time(duration: Duration) -> String {
    let secs = duration.as_secs_f64();
    let nanos = duration.subsec_nanos();

    if secs < 1.0 {
        if nanos < 1_000 {
            format!("{:.0} ns", nanos)
        } else if nanos < 1_000_000 {
            format!("{:.0} µs", nanos / 1_000)
        } else {
            format!("{:.0} ms", nanos / 1_000_000)
        }
    } else {
        if secs < 60.0 {
            format!("{:.2} s", secs)
        } else if secs < 3600.0 {
            format!("{:.2} min", secs / 60.0)
        } else {
            format!("{:.2} h", secs / 3600.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::timing::*;

    #[test]
    fn test_format() {
        assert_eq!("1.00 s", format_elapsed_time(Duration::from_secs(1)));
        assert_eq!("1.34 s", format_elapsed_time(Duration::from_secs_f64(1.34)));
        assert_eq!("340 ms", format_elapsed_time(Duration::from_secs_f64(0.34)));
        assert_eq!("34 ms", format_elapsed_time(Duration::from_secs_f64(0.034)));
        assert_eq!("3 ms", format_elapsed_time(Duration::from_secs_f64(0.0034)));
        assert_eq!(
            "340 µs",
            format_elapsed_time(Duration::from_secs_f64(0.00034))
        );
        assert_eq!(
            "34 µs",
            format_elapsed_time(Duration::from_secs_f64(0.000034))
        );
        assert_eq!(
            "3 µs",
            format_elapsed_time(Duration::from_secs_f64(0.0000034))
        );
        assert_eq!(
            "340 ns",
            format_elapsed_time(Duration::from_secs_f64(0.00000034))
        );
    }
}
