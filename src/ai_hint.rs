use crate::color::ColorRGB;

pub fn get_hint(
    target: &ColorRGB,
    guess: &ColorRGB,
) -> String {

    let mut hints = Vec::new();

    check_channel(
        "Red",
        target.r,
        guess.r,
        &mut hints,
    );

    check_channel(
        "Yellow",
        target.y,
        guess.y,
        &mut hints,
    );

    check_channel(
        "Blue",
        target.b,
        guess.b,
        &mut hints,
    );

    if hints.is_empty() {
        return "Excellent! You're very close.".to_string();
    }

    hints.join("\n")
}

fn check_channel(
    name: &str,
    target: u8,
    guess: u8,
    hints: &mut Vec<String>,
) {

    let diff = target as i16 - guess as i16;

    if diff.abs() <= 10 {
        return;
    }

    let strength =
        if diff.abs() > 60 {
            "significantly"
        } else if diff.abs() > 25 {
            ""
        } else {
            "slightly"
        };

    if diff > 0 {
        hints.push(format!(
            "Increase {} {}",
            name,
            strength
        ));
    } else {
        hints.push(format!(
            "Decrease {} {}",
            name,
            strength
        ));
    }

}