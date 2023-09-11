use eframe::{
    egui::{self, Frame, Margin, Ui},
    epaint::{pos2, Color32, Pos2},
};

struct AppUI {
    // 一个 15 * 15 的棋盘，黑子用 1 表示，白子用 2 表示，空位用 0 表示
    board_data: [[u8; 15]; 15],

    // 棋盘起始点，棋盘左上角距离画布左上角的距离
    start_point: Pos2,

    // 是否该黑子落子了
    is_black: bool,

    // 是否已经产生了赢家
    is_winner: bool,

    frame: egui::Frame,
}

impl Default for AppUI {
    fn default() -> Self {
        Self {
            frame: Frame {
                inner_margin: Margin::same(0.0),
                outer_margin: Margin::same(0.0),
                fill: egui::Color32::LIGHT_YELLOW,
                ..Default::default()
            },
            board_data: [[0; 15]; 15],
            // 棋盘左上角距离画布左上角的距离
            start_point: pos2(15.0, 15.0),
            is_black: true,
            is_winner: false,
        }
    }
}

impl AppUI {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// 绘制棋盘
    fn render_board(&self, ui: &Ui) {
        let stroke = egui::Stroke::new(1.0, egui::Color32::DARK_GRAY);

        // 先画横线
        for i in 0..15 {
            let start = self.start_point + egui::Vec2::new(0.0, i as f32 * 30.0);
            let end = start + egui::Vec2::new(420.0, 0.0);
            ui.painter().line_segment([start, end], stroke);
        }
        // 再画竖线
        for i in 0..15 {
            let start = self.start_point + egui::Vec2::new(i as f32 * 30.0, 0.0);
            let end = start + egui::Vec2::new(0.0, 420.0);
            ui.painter().line_segment([start, end], stroke);
        }
    }

    /// 画圆
    fn render_circle(&self, ui: &Ui, center: egui::Pos2, color: Color32, stroke_color: Color32) {
        let stroke = egui::Stroke::new(1.0, stroke_color);
        ui.painter().circle(center, 14.0, color, stroke)
    }

    /// 画白子
    fn render_white(&self, ui: &Ui, center: egui::Pos2) {
        self.render_circle(ui, center, Color32::WHITE, Color32::GRAY)
    }

    /// 画黑子
    fn render_black(&self, ui: &Ui, center: egui::Pos2) {
        self.render_circle(ui, center, Color32::BLACK, Color32::BLACK)
    }

    /// 绘制棋子
    fn render_piece(&self, ui: &Ui) {
        // 遍历棋子数组数据
        for (i, x) in self.board_data.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                match y {
                    1 => self.render_black(ui, self.get_position(i, j)),
                    2 => self.render_white(ui, self.get_position(i, j)),
                    _ => {}
                }
            }
        }
    }

    fn get_position(&self, x: usize, y: usize) -> Pos2 {
        // start + ( 30 * x, 30 * y )
        let x = x as f32;
        let y = y as f32;
        self.start_point + egui::Vec2::new(30.0 * x, 30.0 * y)
    }

    /// 处理鼠标点击事件
    fn handle_click(&mut self, pos: Pos2) {
        // 首先 xy 都减去 15，然后除以 30，然后四舍五入
        let x = ((pos.x - 15.0) / 30.0).round() as usize;
        let y = ((pos.y - 15.0) / 30.0).round() as usize;
        // 如果点击了棋盘以外的空间，或者该点位已有棋子，什么事都不做
        if x > 14 || y > 14 || self.board_data[x][y] != 0 {
            return;
        }
        self.board_data[x][y] = if self.is_black { 1 } else { 2 };
        if self.check_winner(x, y) {
            self.is_winner = true;
            return;
        };
        self.is_black = !self.is_black;
    }

    /// 检查是否有获胜者
    fn check_winner(&self, x: usize, y: usize) -> bool {
        // 从最后一次的落点开始检查
        let current = self.board_data[x][y];
        let mut count = 1;

        // 先往左数，再往右数，累加，检查是否大于等于 5
        for i in 1..5 {
            if x < i || self.board_data[x - i][y] != current {
                break;
            }
            count += 1;
        }
        for i in 1..5 {
            if x + i > 14 || self.board_data[x + i][y] != current {
                break;
            }
            count += 1;
        }
        if count >= 5 {
            return true;
        } else {
            count = 1;
        }

        // 先往上数，再往下数，累加，检查是否大于等于 5
        for i in 1..5 {
            if y < i || self.board_data[x][y - i] != current {
                break;
            }
            count += 1;
        }
        for i in 1..5 {
            if y + i > 14 || self.board_data[x][y + i] != current {
                break;
            }
            count += 1;
        }
        if count >= 5 {
            return true;
        } else {
            count = 1;
        }

        // 先往左上数，再往右下数，累加，检查是否大于等于 5
        for i in 1..5 {
            if x < i || y < i || self.board_data[x - i][y - i] != current {
                break;
            }
            count += 1;
        }
        for i in 1..5 {
            if x + i > 14 || y + i > 14 || self.board_data[x + i][y + i] != current {
                break;
            }
            count += 1;
        }
        if count >= 5 {
            return true;
        } else {
            count = 1;
        }

        // 先往左下数，再往右上数，累加，检查是否大于等于 5
        // 往左下是 x- y+
        for i in 1..5 {
            if x < i || y + i > 14 || self.board_data[x - i][y + i] != current {
                break;
            }
            count += 1;
        }
        // 往右上是 x+ y-
        for i in 1..5 {
            if x + i > 14 || y < i || self.board_data[x + i][y - i] != current {
                break;
            }
            count += 1;
        }
        if count >= 5 {
            return true;
        }

        false
    }

    fn restart(&mut self) {
        self.board_data = [[0; 15]; 15];
        self.is_black = true;
        self.is_winner = false;
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(self.frame)
            .show(ctx, |ui| {
                self.render_board(ui);
                self.render_piece(ui);

                if self.is_winner {
                    let text = if self.is_black {
                        "Black Won!"
                    } else {
                        "White Won!"
                    };
                    egui::Window::new(text)
                        .collapsible(false)
                        .resizable(false)
                        .show(ctx, |ui| {
                            ui.vertical_centered(|ui| {
                                if ui.button("Restart").clicked() {
                                    self.restart();
                                }
                            });
                        });
                    return;
                }

                // 监听点击事件
                if let Some(pos) = ctx.input(|i| i.pointer.press_origin()) {
                    self.handle_click(pos);
                }
            });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(450.0, 450.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native("Gomoku", options, Box::new(|cc| Box::new(AppUI::new(cc)))).unwrap();
}
