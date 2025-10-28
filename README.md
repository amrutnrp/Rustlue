# 🧩 Rustlue

A **GUI framework with Rust + FLTK + Lua** 
   - `Rust`: because it creates stable executable with no external dependecies like .dll /.lib etc
   - `FLTK`: GUIs with minimal size and most features
   - `Lua `: Can be embedded inside the executable itself and doesn't need another installer

---

## ✨ Features
-  **Tiny Size** : 5MB executable - includes GUI & language interpreter
-  **PC Architecture support** : Supported even on ARM based PCs.    
-  **No external dependencies** : no extra DLL needed, no other installations
-  **Fast Interation** : Helps in rapid development of script & GUI
-  **Declarative UI**: Define window & widgets via a TOML config file, no need to learn any GUI library specific code
-  **Lua scripting**: blazing fast execution and interpreter embedded inside
- <span style="color:gray"> **Customizable styling**: Fonts, colors, layout, and themes</span> <br>
- <span style="color:gray"> **Modify Widgets in Lua**: Get/set text, color, state, and more</span>


---

## 🚀 Getting Started / Installation Guide

1. **Clone the repo**  
   ```bash
   git clone https://github.com/amrutnrp/Rustlue.git
   cd Rustlue
   ```
2. **Build the app**  
   ```bash
   cargo build
   ```
3. **Add your config and script**  
   Place the executable `Rustlue/target/debug/Rustlue.exe` into sired location <br>
   Place `gui_config.toml` and `backend.lua` in the same folder


---

## 📄 Example Config (`gui_config.toml`)

```toml
[window]
title = "My App"
width = 800
height = 600
font = "helvetica,16"
bgcolor = "255-255-255"

[[widgets]]
type = "button"
label = "Click Me"
x = 100
y = 150
width = 120
height = 40
bgcolor = 16711680
id = "my_button"

[[widgets]]
type = "checkbox"
label = "Enable Feature"
x = 100
y = 200
id = "feature_toggle"

[[widgets]]
type = "textbox"
x = 100
y = 250
width = 300
height = 100
id = "editor"
```

---

## 🧠 Example Lua Script (`backend.lua`)

```lua
widgets["my_button"] = {
  action = function()
    local checked = widgets["feature_toggle"]:get_checked()
    print("Button clicked! Feature enabled:", checked)
  end
}

function _main_()
  widgets["editor"]:set_text("Welcome to the app!")
end
```
---
## Limitations
If your usage falls into the following, suggest not to use this repo
- **Mutliple Windows**
- **Complex Libraries** : such as openpyxl / graphics / plotting libraries etc
- **Dynamic Widgets** : code is expected to create/ destroy widgets during the execution
- **OS support** : have not tested it on Linux based OS
- **Asthetics** : FLTK looks old and outdated. But, it gets the job done. 





---

## 📜 License

MIT License. See `LICENSE` for details. 

## ☕ Buy me a Coffee

Not really, :P <br>
If you're using this and found it helpful, let me know , I'll be glad to hear that.

