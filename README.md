## math_calculator

使用 rust 实现的数学表达式解析计算引擎库，它小巧，无任何依赖，具有扩展性(比如可以注册自己的函数到引擎中)，比较完整的完成了数学表达式解析执行，包括词法分析、语法分析、构建AST、运行。



能够处理的表达式样例：
- `1+127-21+(3-4)*6/2.5`
- `(88+(1+8)*6)/2+99`
- `123_345_456 * 1.5 - 2 ^ 4`
- `-4 * 6 + 2e2 - 1.6e-3`
- `sin(pi/2)+cos(45-45*1)+tan(pi/4)`
- `99+abs(-1)-ceil(88.8)+floor(88.8)`
- `max(min(2^3, 3^2), 10*1.5-7)`
- `double(6) + 3` , `double`是一个自定义的函数

### Demo


## Method Support

| symbol      | explanation                  | e.g.                                  |
| ----------- | ---------------------------- | ------------------------------------- |
| `+`         | 加，plus                     | 1+2 = 3                               |
| `-`         | 减，sub                      | 8-3.5 = 4.5                           |
| `*`         | 乘，multiply                 | 2*3 = 6                               |
| `/`         | 除，division                 | 5/2 = 2.5                             |
| `%`         | 取余，remainder              | 5%2 = 1                               |
| `^`         | 整数次方，integer power      | 2^3 = 8, 3^2 = 9                      |
| `e`         | 科学计数法，E-notation       | 1.2e3 = 1.2e+3 = 1200，1.2e-2 = 0.012 |
| `()`        | 括号，brackets               | (2+3)*4 = 20                          |
| `_`         | 数字分隔符，number separator | 123_456_789 = 123456789               |
| `pi`        | π                            | pi = 3.141592653589793                |
| `sin(x)`    | 正弦函数，sine               | sin(pi/2) = 1                         |
| `cos(x)`    | 余弦函数，cosine             | cos(0) = 1                            |
| `tan(x)`    | 正切函数，tangent            | tan(pi/4) = 1                         |
| `cot(x)`    | 余切函数，cotangent          | cot(pi/4) = 1                         |
| `sec(x)`    | 正割函数，secant             | sec(0) = 1                            |
| `csc(x)`    | 余割函数，cosecant           | csc(pi/2) = 1                         |
| `abs(x)`    | 绝对值，absolute value       | abs(-6) = 6                           |
| `ceil(x)`   | 向上取整                     | ceil(4.2) = 5                         |
| `floor(x)`  | 向下取整                     | floor(4.8) = 4                        |
| `round(x)`  | 四舍五入取整                 | round(4.4) = 4, round(4.5) = 5        |
| `sqrt(x)`   | 平方根，square root          | sqrt(4) = abs(sqrt(4)) = 2            |
| `cbrt(x)`   | 立方根，cube root            | cbrt(27) = 3                          |
| `max(x, ...)` | 参数中的较大值              | max(1)=1,max(2,3)=3,max(4,8,6,8,10)=10 |
| `min(x, ...)` | 参数中的较小值              | min(1)=1,min(2,3)=2,max(4,8,6,8,10)=4 |
| `noerr(x)`  | 计算 x 出错时返回 0          | noerr(1 / 1)  = 1, noerr( 1/ 0 ) = 0  |
| `double(x)`  | 返回 x 的双倍值，这是一个自定义的函数示例，你可以注册任意的自定义函数到引擎中  | double(6) = 12  |


## Usage
### 直接clone本地运行
```git 
git clone git@github.com:ucanme/math_calculator.git
```
或
```git
git clone https://github.com/ucanme/math_calculator.git
```
在表达式src/main.rs中(可以直接修改表达式), 直接执行
```shell
cargo run
```


### 你可以直接引用该库嵌入到自己的程序中，添加依赖：
```toml
math_calculator = "0.1.0"
```
在代码中引入：
```
use math_calculator.{}
```
e.g. 1 常规用法： 直接调用解析执行函数 :

