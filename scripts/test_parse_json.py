#flattening nested json keys by joining with a .
def parse_json(json,parents,n) -> dict:
  if not (isinstance(json, list) or isinstance(json, dict)):
     return { ".": json }
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
            if parents == "":
               flattened_dict[k] = v
            else:
              flattened_dict[parents+"."+k] = v
  return flattened_dict

res = parse_json(20,"",0)
print(res)
  