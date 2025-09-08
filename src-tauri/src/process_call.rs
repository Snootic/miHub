use std::ffi::CString;
use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use serde_json::{json, Value};

pub fn call_python(module: &str, kwargs: Vec<(&str, &str)>) -> PyResult<Value> {
  let file_name = format!("{}.py", module);
  
  Python::with_gil(|py| {
    let file_name_cstr = CString::new(file_name).unwrap();
    let module_cstr = CString::new(module).unwrap();
    
    let args_str = kwargs
    .iter()
    .map(|(key, value)| format!("{}='{}'", key, value)) // Formata cada argumento
    .collect::<Vec<String>>()
    .join(", ");
    
    let module_name = format!("src.hoyolab.auto_start.{}", module);
    
    let code = format!(
      r#"import argparse
import sys
import os
project_root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
      
sys.path.insert(0, project_root)
import {} as task
      
args = argparse.Namespace({})
      
def main():
     task.main(args)"#,
      module_name, args_str
    );
    
    let code_cstring = CString::new(code).unwrap();
    
    let module = PyModule::from_code(py, &code_cstring, &file_name_cstr, &module_cstr)?;
    
    let raw_result = module.call_method0("main")?;
    
    convert_to_json(raw_result)
  })
}

fn convert_to_json(py_obj: pyo3::Bound<'_, pyo3::PyAny>) -> PyResult<Value> {
  if let Ok(py_list) = py_obj.downcast::<PyList>() {
    let json_array: PyResult<Vec<Value>> =
    py_list.iter().map(|item| convert_to_json(item)).collect();
    json_array.map(Value::Array)
  } else if let Ok(py_dict) = py_obj.downcast::<PyDict>() {
    let mut json_map = serde_json::Map::new();
    for (key, value) in py_dict {
      let key_str = key.extract::<String>()?;
      let json_value = convert_to_json(value)?;
      json_map.insert(key_str, json_value);
    }
    Ok(Value::Object(json_map))
  } else if let Ok(py_str) = py_obj.extract::<String>() {
    Ok(Value::String(py_str))
  } else {
    Ok(Value::String(format!("{:?}", py_obj)))
  }
}

pub fn handle_python_call(module: &str, kwargs: Vec<(&str, &str)>) -> Result<String, String> {
  match call_python(module, kwargs) {
    Ok(output) => {
      let result = json!({
        "success": true,
        "output": output
      });
      Ok(result.to_string())
    }
    Err(err) => {
      let result = json!({
        "success": false,
        "error": err.to_string()
      });
      Err(result.to_string())
    }
  }
}

pub fn set_sys_path(binding: PathBuf) -> PyResult<()> {
  let libs = binding.to_str().unwrap();
  let win_site_packages_path = binding.join("Python311/site-packages");
  let win_site_packages = win_site_packages_path.to_str().unwrap();
  
  let site_packages_unix_path = binding.join("lib/python3.11/site-packages");
  let site_packages_unix = site_packages_unix_path.to_str().unwrap();
  
  let args = cfg!(target_os = "windows")
  .then(|| vec![libs, win_site_packages])
  .unwrap_or(vec![libs, site_packages_unix]);
  
  pyo3::prepare_freethreaded_python();
  
  Python::with_gil(|py| {
    let sys = py.import("sys")?;
    let path = sys.getattr("path")?;
    for arg in args.iter() {
      path.call_method1("append", (arg,))?;
    }
    
    Ok(())
  })
}
