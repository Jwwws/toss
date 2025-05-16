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

## 使用egui实现prim算法

### 定义节点

```
struct Node {
    id: usize,
    position: Pos2,
    visited: bool,
}
```

定义节点的结构体保存其id，位置，以及是否被访问

### 定义边

```
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
    selected: bool,
}
```

定义边的结构体，其中有来的节点id，去的节点id，以及其权重以及是否被挑选中

### 边的比较

```
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}
```

比较边的权重，通过反转比较顺序与binaryheap结合来定义一个最小堆

### 定义prim算法的可视化

```
struct PrimVisualizer {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    priority_queue: BinaryHeap<Edge>,
    visited: HashSet<usize>,
    current_edge: Option<Edge>,
    running: bool,
    step_delay: f32,
    accumulated_time: f32,
}
```

定义节点，边，优先队列，已经访问的节点集合，当前正在处理的边的集合来保存算法运行到哪一步，同时定义算法运行状态的记录，以及运行时间来实现自动进行prim算法的可视化。

### 初始化节点

```
impl Default for PrimVisualizer {
    fn default() -> Self {
        let mut nodes = Vec::new();
        
        for i in 0..5 {
            let angle = (i as f32) * std::f32::consts::TAU / 5.0;
            nodes.push(Node {
                id: i,
                position: Pos2::new(
                    300.0 + 200.0 * angle.cos(),
                    300.0 + 200.0 * angle.sin(),
                ),
                visited: false,
            });
        }

```

初始化5个节点，其中每个节点都在以（300，300）为圆心200为半径的圆上，并且将原5等分来计算不同点的坐标位置。并且将每个节点设置为unvistied

### 初始化边

```
 let edges = vec![
            Edge { from: 0, to: 1, weight: 2, selected: false },
            Edge { from: 0, to: 3, weight: 6, selected: false },
            Edge { from: 1, to: 2, weight: 3, selected: false },
            Edge { from: 1, to: 3, weight: 8, selected: false },
            Edge { from: 1, to: 4, weight: 5, selected: false },
            Edge { from: 2, to: 4, weight: 7, selected: false },
            Edge { from: 3, to: 4, weight: 9, selected: false },
        ];

```

定义9条边，每一条都保存有其起始位置与终点。将每条边设置为未被选中状态

### 将边加入队列

```
add_edges_to_queue(&mut self, node_id: usize) {
        for edge in &self.edges {
            if edge.from == node_id && !self.visited.contains(&edge.to) {
                self.priority_queue.push(edge.clone());
            }
            if edge.to == node_id && !self.visited.contains(&edge.from) {
                self.priority_queue.push(edge.clone());
            }
        }
```

