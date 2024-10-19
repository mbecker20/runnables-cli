use std::{
  fs,
  path::{Path, PathBuf},
  rc::Rc,
  str::FromStr,
};

use crate::{
  runnables::{ignore_dir, AddRunnables, RunRunnable},
  types::{Runnable, RunnableParams, RunnableParamsVariant},
  CliArgs,
};

use self::{runfile::RunFile, rust_bin::RustBin, rust_lib::RustLib, shell::Shell};

// pub mod javascript;
pub mod runfile;
pub mod rust_bin;
pub mod rust_lib;
pub mod shell;

pub fn get_runnables(args: &CliArgs) -> anyhow::Result<Vec<Runnable>> {
  let path = PathBuf::from_str(&args.path)?;

  let runincludes = get_runincludes(&path);
  let runignores = get_runignores(&path, &runincludes);

  let mut runnables = Vec::new();

  if !args.ignore.contains(&RunnableParamsVariant::RunFile) {
    RunFile::add_runnables(&path, &runincludes, &runignores, &mut runnables);
  }
  if !args.ignore.contains(&RunnableParamsVariant::Shell) {
    Shell::add_runnables(&path, &runincludes, &runignores, &mut runnables);
  }
  if !args.ignore.contains(&RunnableParamsVariant::RustBin) {
    RustBin::add_runnables(&path, &runincludes, &runignores, &mut runnables);
  }
  if !args.ignore.contains(&RunnableParamsVariant::RustLib) {
    RustLib::add_runnables(&path, &runincludes, &runignores, &mut runnables);
  }

  Ok(runnables)
}

pub fn run_runnable(runnable: &Runnable, runnables: &[Rc<Runnable>]) {
  match &runnable.params {
    RunnableParams::RunFile(params) => RunFile::run(runnable, params, runnables),
    RunnableParams::Shell(params) => Shell::run(runnable, params, runnables),
    RunnableParams::RustBin(params) => RustBin::run(runnable, params, runnables),
    RunnableParams::RustLib(params) => RustLib::run(runnable, params, runnables),
    RunnableParams::None => {
      println!("got NONE runnable")
    }
  }
}

fn get_runincludes(path: &Path) -> Vec<PathBuf> {
  let mut runincludes = Vec::<PathBuf>::new();
  get_runincludes_inner(path, &mut runincludes);
  runincludes
}

fn get_runincludes_inner(path: &Path, runincludes: &mut Vec<PathBuf>) {
  add_runincludes(path, runincludes);
  let Ok(entries) = fs::read_dir(path) else {
    return;
  };
  for entry in entries.flatten() {
    if let Ok(metadata) = entry.metadata() {
      if metadata.is_dir() {
        get_runincludes_inner(&entry.path(), runincludes)
      }
    }
  }
}

fn add_runincludes(path: &Path, runincludes: &mut Vec<PathBuf>) {
  let Ok(runinclude) = fs::read_to_string(path.join(".runinclude")) else {
    return Default::default();
  };
  runincludes.extend(
    runinclude
      .split('\n')
      .flat_map(|p| path.join(p).canonicalize()),
  );
}

fn get_runignores(path: &Path, runincludes: &[PathBuf]) -> Vec<PathBuf> {
  let mut runignores = Vec::<PathBuf>::new();
  get_runignores_inner(path, &mut runignores, runincludes);
  runignores
}

fn get_runignores_inner(path: &Path, runignores: &mut Vec<PathBuf>, runincludes: &[PathBuf]) {
  add_runignores(path, runignores);
  let Ok(entries) = fs::read_dir(path) else {
    return;
  };
  for entry in entries.flatten() {
    if let Ok(metadata) = entry.metadata() {
      if metadata.is_dir() {
        let path = entry.path();
        if (!runincludes.is_empty() && runincludes.contains(&path)) && !ignore_dir(&path) {
          get_runignores_inner(&path, runignores, runincludes);
        }
      }
    }
  }
}

fn add_runignores(path: &Path, runignores: &mut Vec<PathBuf>) {
  let Ok(runignore) = fs::read_to_string(path.join(".runignore")) else {
    return Default::default();
  };
  runignores.extend(
    runignore
      .split('\n')
      .flat_map(|p| path.join(p).canonicalize()),
  );
}
