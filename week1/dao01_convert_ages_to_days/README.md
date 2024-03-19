# Convert ages to days

## Reference code:
https://github.com/LiveGray/100-Days-Of-Rust/tree/main/Week-01/Day-01_Convert-Ages-To-Days

## Main Changes:
- introduce module guess
- guess derive Default to provide a default construct.
- read_age method to accept user input and update age_in_days. 
- run method to print the converted age_in_days on the terminal

## Notes
- Use 365 days as the length of a year for this challenge.
- Ignore leap years and days between last birthday and now.
- Expect only positive integer inputs.

## Shortcomings
- 对于if let的使用理解还是不够
根源还是在于，对enum的match理解不够深刻。所以代码中拿掉了关于if let的实现; 后续可以定义年龄的两种表达方式
- 年
- 日
然后实现不同方式的年龄处理，以及相互转换

补充学习：
- if let能让我们通过一种不那么烦琐的语法结合使用if与let，并处理那些只用关心某一种匹配而忽略其他匹配的情况
