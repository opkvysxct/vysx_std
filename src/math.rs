/// Returns `true` if `min < val < max` (both bounds exclusive).
///
/// Returns `false` if any argument is unordered (e.g. NaN).
pub fn inside<T: PartialOrd>(val: T, min: T, max: T) -> bool {
	val > min && val < max
}

/// Returns `true` if `min <= val <= max` (both bounds inclusive).
///
/// Returns `false` if any argument is unordered (e.g. NaN).
pub fn inside_eq<T: PartialOrd>(val: T, min: T, max: T) -> bool {
	val >= min && val <= max
}

/// Returns `true` if `val < min` or `val > max` (both bounds exclusive).
///
/// Values equal to `min` or `max` are neither [`inside`] nor `outside`.
pub fn outside<T: PartialOrd>(val: T, min: T, max: T) -> bool {
	val < min || val > max
}

/// Returns `true` if `val <= min` or `val >= max` (both bounds inclusive).
///
/// Values equal to `min` or `max` count as outside, so this is the exact
/// complement of [`inside`] for ordered values.
pub fn outside_eq<T: PartialOrd>(val: T, min: T, max: T) -> bool {
	val <= min || val >= max
}
