
### 注意：(RUST这个功能太棒了！)

你不可能凭空就知道应该 use 哪个 trait 以及该从 crate 中调用哪个方法，所以每个 crate 都有使用说明文档。Cargo 有一个很棒的功能是：运行 ==cargo doc --open== 命令来构建所有本地依赖提供的文档，并在浏览器中打开。例如，假设你对 rand crate 中的其他功能感兴趣，你可以运行 cargo doc --open 并点击左侧导航栏中的 rand。
