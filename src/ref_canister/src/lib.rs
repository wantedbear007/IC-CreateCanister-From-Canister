#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[ic_cdk::query]
fn bol_hi() -> String {
    format!("ye le hiiiii")
}