```rust
```



e.g. 2 高级用法： 依次调用函数，手动执行 :

```rust
```
编译运行，应该可以看到如下输出：
```bash
expression : 123+89-0.9 ,
ast tree: Ast { tokens: [Token { tok: "123", tok_type: LITERAL, flag: 0, offset: 0 }, Token { tok: "+", tok_type: OPERATOR, flag: 0, offset: 3 }, Token { tok: "89", tok_type: LITERAL, flag: 0, offset: 4 }, Token { tok: "-", tok_type: OPERATOR, flag: 0, offset: 6 }, Token { tok: "0.9", tok_type: LITERAL, flag: 0, offset: 7 }], curr_tok: Some(Token { tok: "123", tok_type: LITERAL, flag: 0, offset: 0 }), curr_idx: 0, depth: 0, priority_map: {"+": 1, "-": 1, "*": 40, "%": 40, "^": 50, "/": 40} }
expression : 123+89-0.9 ,
exec result: 211.1

```

## TrigonometricMode

三角函数的参数类型默认为弧度`RadianMode`，e.g. `sin(pi/2) = 1`.

你可以通过设置 `TrigonometricMode` 调整参数类型，可选 弧度`RadianMode`、角度`AngleMode`，e.g. :

```rust
```

## Register Function

`math-engine` 提供了自定义函数注册到引擎的能力。你可以把常用的函数注册到引擎中，然后就能像内置函数一样在输入的数学表达式中使用。

e.g

```rust
  // RegFunction is Top level function
  // the same function name only needs to be registered once.
  // double is register function name.
  // 1 is a number of parameter signatures. should be -1, 0, or a positive integer
  // func(expr ...engine.ExprAST) float64 is your function.
  // engine.RegFunction("double", 1, func(expr ...engine.ExprAST) float64 {
  //   // // when argc > 0，you can use the index value directly according to the number of parameters
  //   // // without worrying about crossing the boundary.
  //   // // use ExprASTResult to get the result of the ExprAST structure.
  //   // return engine.ExprASTResult(expr[0]) * 2
  // })
```

然后你就可以在输入的表达式中使用这个函数 `double`:

```rust
//exp := "double(6) + 2"
// r, err := engine.ParseAndExec("double(6) + 2")
// if err != nil {
//   panic(err)
// }
// fmt.Printf("double(6) + 2 = %f\n", r) // will print ： double(6) + 2 = 14.000000
```

注意事项：
- 注册的函数名只能是英文字母和数字，且必须英文字母开头（区分大小写）;
- 每一个函数名只能且只需注册一次；
- 注册的函数逻辑中如果有 panic ，需要程序自己捕获处理;
- argc=-1，即该函数的参数是可变的，expr 的长度需要开发者自行逻辑判断处理；


## Compile
```rust
```




## 实现细节

## TODO
### 已实现

- [x] 加 `+`
- [x] 减 `-`
- [x] 乘 `*`
- [x] 除 `/`
- [x] 取余 `%`
- [x] 整数次方 `^`
- [x] 科学计数法 e.g. `1.2e7`、  `1.2e-7`
- [x] 括号 `()`
- [x] 混合运算 e.g. `1+2*6/4+(456-8*9.2)-(2+4^5)*2e3+1.2e-2`
- [x] 友好的长数字 e.g. `123_456_789`
- [x] 三角函数 e.g. `sin, cos, tan, cot, sec, csc`
- [x] 常量 pi
- [x] 辅助函数 e.g. `abs, ceil, floor, sqrt, cbrt, max, min, noerr`
- [x] 提供自定义函数注册功能，注册后可以在表达式中使用
- [x] 精确的数据计算
- [x] 友好的错误消息 e.g.
```bash
input /> 123+89-0.0.9
called `Result::unwrap()` on an `Err` value: UnknowChar(".")
```

# 该项目参考go项目 https://github.com/dengsgo/math-engine