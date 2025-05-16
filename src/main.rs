use eframe::egui;
use std::time::Duration;

// 排序状态跟踪
struct BubbleSortVisualizer {
    data: Vec<u32>,           // 待排序数据
    i: usize,                // 外层循环索引
    j: usize,                // 内层循环索引
    comparing: bool,         // 是否正在比较
    swapped: bool,           // 本轮是否发生交换
    running: bool,           // 是否正在运行
    done: bool,              // 是否已完成排序
}

impl Default for BubbleSortVisualizer {
    fn default() -> Self {
        Self {
            data: vec![35, 12, 68, 42, 7, 23, 94, 51],
            i: 0,
            j: 0,
            comparing: false,
            swapped: false,
            running: false,
            done: false,
        }
    }
}

impl eframe::App for BubbleSortVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("bubblesort");
            
            // 控制按钮
            ui.horizontal(|ui| {
                if ui.button("start").clicked() && !self.done {
                    self.running = true;
                }
                if ui.button("stop").clicked() {
                    self.running = false;
                }
                if ui.button( "next").clicked() && !self.running && !self.done {
                    self.step();
                }
                if ui.button("reset").clicked() {
                    *self = Self::default();
                }
            });

            // 状态显示
            ui.separator();
            ui.label(format!("now: {}", 
                if self.done { "finished" } 
                else if self.running { "running..." } 
                else { "stopped" }
            ));
            ui.label(format!("compared time: {}", self.i * (self.data.len() - 1) + self.j));
            ui.label(format!("switched time: {}", self.i));
        });

        // 主可视化区域
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let spacing = 40.0;
            let start_x = 50.0;
            let base_y = 300.0;
            let max_height = 200.0;

            // 计算每个元素的绘制参数
            let max_value = *self.data.iter().max().unwrap_or(&1) as f32;
            let column_width = 30.0;

            for (idx, &value) in self.data.iter().enumerate() {
                let height = (value as f32 / max_value) * max_height;
                let x = start_x + idx as f32 * spacing;
                
                // 确定颜色
                let color = if self.done {
                    egui::Color32::LIGHT_GREEN // 已完成
                } else if idx >= self.data.len() - self.i {
                    egui::Color32::GREEN // 已排序部分
                } else if self.comparing && (idx == self.j || idx == self.j + 1) {
                    egui::Color32::RED // 正在比较的元素
                } else {
                    egui::Color32::GRAY // 未处理部分
                };

                // 绘制柱状图
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(x, base_y - height),
                        egui::vec2(column_width, height),
                    ),
                    5.0,
                    color,
                );

                // 显示数值
                painter.text(
                    egui::pos2(x + column_width/2.0, base_y + 20.0),
                    egui::Align2::CENTER_CENTER,
                    value.to_string(),
                    egui::FontId::monospace(14.0),
                    egui::Color32::BLACK,
                );
            }
        });

        // 自动步进逻辑
        if self.running && !self.done {
            ctx.request_repaint_after(Duration::from_millis(500));
            self.step();
        }
    }
}

impl BubbleSortVisualizer {
    fn step(&mut self) {
        if self.i >= self.data.len() - 1 {
            self.done = true;
            return;
        }

        self.comparing = true;
        
        if self.data[self.j] > self.data[self.j + 1] {
            self.data.swap(self.j, self.j + 1);
            self.swapped = true;
        }

        // 移动到下一对元素
        self.j += 1;

        // 完成一轮内循环
        if self.j >= self.data.len() - 1 - self.i {
            self.i += 1;
            self.j = 0;
            if !self.swapped {
                self.done = true;
            }
            self.swapped = false;
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ =eframe::run_native(
        "bubblesort",
        options,
        Box::new(|_| Box::new(BubbleSortVisualizer::default())),
    );
}