from pathlib import Path
import json

def make_rust_test(json) -> str:
  test = "\n"
  if "desc" in json:
    for line in str.splitlines(json["desc"]):
      test += "\n/// " + line
  test += "\n#[test]\npub fn "
  test += '_'.join(str.split(str.lower(json["name"]).replace('-', "").replace('(',"").replace(')', "").replace(',',""), ' '))
  test += " () {"
  test += "\n\tlet template = " + json_dump(json["template"]) + ".to_string();"
  test += "\n\tlet engine = TemplateEngine{};"
  test += "\n\tlet mut ctx: std::collections::HashMap<&str, CtxValue> = std::collections::HashMap::new();"
  # flatten json keys into one dict then insert into hashmap
  flat_dict = flatten_json(json["data"],"",0)
  for key in flat_dict:
    test += "\n\tctx.insert(" + json_dump(key) + "," + wrap_ctx_value(json_dump(flat_dict[key])) + ");"
  test += "\n\tlet result = engine.oneoff_render(template, ctx);"
  test += "\n\tlet expected = String::from(" + json_dump(json["expected"]) + ");"
  test += "\n\tassert_eq!(result, expected)"
  test += "\n}"
  return test

# may use to alter the json dump later otherwise delete
def json_dump(maybe_wrapped_string):
   return json.dumps(maybe_wrapped_string)

# Rust CtxValue enum for the hashmap
def wrap_ctx_value(val: str):
   
   if val.startswith("true") or val.startswith("false"):
      return f"CtxValue::Boolean({str(val)})"
   if val.isnumeric():
      return f"CtxValue::Number({str(val)})"
   if val.startswith("null"):
      return "CtxValue::Boolean(false)"
  
   else:
      return f"CtxValue::String({val}.to_string())"


# flattening nested json keys by joining with a .
def flatten_json(json,parents,n) -> dict:
  if not (isinstance(json, list) or isinstance(json, dict)):
    return { ".": json }
  flattened_dict = {}
  if isinstance(json, list):
     for idx, val in enumerate(json):
        if isinstance(val, dict) or isinstance(val,list):
          list_dict = flatten_json(val, parents+"."+str(idx), n+1)
          flattened_dict.update(list_dict)
          # print(flattened_dict)
        else:
            flattened_dict[parents+"."+str(idx)] = val

  else:
    for k, v in json.items():
        if isinstance(v, dict) or isinstance(v,list):
            flatten_json(v,parents+"."+k, n+1)
        else:
            if parents == "":
               flattened_dict[k] = v
            else:
              flattened_dict[parents+"."+k] = v
  return flattened_dict


spec_dir = Path("../mustache_spec/spec/specs")
for file in spec_dir.glob("*.json"):
  # skip optional tests
  if file.name[0] == "~":
     continue
  read_stream = file.open()
  jsond = json.load(read_stream)
  test_dir = Path("../src/html_templating/spec_tests")
  test_file = Path("../src/html_templating/spec_tests/" + file.name[:-5].replace("~", "_").replace("-", "_") + ".rs")
  write_stream = test_file.open("w")
  file_txt = "#[cfg(test)]\nmod tests {\n\tuse crate::html_templating::{TemplateEngine, OneoffRender, CtxValue};\n\t"
  for line in jsond["overview"].splitlines():
     file_txt += "\n// " + line
  for test_json in jsond["tests"]:
    test_str = make_rust_test(test_json)
    file_txt += test_str

  file_txt += "\n}"
  write_stream.write(file_txt)
    

  






