<div align="center">
    <h1>TemplateStr-Go</h1>
    <h2>TemplateStr allows to add variable, function, condition and switch in a string.</h2>
    <img src="https://img.shields.io/static/v1?label=Rust&message=1.58%5E&color=964B00&style=flat-square&logo=Rust&logoColor=964B00"/>
    <a href="https://github.com/CheeseGrinder/TemplateStr-Rust/actions/workflows/rust.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/CheeseGrinder/TemplateStr-Rust/rust.yml?label=Test&style=flat-square"/>
    </a>
</div>

### Install :
```
[dependencies]
template_str = { git="https://github.com/CheeseGrinder/TemplateStr-Rust", tag="vX.X.X" }
```

### Import :

```rust
use template_str::{TemplateStr, t_type::{VariableMap as VMap, TVal, FuncMap}, varmap, list_func};
```

### Construtor :

```rust
let template = TemplateStr::new(map, Some(vec));
```

<ul>
<li>
<details>
<summary><code>funcArray</code>: is a array of Functions you want to pass to be called in your text</summary><br>

```rust
let vec: FuncMap = list_func![test, test_type];
```

</details>
</li>

<li>
<details>
<summary><code>varMap</code>: is a map of the Variables you want to pass to be called in your text</summary><br>

```rust
let map: VMap = varmap!{
        "Build" => "Succes",
        "var" => "int",
        "str" => "Jame",
        "int" => 32,
        "float" => 4.2,
        "bool" => true,
        "lower" => "azerty",
        "upper" => "AZERTY",
        "swap" => "AzErTy",
        "Map" => varmap!{
            "value" => "Map in Map",
        },
        "MasterMap" => varmap!{
            "SecondMap" => varmap!{
                "value" => "Map in Map in Map",
            },
        },
    };
```

</details>
</li>
</ul>

### Function :

```go
parser.Parse(text)
```

- `Parse(text: string) string` : parse all (variable, function, condition and switch)
- `ParseVariable(text: string) string` : parse Variable ; ${{variableName}}
- `ParseFunction(text: string) string` : parse Function and Custom Function ; @{{functionName}}
- `ParseCondition(text: string) string` : parse Condition ; #{{value1 == value2; trueValue | falseValue}}
- `ParseSwitch(text: string) string` : parse Switch ; ?{{var; value1:#0F0, value2:#00F, ..., _:#000}}
- `HasOne(text: string) bool` : check if there are one syntaxe
- `HasVariable(text: string) bool` : check if there are any Variable
- `HasFunction(text: string) bool` : check if there are any Function
- `HasCondition(text: string) bool` : check if there are any Condition
- `HasSwitch(text: string) bool` : check if there are any Switch

### Exemple Syntaxe :

<ul>
<li>
<details>
<summary><strong>Variable</strong></summary>
</br>

The syntax of the Variables is like if :
- `${{variable}}`
- `${{Map.value}}`
- `${{MasterMap.SecondMap.value. ...}}`

if the value does not exist then `None` is return

<!-- V Be careful, it's not a "go" code, it's just to have some colour in the rendering -->
```go
name = "Jame"

"name is ${{name}}" => parse => "name is Jame"
```

</details>
</li>

<li>
<details>
<summary><strong>Function</strong></summary>
</br>

The syntax of the Function is like if : `@{{function; parameter}}` or `@{{function}}`

internal function list :

- `@{{uppercase; variableName}}`
- `@{{uppercaseFirst; variableName}}`
- `@{{lowercase; variableName}}`
- `@{{swapcase; variableName}}`
- `@{{time}}`
- `@{{date}}`
- `@{{dateTime}}`

<!-- V Be careful, it's not a "go" code, it's just to have some colour in the rendering -->
```go
name = "jame"

"name is @{{uppercase; name}}" => parse => "name is JAME"
```

</details>
</li>

<li>
<details>
<summary><strong>Custom Function</strong></summary>
</br>

The syntax of the Custom Function is like if : `@{{customFunction; param1 param2 ...}}` or `@{{customFunction}}`

`Syntaxe Typing` can be used at the parameter level of custom functions

For developers :
- Parameters to be passed in a `list/vec/array`
- The custom function must necessarily return a `str/string`

</details>
</li>

<li>
<details>
<summary><strong>Condition</strong></summary>
</br>

The syntax of the Condition is like if :
- `#{{value1 == value2; trueValue | falseValue}}`

comparator:
- `==`
- `!=`
- `<=` *
- `<` *
- `>=` *
- `>` *

<details>
<summary>* for this comparator the type <code>string</code> and <code>bool</code> are modified :</summary>

- `string` it's the number of characters that is compared ('text' = 4)
- `bool` it's the value in int that is compared (True = 1)

</details></br>

`value1` is compared with `value2`

`Syntaxe Typing` can be used at `value1` and `value2` level

<!-- V Be careful, it's not a "go" code, it's just to have some colour in the rendering -->
```go
name = "Jame"

"Jame is equal to James ? #{{name == 'James'; Yes | No}}" => parse => "Jame is equal to James ? No"
```

</details>
</li>

<li>
<details>
<summary><strong>Switch</strong></summary>
</br>

The syntax of the Switch is like if : 
- `?{{variableName; value1:#0F0, value2:#00F, ..., _:#000}}`
- `?{{type/variableName; value1:#0F0, value2:#00F, ..., _:#000}}`

The value of `variableName` is compared with all the `values*`,
if a `values*` is equal to the value of `variableName` then the value after the ":" will be returned

you can specify the type of `variableName`, but don't use `Syntaxe Typing`.
If the type is specified then all `values*` will be typed with the same type.

syntaxe for specify type `variableName` :
- `str`
- `int`
- `float`

<!-- V Be careful, it's not a "go" code, it's just to have some colour in the rendering -->
```go
name = "Jame"
yearsOld = 36

"how old is Jame ? ?{{name; Jame:42 years old, William:36 years old, _:I don't know}}" => parse => "how old is Jame ? 42 years old"
"who at 36 years old ? ?{{int/yearsOld; 42:Jame !, 36:William !, _:I don't know}}" => parse => "who at 42 years old ? William !"
```

</details>
</li>
</ul>

### Syntaxe Typing :

| Format                       | Type    | Description                                                             | Return                 |
|------------------------------|---------|-------------------------------------------------------------------------|------------------------|
| variableName                 | `*`     | Is the key of the value in the dictionary pass to the constructor       | value of `variableName`|
| b/True                       | `bool`  | Type the string True as `bool`                                          | True                   |
| i/123                        | `int`   | Type the string 123 as type `int`                                       | 123                    |
| f/123.4                      | `float` | Type the string 123.4 as type `float`                                   | 123.4                  |
| "text" or 'text' or \`text\` | `str`   | It just takes what's in quote, not to be interpreted as a variable name | text                   |

### TODO

- [ ] : Add exemple
- [x] : Add test

 