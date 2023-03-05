#[derive(Debug, pyo3::prelude::FromPyObject)]
pub enum RendererCommand {
    DrawRectangle {
        position: (f32, f32, f32),
        color: (u8, u8, u8, u8),
    },
    SetClearColor {
        color: (u8, u8, u8, u8),
    },
}

#[derive(Debug)]
#[pyo3::prelude::pyclass]
pub struct SharedBuffer {
    pub data: Vec<RendererCommand>,
}

#[pyo3::prelude::pymethods]
impl SharedBuffer {
    pub fn append(&mut self, value: RendererCommand) {
        self.data.push(value)
    }
}

fn main() {
    let code = std::fs::read_to_string("game/game.py").unwrap();

    pyo3::Python::with_gil(|py| {
        let game_module = pyo3::prelude::PyModule::from_code(py, &code, "game.py", "game").unwrap();

        let game_state = game_module.call_method0("game_init").unwrap();
        let buffer = pyo3::PyCell::new(py, SharedBuffer { data: vec![] }).unwrap();

        loop {
            println!("");

            {
                let t = std::time::Instant::now();
                let buffer_borrow = buffer.borrow_mut();
                let _ = game_module.call_method1("game_update", (game_state,));
                println!("[ RS ] calling game_update took: {:?}", t.elapsed());

                let t = std::time::Instant::now();
                game_module
                    .call_method1("game_render", (game_state, buffer_borrow))
                    .unwrap();
                println!("[ RS ] calling game_render took: {:?}", t.elapsed());
            }

            {
                let t = std::time::Instant::now();
                let mut buffer_borrow = buffer.borrow_mut();
                for command in buffer_borrow.data.drain(..) {
                    match command {
                        RendererCommand::DrawRectangle { position, color } => {
                            // println!(
                            //     "[ RS ] [ INFO ] Drawing rectangle at {position:?} with color {color:?}"
                            // );
                        }

                        RendererCommand::SetClearColor { color } => {
                            // println!("[ RS ] [ INFO ] Setting clear color to {color:?}")
                        }
                    }
                }
                println!("[ RS ] handling renderer commands took: {:?}", t.elapsed());
            }

            {
                let t = std::time::Instant::now();
                let mut buffer_borrow = buffer.borrow_mut();
                for _ in 0..3300 {
                    buffer_borrow.data.push(RendererCommand::DrawRectangle {
                        position: (0.0, 0.0, 0.0),
                        color: (255, 255, 255, 255),
                    });
                }

                println!(
                    "[ RS ] Doing the same as game_render does but in rust took: {:?}",
                    t.elapsed()
                );
                buffer_borrow.data.clear();
            }
        }
    });
}
