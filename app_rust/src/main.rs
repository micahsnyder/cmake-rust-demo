/*
 * Example rust app.
 *
 * Copyright (C) 2022 Micah Snyder.
 */

fn main() {
    let uuid = gen_uuid::gen_uuid();
    println!("Hello, {}!", uuid);
}
