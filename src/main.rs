use tracing::{field, Level};

fn main() {
    // Set up where Tokio events get logged to.
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .json()
        .with_current_span(true)
        .flatten_event(true)
        .init();

    // Create an error
    let e = MyError {
        msg: "Something bad happened",
        fields: Fields {
            account_id: Some("a234fe".to_owned()),
            ..Default::default()
        },
    };

    // Log the error.
    e.record();
}

/// A structured error type.
#[derive(Default)]
pub struct MyError {
    pub msg: &'static str,
    pub fields: Fields,
}

/// Business logic. In this example, we're modelling a game server.
#[derive(Default)]
pub struct Fields {
    pub account_id: Option<String>,
    pub game_id: Option<i64>,
    pub highscore: Option<u32>,
}

impl MyError {
    /// Log the error.
    pub fn record(self) {
        // The tracing lib requires fields to be known at compile time, so
        // initialize them all, but don't actually set any of them.
        let span = tracing::error_span!(
            "my_error",
            account_id = field::Empty,
            game_id = field::Empty,
            highscore = field::Empty
        );

        // If a given field is set in the Fields object, also set it in the
        // tracing span.
        if let Some(account_id) = self.fields.account_id {
            span.record("account_id", &account_id.as_str());
        }
        if let Some(game_id) = self.fields.game_id {
            span.record("game_id", &game_id);
        }
        if let Some(highscore) = self.fields.highscore {
            span.record("highscore", &highscore);
        }

        // Actually log the error.
        let _enter = span.enter();
        tracing::error!(error = self.msg);
    }
}
