# lab02
## egui示例
### 结构体定义

```
pub struct BasicApp{
g:egui_graphs::Graph,
}
```

### 结构体函数的定义

```
impl BasicApp{
fn new(_:&eframe::CreationContext<'_>')-
	let g=generate_graph();
	self{g:egui_graphs::Graph::from(&g)}
}
```

```
let g = generate_graph();
```

生成一个包含3个节点和3条边的3角形图

```
self{g:egui_graphs::Graph::from(&g)
```

将自定义图转换为egui兼容模式

### 图生成函数

```
fn generate_graph() -> StableGraph<(), ()> {
    let mut g = StableGraph::new();
    let a = g.add_node(()); 
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, ());
    g.add_edge(b, c, ());
    g.add_edge(c, a, ());
    g
}
```

使用new函数不断生成新的边和节点

### 图的绘制

```
fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut DefaultGraphView::new(&mut self.g));
        });
    }
```

defaultGraphView:egui_graphs提供的默认图视图组件自动处理边的节点绘制、布局和交互

centralpanel：egui的中心面板组件



### 主函数

```
fn main() {
    run_native(
        "egui_graphs_basic_demo", 
        NativeOptions::default(),  
        Box::new(|cc| Ok(Box::new(BasicApp::new(cc)))), 
    )
    .unwrap();
}
```

设置默认的窗口配置

### 生成窗口

```
fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        Window::new("graph").show(ctx, |ui| {
            ui.add(&mut DefaultGraphView::new(&mut self.g));
        });
    }
}
```

使用window来生成一个窗口，其中可以自定义命名