slint::slint!{ import { Demo } from "./src/rects.slint"; }

fn main() {
    Demo::new().unwrap().run().unwrap();
}
