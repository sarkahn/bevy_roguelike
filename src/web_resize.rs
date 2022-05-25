// MIT License

// Copyright (c) 2020 Carl Öst Wilkens

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use bevy::{
    prelude::{App, Plugin, Res, ResMut},
    window::Windows,
};
use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

type OnResizeSender = Sender<()>;
type OnResizeReceiver = Receiver<()>;

pub struct FullViewportPlugin;

#[cfg(target_arch = "wasm32")]
fn build(app:&mut App) {
    let channel = std::sync::mpsc::channel();
    let resize_sender: OnResizeSender = channel.0;
    let resize_receiver: OnResizeReceiver = channel.1;
    
    app.insert_resource(Mutex::new(resize_sender))
        .insert_resource(Mutex::new(resize_receiver))
        .add_startup_system(setup_viewport_resize_system.system())
        .add_system(viewport_resize_system.system());
}

#[cfg(not(target_arch = "wasm32"))]
fn build(_app:&mut App) {

}

impl Plugin for FullViewportPlugin {
    fn build(&self, app: &mut App) {
        //if cfg!(wasm) {
            build(app);
        //}
    }
}

#[allow(dead_code)]
fn get_viewport_size() -> (f32, f32) {
    let web_window = web_sys::window().expect("could not get window");
    let document_element = web_window
        .document()
        .expect("could not get document")
        .document_element()
        .expect("could not get document element");

    let width = document_element.client_width();
    let height = document_element.client_height();

    (width as f32, height as f32)
}

#[allow(dead_code)]
fn setup_viewport_resize_system(resize_sender: Res<Mutex<OnResizeSender>>) {
    let web_window = web_sys::window().expect("could not get window");
    let local_sender = resize_sender.lock().unwrap().clone();

    local_sender.send(()).unwrap();

    gloo_events::EventListener::new(&web_window, "resize", move |_event| {
        local_sender.send(()).unwrap();
    })
    .forget();
}

#[allow(dead_code)]
fn viewport_resize_system(
    mut windows: ResMut<Windows>,
    resize_receiver: Res<Mutex<OnResizeReceiver>>,
) {
    if resize_receiver.lock().unwrap().try_recv().is_ok() {
        if let Some(window) = windows.get_primary_mut() {
            let size = get_viewport_size();
            window.set_resolution(size.0, size.1);
        }
    }
}
