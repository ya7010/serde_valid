#[cfg(any(feature = "chrono", feature = "jiff"))]
use serde_valid::utils::{
    duration_exclusive_maximum, duration_exclusive_minimum, duration_maximum, duration_minimum,
    DurationValue,
};

#[cfg(any(feature = "chrono", feature = "jiff"))]
fn check_boundaries<T: DurationValue + Clone>(before: T, boundary: T, after: T) {
    assert!(duration_minimum(boundary.clone())(&before).is_err());
    assert!(duration_minimum(boundary.clone())(&boundary).is_ok());
    assert!(duration_minimum(boundary.clone())(&after).is_ok());
    assert!(duration_maximum(boundary.clone())(&before).is_ok());
    assert!(duration_maximum(boundary.clone())(&boundary).is_ok());
    assert!(duration_maximum(boundary.clone())(&after).is_err());
    assert!(duration_exclusive_minimum(boundary.clone())(&before).is_err());
    assert!(duration_exclusive_minimum(boundary.clone())(&boundary).is_err());
    assert!(duration_exclusive_minimum(boundary.clone())(&after).is_ok());
    assert!(duration_exclusive_maximum(boundary.clone())(&before).is_ok());
    assert!(duration_exclusive_maximum(boundary.clone())(&boundary).is_err());
    assert!(duration_exclusive_maximum(boundary)(&after).is_err());
}

#[cfg(feature = "chrono")]
mod chrono_tests {
    use super::*;
    use chrono::TimeDelta;
    use serde_valid::Validate;

    #[test]
    fn signed_nanosecond_boundaries() {
        check_boundaries(
            TimeDelta::nanoseconds(-2),
            TimeDelta::nanoseconds(-1),
            TimeDelta::zero(),
        );
        check_boundaries(
            TimeDelta::try_milliseconds(-i64::MAX).unwrap(),
            TimeDelta::zero(),
            TimeDelta::try_milliseconds(i64::MAX).unwrap(),
        );
    }

    #[test]
    fn derive_reports_duration_errors() {
        #[derive(Validate)]
        struct Data {
            #[validate(custom = duration_minimum(TimeDelta::zero()))]
            duration: TimeDelta,
        }

        assert!(Data {
            duration: TimeDelta::zero()
        }
        .validate()
        .is_ok());
        let value = TimeDelta::nanoseconds(-1);
        let errors =
            serde_json::to_value(Data { duration: value }.validate().unwrap_err()).unwrap();
        assert_eq!(
            errors["properties"]["duration"]["errors"][0],
            format!(
                "Duration {value:?} is less than minimum {:?}.",
                TimeDelta::zero()
            )
        );
    }
}

#[cfg(feature = "jiff")]
mod jiff_tests {
    use super::*;
    use jiff::SignedDuration;
    use serde_valid::Validate;

    #[test]
    fn signed_nanosecond_boundaries() {
        check_boundaries(
            SignedDuration::from_nanos(-2),
            SignedDuration::from_nanos(-1),
            SignedDuration::ZERO,
        );
        check_boundaries(
            SignedDuration::MIN,
            SignedDuration::ZERO,
            SignedDuration::MAX,
        );
    }

    #[test]
    fn derive_reports_duration_errors() {
        #[derive(Validate)]
        struct Data {
            #[validate(custom = duration_exclusive_maximum(SignedDuration::ZERO))]
            duration: SignedDuration,
        }

        assert!(Data {
            duration: SignedDuration::from_nanos(-1)
        }
        .validate()
        .is_ok());
        let errors = serde_json::to_value(
            Data {
                duration: SignedDuration::ZERO,
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();
        assert_eq!(
            errors["properties"]["duration"]["errors"][0],
            format!(
                "Duration {:?} is greater than or equal to exclusive maximum {:?}.",
                SignedDuration::ZERO,
                SignedDuration::ZERO
            )
        );
    }
}
