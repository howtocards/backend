
#[derive(Serialize)]
pub struct ErrorAnswer {
    ok: bool,
    error: String,
}

impl ErrorAnswer {
    pub fn new(error: String) -> Self {
        ErrorAnswer { ok: false, error }
    }
}

#[derive(Serialize)]
pub struct SuccessAnswer<T> {
    ok: bool,
    result: T,
}

impl<T> SuccessAnswer<T> {
    pub fn new(result: T) -> Self {
        SuccessAnswer { ok: true, result }
    }
}
