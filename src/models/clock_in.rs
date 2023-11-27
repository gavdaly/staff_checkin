struct ClockIn {
    id: Uuid,
    check_type: CheckType,
    time: DateTime<Local>,
    user_verified: bool
}

enum CheckType {
    Location{lat: f64, lng: f64, accuracy: f64},
    Qr {code: String},
    Manual {submitted_at: DateTime<Local>, reason: String}
}