use std::{cmp::Ordering, path::PathBuf};

use dioxus::prelude::*;

use crate::{
    backend::{
        self,
        local::{DirEntry, FileType},
    },
    frontend::Route,
};

#[component]
pub fn Local(mut directory: String) -> Element {
    if directory.is_empty() {
        directory = "/".to_owned();
    }
    let directory = use_signal(|| directory);
    let files = use_resource(move || backend::local::list_files(directory().to_owned()));

    rsx! {
        Path { directory }
        Content { directory, files_resource: files }
    }
}

#[component]
fn Path(directory: Signal<String>) -> Element {
    let directory_string = directory();

    let paths: Vec<_> = directory_string
        .bytes()
        .enumerate()
        .filter_map(|(index, byte)| {
            if byte == b'/' {
                Some(directory_string[..=index].to_owned())
            } else {
                None
            }
        })
        .collect();

    rsx! {
        div { id: "path",
            span { class: "component",
                a { href: "/", "exit" }
            }
            for path in paths {
                span { class: "component", ">" }
                PathComponent { directory, path }
            }
        }
    }
}

#[component]
fn PathComponent(directory: Signal<String>, path: String) -> Element {
    let name = path.split('/').nth_back(1).unwrap_or_default().to_owned();
    rsx! {
        span { class: "component",
            Link {
                class: "dir",
                to: Route::Local {
                    directory: path.clone(),
                },
                onclick: move |_| {
                    directory.set(path.clone());
                },
                "{name}/"
            }
        }
    }
}

#[component]
fn Content(
    directory: Signal<String>,
    files_resource: Resource<Result<Vec<DirEntry>, HttpError>>,
) -> Element {
    let result = files_resource.value().read_unchecked().cloned();

    rsx! {
        div { id: "content" }
        match result {
            Some(Ok(dir_entries)) => rsx! {
                FileList { directory, dir_entries }
            },
            Some(Err(err)) => rsx! {
                span { color: "red", "Got error: {err}!" }
            },
            None => rsx! {
                p { "Loading ..." }
            },
        }
    }
}

#[component]
fn FileList(directory: Signal<String>, dir_entries: Vec<DirEntry>) -> Element {
    // We want directories before files
    dir_entries.sort_by(|a, b| {
        if a == b {
            Ordering::Equal
        } else if matches!(a.file_type, FileType::Directory) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    rsx! {
        for DirEntry { path , file_name , file_type } in dir_entries {
            match file_type {
                FileType::File => rsx! {
                    File { file_name, file_path: path }
                },
                FileType::Directory => rsx! {
                    Directory { directory, file_name, file_path: path }
                },
                FileType::Symlink | FileType::Unknown => rsx! {},
            }
        }
    }
}

#[component]
fn Directory(directory: Signal<String>, file_name: String, file_path: PathBuf) -> Element {
    let path = file_path.display().to_string().replace("\\", "/") + "/";
    let path_clone = path.clone();
    rsx! {
        div { class: "entry",

            Link {
                class: "dir",
                to: Route::Local { directory: path },
                onclick: move |_| {
                    directory.set(path_clone.clone());
                },
                "{file_name}/"
            }
        }
    }
}

#[component]
fn File(file_name: String, file_path: PathBuf) -> Element {
    /*
    var form = document.createElement("form");
    form.className = "entry";
    form.action = "/local/play/"+
        encodeURIComponent(location.href)+"/"+
        encodeURIComponent(file.path);
    form.method = "post";

    var el = document.createElement("a");
    el.className = "file";
    el.innerText = file.name;
    el.href = "javascript:void";
    el.onclick = form.submit.bind(form);

    form.appendChild(el);

    return form;
    */
    rsx! {
        form { class: "entry", method: "post",
            // action: "/local/play/"+ encodeURIComponent(location.href)+"/"+ encodeURIComponent(file.path);
            a { class: "file", "{file_name}" }
        }
    }
}
