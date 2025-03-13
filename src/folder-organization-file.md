# `.folder-organization` file.

## Example

```json
{
  "folders": {
    ".": {
      "folder-name": ".",
      "title": ". folder",
      "description": "description",
      "children": ["./assets", "./docs", "./src", "./tests"]
    },
    "./src": {
      "folder-name": "src",
      "title": "src folder",
      "description": "description",
      "children": ["./src/src1"]
    }
  },
  "options" : {
    "python-project": false
  }
}

```

## Options

### python-project

Will be used to lint folder names.

By default, kebab-case is the norm for folder names.

Todo: change the option name to just "kebab-case"? More general
