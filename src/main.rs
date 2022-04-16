#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gl_test::GlTest;

fn main() {
    let windowed_context = GlTest::new(800., 600.).unwrap();
    windowed_context.run();
}
