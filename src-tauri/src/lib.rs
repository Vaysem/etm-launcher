
use std::{env, fs, path::Path, sync::Mutex};
use tauri::{ Builder, Manager, Url, image::Image, Size, PhysicalSize};
use winreg::{enums::*, RegKey};
use scraper::{Html, Selector};
use once_cell::sync::Lazy;

#[derive(Debug, Default)]
pub struct Document {
    name: String,
    title: String,
    path: String,
    icon: String,
    width: u16,
    height: u16,
    resize: bool,
    maximize: bool,
}

const TYPE_FILE: &str = "etm";
const MIME: &str = "text/html";
pub static ERRORS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn log_error(msg: impl Into<String>) {
    ERRORS.lock().unwrap().push(msg.into());
}

/*
#[cfg(target_os = "windows")] 
pub fn set_icon(win: &tauri::WebviewWindow, icon_path: &str) { 
    use windows::Win32::UI::WindowsAndMessaging::*; 
    use windows::Win32::Foundation::*; 
    use windows::core::PCWSTR; 
    use std::ffi::OsStr; 
    use std::os::windows::ffi::OsStrExt; 
    let raw = win.hwnd().unwrap().0 as isize; 
    let hwnd = HWND(raw as *mut std::ffi::c_void); 
    let mut wpath: Vec<u16> = OsStr::new(icon_path).encode_wide().collect(); 
    wpath.push(0); 
    unsafe { 
        let hicon = LoadImageW( Some(HINSTANCE(std::ptr::null_mut())), PCWSTR(wpath.as_ptr()), IMAGE_ICON, 0, 0, LR_LOADFROMFILE, ); 
        if hicon.clone().expect("REASON").is_invalid() { 
            log_error("Failed to load icon"); 
            return; 
        } 
        SendMessageW(hwnd, WM_SETICON, Some(WPARAM(1)), Some(LPARAM(hicon.clone().unwrap().0 as isize))); 
        SendMessageW(hwnd, WM_SETICON, Some(WPARAM(0)), Some(LPARAM(hicon.clone().unwrap().0 as isize))); 
    } 
}
*/

fn registration_type_file() -> bool {
    if isset_my_mime() {
        return true;
    }

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let key_path = format!(".{}", TYPE_FILE);
    
    let (key, _disp) = match hkcr.create_subkey(&key_path) {
        Ok(pair) => pair,
        Err(e) => {
            log_error(format!("Failed to create subkey \\{}: {e:#?}", key_path));
            return false;
        }
    };

    match key.set_value("Content Type", &MIME) {
        Ok(_) => true,
        Err(e) => {
            log_error(format!("Failed to set Content Type: {e:#?}"));
            false
        }
    }
}

fn isset_my_mime() -> bool {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let key_path = format!(".{}", TYPE_FILE);

    let key = match hkcr.open_subkey_with_flags(&key_path, KEY_READ) {
        Ok(k) => k,
        Err(r) =>{
            log_error(format!("Failed to open subkey: {r:#?}"));
            return false
        }
    };

    match key.get_value::<String, _>("Content Type") {
        Ok(v) => v.eq_ignore_ascii_case(MIME),
        Err(r) => {
            log_error(format!("Failed to get value: {r:#?}"));
            false
        }
    }
}

fn valid_main_format(path: &str) -> bool {
    let path = Path::new(path);
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case(TYPE_FILE))
        .unwrap_or(false)
        && path.is_file()
}

fn valid_file(path: &str) -> bool {
    Path::new(path).is_file()
}

fn get_u16(text: &str) -> u16 {
    let get_number: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
    match get_number.parse::<u16>() {
        Ok(num) => num,
        Err(r) => {
            log_error(format!("Is not int value: {r:#?}"));
            0u16
        },
    }
}

fn parse_data_file(path: &str) -> Document {
    let html_content = fs::read_to_string(path).unwrap();
    let file_path = Path::new(path);
    let document = Html::parse_document(&html_content);
    let folder = file_path.parent().unwrap();

    let mut config = Document {
        name: "".to_string(),
        title: "".to_string(),
        icon: "".to_string(),
        path: path.to_string(),
        width: 600,
        height: 400,
        resize: true,
        maximize: true,
    };

    config.icon = match document
        .select(&Selector::parse(r#"head > link[rel*="icon"][href$=".png"], head > link[rel*="icon"][href$=".ico"]"#).unwrap())
        .next()
    {
        Some(e) => {
            let href = match e.value().attr("href") {
                Some(hr) => {
                    let icon_file = format!("{}\\{}", folder.to_string_lossy(), hr.trim_matches('/').replace("/", "\\"));
                    if valid_file(&icon_file) {
                        icon_file.to_string()
                    } else {
                        log_error(format!("Not icon file: {}", icon_file.to_string()));
                        String::new()
                    }
                }
                None => String::new(),
            };
            href
        }
        None => String::new(),
    };

    config.name = file_path
        .file_stem()
        .and_then(|e| e.to_str())
        .unwrap_or("...")
        .to_string();

    config.title = match document
        .select(&Selector::parse(r#"title"#).unwrap())
        .next()
    {
        Some(e) => e.text().collect::<String>(),
        None => config.name.clone(),
    };

    if let Some(meta_conf) = document
        .select(&Selector::parse(r#"meta[type=window]"#).unwrap())
        .next()
    {
        let attrs = meta_conf.value();

        if let Some(width_str) = attrs.attr("width") {
            let width = get_u16(width_str);
            if width > 100 {
                config.width = width;
            }
        }
        if let Some(height_str) = attrs.attr("height") {
            let height = get_u16(height_str);
            if height > 0 {
                config.height = height;
            }
        }
        if let Some(resize_val) = attrs.attr("resize") {
            if resize_val.to_lowercase().trim() == "false" {
                config.resize = false;
            }
        }

        if let Some(maximize_val) = attrs.attr("maximize") {
            if maximize_val.to_lowercase().trim() == "false" {
                config.maximize = false;
            }
        }
    }
    
    config
}

pub fn run() {
    registration_type_file();
    let args: Vec<String> = env::args().collect();
    let mut etm: Document = Default::default();
        
    if args.len() > 1 && valid_main_format(&args[1]) {        
        etm = parse_data_file(&args[1]);
    }else{
        return;
    }

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(move | app | {

            if !etm.path.is_empty() {
                if let Some(encoded_path) = Url::from_file_path(&etm.path).ok() {
                    let win = app.get_webview_window("main").unwrap();
                    match win.navigate(encoded_path) {
                        Ok(_) => {
                            if !etm.icon.is_empty(){ 
                                //set_icon(&win, &etm.icon);                                
                                match Image::from_path(Path::new(&etm.icon)) {
                                    Ok(im)=> {win.set_icon(im).unwrap()},
                                    Err(e)=>log_error(format!("Error icon format: {}\n{}", &etm.icon, e))
                                };
                            }
                            
                            win.set_size(Size::Physical(PhysicalSize {
                                width: etm.width.into(),
                                height: etm.height.into(),
                            })).unwrap();
                            win.set_title(&etm.title).unwrap();
                            win.set_maximizable(etm.maximize).unwrap();
                            win.set_resizable(etm.resize).unwrap();
                            win.center().unwrap();
                            win.show().unwrap();
                            win.set_focus().unwrap();

                        },
                        Err(e) => log_error(format!("Error URL path: {}\n{:#?}", etm.path, e)),
                    }
                    for err in ERRORS.lock().unwrap().iter() {
                        let escaped = serde_json::to_string(err).unwrap(); // гарантированный escape
                        win.eval(format!("console.warn({escaped})")).unwrap();
                    }
                };
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
