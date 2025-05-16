# lab02
## egui
使用 egui要先定义一个结构体BasicApp。如： 

```
pub struct BasicApp{
g:egui_graphs::Graph,
}
```

然后实现结构体的函数

```
impl BasicApp{
fn new(_:&eframe::CreationContext<'_>')-
	let g=generate_graph();
	self{g:egui_graphs::Graph::from(&g)}
}
```

```
self{g:egui_graphs::Graph::from(&g)
```

将自定义图转换为egui兼容模式，