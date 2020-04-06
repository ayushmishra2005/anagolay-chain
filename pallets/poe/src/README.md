##  Custom type definition

Polkadot Js app

```json

{
  "ForWhat": {
    "_enum": [
      "Photo",
      "Camera",
      "Lens",
      "SmartPhone"
    ]
  },
  "RuleOperation": {
    "op": "Vec<u8>",
    "what": "Vec<u8>",
    "output": "bool"
  },
  "Rule": {
    "name": "Vec<u8>",
    "version": "u32",
    "for_what": "ForWhat",
    "ops": "Vec<RuleOperation>"
  }
}

```


other JS app that is not polkadotjs


```json
[
  {
    "ForWhat": {
      "_enum": [
        "Photo",
        "Camera",
        "Lens",
        "SmartPhone"
      ]
    }
  },
  {
    "RuleOperation": {
      "op": "Vec<u8>",
      "what": "Vec<u8>",
      "output": "bool"
    }
  },
  {
    "Rule": {
      "name": "Vec<u8>",
      "version": "u32",
      "for_what": "ForWhat",
      "ops": "Vec<RuleOperation>"
    }
  }
]
```
