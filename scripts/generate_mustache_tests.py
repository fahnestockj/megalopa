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
  test += "\n\tlet template = " + coerce_str(json["template"]) + r";"
  test += "\n\tlet engine = create_oneoff_engine(template);"
  test += "\n\tlet mut ctx = std::collections::HashMap::new();"
  # flatten json keys into one dict then insert into hashmap
  flat_dict = {}

  if isinstance(json["data"],dict):
    flat_dict = parse_json(json["data"],"",0)
  for key in flat_dict:
    test += "\n\tctx.insert(" + coerce_str(key) + "," + coerce_str(flat_dict[key]) + ");"
  test += "\n\tlet result = engine.oneoff_render(ctx);"
  test += "\n\tlet expected = String::from(" + coerce_str(json["expected"]) + ");"
  test += "\n\tassert_eq!(result, expected)"
  test += "\n}"
  return test

# the goal is to get a double quotes wrapped string with escaped chars
def coerce_str(maybe_wrapped_string) -> str:
   return json.dumps(maybe_wrapped_string)
  

def parse_json(json,parents,n) -> dict:
  flattened_dict = {}
  if isinstance(json, list):
     for idx, val in enumerate(json):
        if isinstance(val, dict) or isinstance(val,list):
          list_dict = parse_json(val, parents+"."+str(idx), n+1)
          flattened_dict.update(list_dict)
          # print(flattened_dict)
        else:
            flattened_dict[parents+"."+str(idx)] = val

  else:
    for k, v in json.items():
        if isinstance(v, dict) or isinstance(v,list):
            parse_json(v,parents+"."+k, n+1)
        else:
            flattened_dict[parents+"."+k] = v
  return flattened_dict


spec_dir = Path("../mustache_spec/spec/specs")
for file in spec_dir.glob("*.json"):
  if file.name == "~lambdas.json":
     continue
  read_stream = file.open()
  jsond = json.load(read_stream)
  test_dir = Path("../src/html_templating/spec_tests")
  test_file = Path("../src/html_templating/spec_tests/" + file.name[:-5].replace("~", "").replace("-", "_") + ".rs")
  write_stream = test_file.open("w")
  file_txt = "#[cfg(test)]\nmod tests {\n\tuse crate::html_templating::{create_oneoff_engine, oneoff_render};\n\t"
  for test_json in jsond["tests"]:
    test_str = make_rust_test(test_json)
    file_txt += test_str

  file_txt += "\n}"
  write_stream.write(file_txt)
    

  






