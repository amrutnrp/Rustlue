
mod my_import;
use my_import::*;
mod stucts;
use stucts::*;
use std::thread;

fn main() -> mlua::Result<()> {
    let config_handle = thread::spawn(|| fs::read_to_string("gui_config.toml"));
    let backend_handle = thread::spawn(|| fs::read_to_string("backend.lua"));

    let config_str = config_handle
        .join()
        .map_err(|_| mlua::Error::external("Failed to join config thread"))?
        .map_err(mlua::Error::external)?;

    let config: Config = toml::from_str(&config_str).map_err(mlua::Error::external)?;

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("widgets", lua.create_table()?)?;

    globals.set("get_cwd", lua.create_function(|_, ()| {
        let path = env::current_dir().map_err(mlua::Error::external)?;
        Ok(path.to_string_lossy().into_owned())
    })?)?;

    globals.set("open_file_dialog", lua.create_function(|_, ()| open_file_dialog())?)?;
    globals.set("save_file_dialog", lua.create_function(|_, ()| save_file_dialog())?)?;

    let app = app::App::default();
    let x = config.window.x.unwrap_or(100);
    let y = config.window.y.unwrap_or(100);

    let (default_font, default_size) = parse_font(&config.window.font);
    draw::set_font(default_font, default_size);

    let mut wind = Window::new(x, y, config.window.width, config.window.height, config.window.title.as_str());

    if let Some(bg) = parse_rgb_color(&config.window.bgcolor) {
        wind.set_color(Color::from_u32(bg));
    }

    let mut widget_map: HashMap<String, Rc<RefCell<Widget>>> = HashMap::with_capacity(config.widgets.len());
    let mut buffer_map: HashMap<String, Rc<RefCell<TextBuffer>>> = HashMap::new();

    for w in &config.widgets {
        let wx = w.x.unwrap_or(10);
        let wy = w.y.unwrap_or(10);
        let ww = w.width.unwrap_or(150);
        let wh = w.height.unwrap_or(30);
        let label = w.label.clone().unwrap_or_default();
        let font_size = w.fontsize.unwrap_or(default_size);
        let fg = w.fgcolor.map(Color::from_u32).unwrap_or(Color::Black);
        let bg = w.bgcolor.map(Color::from_u32).unwrap_or(Color::White);
        let id = w.id.clone().unwrap_or_else(|| format!("{}_{}_{}", w.kind, wx, wy));

        let widget: Option<Rc<RefCell<Widget>>> = match w.kind.as_str() {
            "button" => {
                let id_for_lua = id.clone();
                let lua = lua.clone();
                let mut btn = Button::new(wx, wy, ww, wh, label.as_str());
                btn.set_label_size(font_size);
                btn.set_label_color(fg);
                btn.set_color(bg);
                btn.set_frame(FrameType::RFlatBox);

                btn.set_callback(move |_| {
                    let globals = lua.globals();
                    if let Ok(widgets_table) = globals.get::<mlua::Table>("widgets") {
                        if let Ok(widget_table) = widgets_table.get::<mlua::Table>(&*id_for_lua) {
                            if let Ok(func) = widget_table.get::<mlua::Function>("action") {
                                if let Err(e) = func.call::<()>(()) {
                                    eprintln!("Lua error calling action for '{}': {}", id_for_lua, visualize_whitespace(&e.to_string()));
                                }
                            } else {
                                eprintln!("No 'action' function found for widget '{}'", id_for_lua);
                            }
                        } else {
                            eprintln!("No widget table found for '{}'", id_for_lua);
                        }
                    } else {
                        eprintln!("Global 'widgets' table not found in Lua");
                    }
                });

                Some(Rc::new(RefCell::new(unsafe { btn.into_widget() })))
            }
            "label" => {
                let mut frame = Frame::new(wx, wy, ww, wh, label.as_str());
                frame.set_label_size(font_size);
                frame.set_label_color(fg);
                frame.set_color(bg);
                frame.set_frame(FrameType::FlatBox);
                frame.set_align(Align::Wrap | Align::Inside | Align::Left);
                Some(Rc::new(RefCell::new(unsafe { frame.into_widget() })))
            }
            "checkbox" => {
                let mut cb = CheckButton::new(wx, wy, ww, wh, label.as_str());
                cb.set_label_size(font_size);
                cb.set_label_color(fg);
                cb.set_color(bg);

                let cb_rc = Rc::new(RefCell::new(cb));
                let widget_rc: Rc<RefCell<Widget>> = Rc::new(RefCell::new(unsafe { cb_rc.borrow().clone().into_widget() }));
                widget_map.insert(id.clone(), widget_rc.clone());

                let widget_obj = lua.create_table()?;

                let cb_set = Rc::clone(&cb_rc);
                let set_checked = lua.create_function(move |_, checked: bool| {
                    cb_set.borrow_mut().set_value(checked);
                    Ok(())
                })?;
                widget_obj.set("set_checked", set_checked)?;

                let cb_get = Rc::clone(&cb_rc);
                let get_checked = lua.create_function(move |_, ()| {
                    Ok(cb_get.borrow().value())
                })?;
                widget_obj.set("get_checked", get_checked)?;

                let widget_table: mlua::Table = lua.globals().get("widgets")?;
                widget_table.set(id.clone(), widget_obj)?;

                Some(widget_rc)
            }
            "textbox" => {
                let mut editor = TextEditor::new(wx, wy, ww, wh, label.as_str());
                editor.set_text_font(Font::Courier);
                editor.set_text_size(font_size);
                editor.set_text_color(fg);
                editor.set_color(bg);

                let buffer = Rc::new(RefCell::new(TextBuffer::default()));
                buffer.borrow_mut().set_text("");
                editor.set_buffer(Some(buffer.borrow().clone()));

                buffer_map.insert(id.clone(), buffer.clone());

                Some(Rc::new(RefCell::new(unsafe { editor.into_widget() })))
            }
            _ => None,
        };

        if let Some(wgt) = widget {
            widget_map.insert(id, wgt);
        }
    }

    for w in &config.widgets {
        if w.kind == "vbox" || w.kind == "hbox" {
            let wx = w.x.unwrap_or(10);
            let wy = w.y.unwrap_or(10);
            let ww = w.width.unwrap_or(150);
            let wh = w.height.unwrap_or(30);
            let id = w.id.clone().unwrap_or_else(|| format!("{}_{}_{}", w.kind, wx, wy));

            let mut pack = Pack::new(wx, wy, ww, wh, "");
            pack.set_type(if w.kind == "vbox" { PackType::Vertical } else { PackType::Horizontal });
            pack.set_spacing(5);
            pack.begin();

            if let Some(children) = &w.children {
                for child_id in children {
                    if let Some(child_widget) = widget_map.get(child_id) {
                        pack.add(&*child_widget.borrow());
                    }
                }
            }
            pack.end();
            pack.show();
            widget_map.insert(id, Rc::new(RefCell::new(unsafe { pack.into_widget() })));
        }
    }

    let widget_table: mlua::Table = lua.globals().get("widgets")?;

    for (id, widget) in &widget_map {
        let id_clone = id.clone();

        let widget_ref_get = Rc::clone(widget);
        let get_text = lua.create_function(move |_, ()| {
            Ok(widget_ref_get.borrow().label())
        })?;

        let widget_ref_set = Rc::clone(widget);
        let set_text = lua.create_function(move |_, text: String| {
            widget_ref_set.borrow_mut().set_label(&text);
            Ok(())
        })?;

        let widget_ref_color = Rc::clone(widget);
        let set_color = lua.create_function(move |_, color: u32| {
            widget_ref_color.borrow_mut().set_color(Color::from_u32(color));
            Ok(())
        })?;

        let widget_obj = lua.create_table()?;
        widget_obj.set("get_text", get_text)?;
        widget_obj.set("set_text", set_text)?;
        widget_obj.set("set_color", set_color)?;

        if let Some(buffer) = buffer_map.get(id) {
            let buffer_for_get_all = buffer.clone();
            let get_text = lua.create_function(move |_, ()| {
                Ok(buffer_for_get_all.borrow().text())
            })?;
            widget_obj.set("get_text", get_text)?;

            let buffer_for_set = buffer.clone();
            let set_text = lua.create_function(move |_, text: String| {
                buffer_for_set.borrow_mut().set_text(&text);
                Ok(())
            })?;
            widget_obj.set("set_text", set_text)?;

            let buffer_for_get = buffer.clone();
            let get_selected_text = lua.create_function(move |_, ()| {
                Ok(buffer_for_get.borrow().selection_text())
            })?;
            widget_obj.set("get_selected_text", get_selected_text)?;
        }

        if let Ok(existing) = widget_table.get::<mlua::Table>(&*id_clone) {
            for pair in widget_obj.clone().pairs::<String, mlua::Value>() {
                if let Ok((k, v)) = pair {
                    existing.set(k, v)?;
                }
            }
        } else {
            widget_table.set(id_clone, widget_obj)?;
        }
    }

    wind.end();
    wind.show();

    let backend_code = backend_handle
        .join()
        .map_err(|_| mlua::Error::external("Failed to join backend thread"))?
        .map_err(mlua::Error::external)?;
    lua.load(&backend_code).exec().map_err(mlua::Error::external)?;

    if let Ok(main_func) = lua.globals().get::<mlua::Function>("_main_") {
        main_func.call::<()>(()).map_err(mlua::Error::external)?;
    }

    app.run().map_err(mlua::Error::external)?;
    Ok(())
}
