# commit-formatter

A Rust version [commitizen](https://github.com/commitizen-tools/commitizen).

## Download

```shell
cargo install commit-formatter
```

If you get a error: 'command not found: commit-formatter', maybe you need to add cargo installation directory to PATH.

## Usage

Recommend to use alias like `cf`.

```shell
git add <files>
cf
```

## Custom

Put a `commit-format.json` in your project directory.

### Example

```json
[
	{
		"text": "特性",
		"description": "新功能"
	},
	{
		"text": "",
		"description": "修復bug"
	}
]
```
