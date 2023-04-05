fn main() {
    //变量屏蔽
    let v = "hello world";
    assert_eq!(v, "hello world");
    let v = "hello rust";//第二次赋值会覆盖第一次的
    assert_eq!(v, "hello rust");
    let v = "hello three";//此次赋值会覆盖前一次
    //连续使用let赋值的方式叫做“变量屏蔽”
    assert_eq!(v, "hello three");
    {
        let v = "hello world";//这次赋值在"}"后失效，
        assert_eq!(v, "hello world");
    }
    assert_eq!(v, "hello three");//所以此次v的值还是第三次赋值
}