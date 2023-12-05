use crate::config::Config;

use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExtInput {
    args: Vec<String>,
    filename: std::path::PathBuf,
}

const EDITOR_ENV: &str = "helix:editor/env";

pub(crate) fn cmd(
    cx: &mut compositor::Context,
    args: &[Cow<str>],
    event: PromptEvent,
) -> anyhow::Result<()> {
    if event != PromptEvent::Validate {
        return Ok(());
    }

    ensure!(
        args.len() >= 2,
        ":ext takes at least 2 arguments: filename, function name"
    );
    let user_data = extism::UserData::new(cx.editor as *mut Editor);
    let (_view, doc) = current!(cx.editor);
    let path = doc.path().and_then(|x| x.to_str()).unwrap_or_default();
    let plugin = &args[0];
    let config = Config::load_default().map_err(|x| anyhow::Error::msg(x.to_string()))?;
    let manifest = match config.plugins.get(plugin.as_ref()) {
        None => extism::Manifest::new([extism::Wasm::file(plugin.as_ref())]),
        Some(p) => p.clone(),
    };
    let function_name = &args[1];
    let rest = &args[2..];
    let mut plugin = extism::PluginBuilder::new(manifest)
        .with_wasi(true)
        .with_function_in_namespace(
            EDITOR_ENV,
            "save",
            [extism::PTR],
            [],
            user_data.clone(),
            |plugin, inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let id = {
                    let (_view, doc) = current!(editor);
                    doc.id()
                };
                let p = if inputs[0].unwrap_i64() != 0 {
                    Some(plugin.memory_get_val::<&str>(&inputs[0])?)
                } else {
                    None
                };
                editor.save(id, p, false)?;
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "set_status",
            [extism::PTR],
            [],
            user_data.clone(),
            |plugin, inputs, _outputs, user_data| {
                let status: &str = plugin.memory_get_val(&inputs[0])?;
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                editor.set_status(status.to_string());
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "clear_status",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                editor.clear_status();
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "set_path",
            [extism::PTR],
            [],
            user_data.clone(),
            |plugin, inputs, _outputs, user_data| {
                let path: &str = plugin.memory_get_val(&inputs[0])?;
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (_view, doc) = current!(editor);
                doc.set_path(Some(path.as_ref()));
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "undo",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (mut view, doc) = current!(editor);
                doc.undo(&mut view);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "redo",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (mut view, doc) = current!(editor);
                doc.redo(&mut view);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "open",
            [extism::PTR],
            [],
            user_data.clone(),
            |plugin, inputs, _outputs, user_data| {
                let path: &str = plugin.memory_get_val(&inputs[0])?;
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (path, pos) = args::parse_file(path);
                let path = helix_core::path::expand_tilde(&path);
                let _ = editor.open(&path, Action::Replace)?;
                let (view, doc) = current!(editor);
                let pos = Selection::point(pos_at_coords(doc.text().slice(..), pos, true));
                doc.set_selection(view.id, pos);
                align_view(doc, view, Align::Center);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "close",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let id = {
                    let (_view, doc) = current!(editor);
                    doc.id()
                };
                editor
                    .close_document(id, true)
                    .map_err(|_| anyhow::Error::msg("close failed"))?;
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "vsplit",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let _doc = editor.new_file(Action::VerticalSplit);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "hsplit",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                editor.new_file(Action::HorizontalSplit);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "focus_next",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                editor.focus_next();
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "focus_prev",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                editor.focus_prev();
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "selection_insert_text_after",
            [extism::PTR],
            [],
            user_data.clone(),
            |plugin, inputs, _outputs, user_data| {
                let text: &str = plugin.memory_get_val(&inputs[0])?;
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (view, doc) = current!(editor);
                let sels = doc.selections();
                let mut txn: Option<Transaction> = None;
                for (_, sel) in sels.iter() {
                    let x = Transaction::insert(
                        doc.text(),
                        &Selection::point(sel.primary().anchor),
                        text.into(),
                    );

                    match txn {
                        Some(t) => txn = Some(t.compose(x)),
                        None => {
                            txn = Some(x);
                        }
                    }
                }
                if let Some(txn) = txn {
                    doc.apply(&txn, view.id);
                }
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "selection_insert_text_before",
            [extism::PTR],
            [],
            user_data.clone(),
            |plugin, inputs, _outputs, user_data| {
                let text: &str = plugin.memory_get_val(&inputs[0])?;
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (view, doc) = current!(editor);
                let sels = doc.selections();
                let mut txn: Option<Transaction> = None;
                for (_, sel) in sels.iter() {
                    let x = Transaction::insert(
                        doc.text(),
                        &Selection::point(sel.primary().anchor),
                        text.into(),
                    );

                    match txn {
                        Some(t) => txn = Some(t.compose(x)),
                        None => {
                            txn = Some(x);
                        }
                    }
                }
                if let Some(txn) = txn {
                    doc.apply(&txn, view.id);
                }
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "selection_replace_text",
            [extism::PTR],
            [],
            user_data.clone(),
            |plugin, inputs, _outputs, user_data| {
                let text: &str = plugin.memory_get_val(&inputs[1])?;
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (view, doc) = current!(editor);
                let sel = doc.selection(view.id);
                let txn = Transaction::change_by_selection(doc.text(), &sel, |range| {
                    (range.from(), range.to(), Some(text.into()))
                });
                doc.apply(&txn, view.id);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "selection_add",
            [extism::ValType::I64, extism::ValType::I64],
            [extism::ValType::I64],
            user_data.clone(),
            |_plugin, inputs, outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (view, doc) = current!(editor);
                let sel = doc.selection(view.id).clone();
                let a = inputs[0].unwrap_i64() as u64;
                let b = inputs[1].unwrap_i64() as u64;
                let sel = sel.push(Range::new(a as usize, b as usize));
                let n = sel.len();
                doc.set_selection(view.id, sel);
                outputs[0] = extism::Val::I64(n as i64);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "selection_reset",
            [],
            [],
            user_data.clone(),
            |_plugin, _inputs, _outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (view, doc) = current!(editor);
                doc.reset_selection(view.id);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "selection_count",
            [],
            [extism::ValType::I64],
            user_data.clone(),
            |_plugin, _inputs, outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (view, doc) = current!(editor);
                let sel = doc.selection(view.id);
                outputs[0] = extism::Val::I64(sel.ranges().len() as i64);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "selection_begin",
            [extism::ValType::I64],
            [extism::ValType::I64],
            user_data.clone(),
            |_plugin, inputs, outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (view, doc) = current!(editor);
                let sel = doc.selection(view.id);
                outputs[0] =
                    extism::Val::I64(sel.ranges()[inputs[0].unwrap_i64() as usize].from() as i64);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "selection_end",
            [extism::ValType::I64],
            [extism::ValType::I64],
            user_data.clone(),
            |_plugin, inputs, outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (view, doc) = current!(editor);
                let sel = doc.selection(view.id);
                outputs[0] =
                    extism::Val::I64(sel.ranges()[inputs[0].unwrap_i64() as usize].to() as i64);
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "text",
            [extism::ValType::I64, extism::ValType::I64],
            [extism::PTR],
            user_data.clone(),
            |plugin, inputs, outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let range = Range::new(
                    inputs[0].unwrap_i64() as usize,
                    inputs[1].unwrap_i64() as usize,
                );
                let (_view, doc) = current!(editor);
                let s = range.slice(doc.text().slice(..)).to_string();
                plugin.memory_set_val(&mut outputs[0], &s)?;
                Ok(())
            },
        )
        .with_function_in_namespace(
            EDITOR_ENV,
            "language_name",
            [],
            [extism::PTR],
            user_data.clone(),
            |plugin, _inputs, outputs, user_data| {
                let editor = user_data.get()?;
                let mut editor = editor.lock().unwrap();
                let editor: &mut Editor = unsafe { &mut **editor };
                let (_view, doc) = current!(editor);
                plugin.memory_set_val(&mut outputs[0], doc.language_name().unwrap_or_default())?;
                Ok(())
            },
        )
        .build()?;
    let res: anyhow::Result<()> = plugin.call(
        function_name,
        extism::convert::Json(ExtInput {
            args: rest.into_iter().map(|x| x.to_string()).collect(),
            filename: PathBuf::from(path),
        }),
    );

    if let Err(e) = &res {
        let _ = std::fs::write("error.txt", e.to_string());
    }

    res?;
    Ok(())
}
